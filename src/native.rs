use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use inkwell::OptimizationLevel;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};

#[derive(Debug)]
pub enum NativeError {
    Target(String),
    Verification(String),
    ObjectEmission(String),
    Filesystem { path: PathBuf, source: io::Error },
    LinkerUnavailable(io::Error),
    LinkerFailed { status: Option<i32>, stderr: String },
}

impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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

/// Emits host-native object bytes, using LLVM's discovered triple and data layout.
pub fn emit_object(module: &Module<'_>, output: &Path) -> Result<(), NativeError> {
    Target::initialize_native(&InitializationConfig::default()).map_err(NativeError::Target)?;
    let triple = TargetMachine::get_default_triple();
    let target =
        Target::from_triple(&triple).map_err(|error| NativeError::Target(error.to_string()))?;
    let cpu = TargetMachine::get_host_cpu_name();
    let features = TargetMachine::get_host_cpu_features();
    let cpu = cpu
        .to_str()
        .map_err(|error| NativeError::Target(error.to_string()))?;
    let features = features
        .to_str()
        .map_err(|error| NativeError::Target(error.to_string()))?;
    let machine = target
        .create_target_machine(
            &triple,
            cpu,
            features,
            OptimizationLevel::None,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .ok_or_else(|| {
            NativeError::Target("LLVM could not create a host target machine".to_owned())
        })?;
    module.set_triple(&triple);
    module.set_data_layout(&machine.get_target_data().get_data_layout());
    module
        .verify()
        .map_err(|error| NativeError::Verification(error.to_string()))?;
    // Emit to memory so filesystem paths need not be UTF-8 (Inkwell's file API requires it).
    let object = machine
        .write_to_memory_buffer(module, FileType::Object)
        .map_err(|error| NativeError::ObjectEmission(error.to_string()))?;
    fs::write(output, object.as_slice()).map_err(|error| filesystem_error(output, error))
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
    emit_object(module, &object)?;
    link_object(&object, &executable)?;
    fs::rename(&executable, &output).map_err(|error| filesystem_error(&output, error))?;
    Ok(())
}
