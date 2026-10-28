use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::optimization::{self, BuildOptions, OptimizationError};
use inkwell::module::Module;
use inkwell::targets::FileType;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum NativeError {
    Target(String),
    Optimization(OptimizationError),
    Verification(String),
    ObjectEmission(String),
    Filesystem { path: PathBuf, source: io::Error },
    LinkerUnavailable(io::Error),
    LinkerFailed { status: Option<i32>, stderr: String },
}

impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Optimization(error) => error.fmt(f),
            Self::Target(message) => write!(f, "LLVM host target initialization failed: {message}"),
            Self::Verification(message) => write!(f, "LLVM module verification failed: {message}"),
            Self::ObjectEmission(message) => write!(f, "LLVM object emission failed: {message}"),
            Self::Filesystem { path, source } => {
                write!(f, "Cannot access {}: {source}", path.display())
            }
            Self::LinkerUnavailable(source) => write!(
                f,
                "Cannot start the system Clang linker driver (install Xcode command line tools): {source}"
            ),
            Self::LinkerFailed { status, stderr } => write!(
                f,
                "Clang linker failed (exit status {status:?}): {}",
                stderr.trim()
            ),
        }
    }
}

impl std::error::Error for NativeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Filesystem { source, .. } | Self::LinkerUnavailable(source) => Some(source),
            Self::Optimization(error) => Some(error),
            _ => None,
        }
    }
}

fn filesystem_error(path: &Path, source: io::Error) -> NativeError {
    NativeError::Filesystem {
        path: path.to_owned(),
        source,
    }
}

#[derive(Debug, Default, Clone)]
pub struct NativeTimings {
    pub target_setup: Duration,
    pub optimization: Duration,
    pub object: Duration,
    pub link: Duration,
}

/// Compatibility object API; new programs use the default O2 policy.
pub fn emit_object(module: &Module<'_>, output: &Path) -> Result<(), NativeError> {
    emit_object_with_options(module, output, &BuildOptions::default()).map(|_| ())
}

pub fn emit_object_with_options(
    module: &Module<'_>,
    output: &Path,
    options: &BuildOptions,
) -> Result<NativeTimings, NativeError> {
    module
        .verify()
        .map_err(|error| NativeError::Verification(error.to_string()))?;
    let start = Instant::now();
    let machine =
        optimization::target_machine(module, options).map_err(NativeError::Optimization)?;
    let target_setup = start.elapsed();
    let start = Instant::now();
    optimization::optimize(module, &machine, options).map_err(NativeError::Optimization)?;
    let optimization = start.elapsed();
    if let Some(path) = &options.emit_ir {
        fs::write(path, module.print_to_string().to_bytes())
            .map_err(|error| filesystem_error(path, error))?;
    }
    if let Some(path) = &options.emit_assembly {
        let assembly = machine
            .write_to_memory_buffer(module, FileType::Assembly)
            .map_err(|error| NativeError::ObjectEmission(error.to_string()))?;
        fs::write(path, assembly.as_slice()).map_err(|error| filesystem_error(path, error))?;
    }
    let start = Instant::now();
    module
        .verify()
        .map_err(|error| NativeError::Verification(error.to_string()))?;
    let object = machine
        .write_to_memory_buffer(module, FileType::Object)
        .map_err(|error| NativeError::ObjectEmission(error.to_string()))?;
    fs::write(output, object.as_slice()).map_err(|error| filesystem_error(output, error))?;
    Ok(NativeTimings {
        target_setup,
        optimization,
        object: start.elapsed(),
        link: Duration::ZERO,
    })
}

/// Uses Clang only as the native linker driver; paths are passed as separate arguments.
pub fn link_object(object: &Path, output: &Path) -> Result<(), NativeError> {
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("xcrun");
        command.arg("clang");
        command
    };
    #[cfg(not(target_os = "macos"))]
    let mut command = Command::new("clang");

    let result = command
        .arg(object)
        .arg("-o")
        .arg(output)
        .output()
        .map_err(NativeError::LinkerUnavailable)?;
    if !result.status.success() {
        return Err(NativeError::LinkerFailed {
            status: result.status.code(),
            stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
        });
    }
    Ok(())
}

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn create(parent: &Path) -> Result<Self, NativeError> {
        for _ in 0..100 {
            let id = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(".cheeku-{}-{id}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(filesystem_error(&path, error)),
            }
        }
        Err(filesystem_error(
            parent,
            io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Cannot allocate a temporary compilation directory",
            ),
        ))
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Stages object and executable beside the output, publishing only after linking succeeds.
pub fn build_executable(module: &Module<'_>, output: &Path) -> Result<(), NativeError> {
    build_executable_with_options(module, output, &BuildOptions::default()).map(|_| ())
}

pub fn build_executable_with_options(
    module: &Module<'_>,
    output: &Path,
    options: &BuildOptions,
) -> Result<NativeTimings, NativeError> {
    let output = std::path::absolute(output).map_err(|error| filesystem_error(output, error))?;
    let parent = output.parent().ok_or_else(|| {
        filesystem_error(
            &output,
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Output must name an executable file",
            ),
        )
    })?;
    let temporary = TemporaryDirectory::create(parent)?;
    let object = temporary.0.join("program.o");
    let executable = temporary.0.join("program");
    let mut timings = emit_object_with_options(module, &object, options)?;
    let start = Instant::now();
    link_object(&object, &executable)?;
    timings.link = start.elapsed();
    fs::rename(&executable, &output).map_err(|error| filesystem_error(&output, error))?;
    Ok(timings)
}
