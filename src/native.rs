use std::ffi::OsString;
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
                "Cannot start the system Clang linker driver ({}): {source}",
                linker_hint()
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

/// Default CLI output; explicitly requested paths are never rewritten.
#[cfg(target_os = "windows")]
pub const DEFAULT_EXECUTABLE_NAME: &str = "main.exe";
#[cfg(not(target_os = "windows"))]
pub const DEFAULT_EXECUTABLE_NAME: &str = "main";

fn linker_hint() -> &'static str {
    if cfg!(target_os = "macos") {
        "install Xcode command line tools (xcrun clang)"
    } else if cfg!(target_os = "windows") {
        "install LLVM clang-cl and Visual Studio C++ Build Tools with the Windows SDK; use an x64 Developer PowerShell"
    } else {
        "install Clang, a GNU linker, and the C library development packages"
    }
}

/// One executable path, never a shell command. Useful for controlled toolchain selection.
fn configured_linker() -> Option<OsString> {
    if let Some(linker) = std::env::var_os("CHEEKU_LINKER") {
        return Some(linker);
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var_os("LLVM_SYS_231_PREFIX")
            .map(|prefix| PathBuf::from(prefix).join("bin").join("clang-cl.exe"))
            .filter(|path| path.is_file())
            .map(Into::into)
    }
    #[cfg(not(target_os = "windows"))]
    {
        None
    }
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

/// Uses the platform Clang driver with separate arguments, never a shell.
/// Windows requires an x64 MSVC developer environment (LIB/INCLUDE/SDK).
pub fn link_object(object: &Path, output: &Path) -> Result<(), NativeError> {
    let object = std::path::absolute(object).map_err(|error| filesystem_error(object, error))?;
    let output = std::path::absolute(output).map_err(|error| filesystem_error(output, error))?;
    let mut command = linker_command(&object, &output);
    let result = command.output().map_err(NativeError::LinkerUnavailable)?;
    if !result.status.success() {
        // Some drivers report useful diagnostics on stdout (notably MSVC).
        let diagnostics = format!(
            "{}{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        return Err(NativeError::LinkerFailed {
            status: result.status.code(),
            stderr: format!("{diagnostics}\nToolchain: {}", linker_hint()),
        });
    }
    Ok(())
}

fn linker_command(object: &Path, output: &Path) -> Command {
    #[cfg(target_os = "windows")]
    {
        let mut command =
            Command::new(configured_linker().unwrap_or_else(|| "clang-cl.exe".into()));
        let mut destination = OsString::from("/Fe");
        destination.push(output);
        command
            .args(["/nologo", "/MD"])
            .arg(object)
            .arg(destination)
            .args([
                "/link",
                "/MACHINE:X64",
                "/SUBSYSTEM:CONSOLE",
                "/DEFAULTLIB:msvcrt.lib",
                "/DEFAULTLIB:legacy_stdio_definitions.lib",
            ]);
        command
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut command = if let Some(linker) = configured_linker() {
            Command::new(linker)
        } else if cfg!(target_os = "macos") {
            let mut driver = Command::new("xcrun");
            driver.arg("clang");
            driver
        } else {
            Command::new("clang")
        };
        // link_object normalizes both paths to absolute paths, so they cannot
        // be mistaken for options. Keep libm after the object for --as-needed.
        command.arg("-o").arg(output).arg(object);
        #[cfg(target_os = "linux")]
        command.arg("-lm");
        command
    }
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
    let object = temporary.0.join(if cfg!(windows) {
        "program.obj"
    } else {
        "program.o"
    });
    let executable = temporary.0.join(DEFAULT_EXECUTABLE_NAME);
    let mut timings = emit_object_with_options(module, &object, options)?;
    let start = Instant::now();
    link_object(&object, &executable)?;
    timings.link = start.elapsed();
    // Rust's rename replaces an existing destination on Unix and Windows. Never
    // remove the working executable first: a locked/invalid destination must fail
    // without deleting it. Both files are on the same filesystem. No universal
    // filesystem atomicity or power-loss durability guarantee is claimed.
    fs::rename(&executable, &output).map_err(|error| filesystem_error(&output, error))?;
    Ok(timings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linker_keeps_paths_as_individual_arguments() {
        let object = Path::new("object with spaces.obj");
        let output = Path::new("program with spaces.exe");
        let command = linker_command(object, output);
        let args: Vec<_> = command.get_args().collect();
        assert!(args.contains(&object.as_os_str()));
        #[cfg(not(windows))]
        {
            assert!(args.contains(&output.as_os_str()));
            #[cfg(target_os = "linux")]
            assert!(args.contains(&std::ffi::OsStr::new("-lm")));
        }
        #[cfg(windows)]
        assert!(args.contains(&std::ffi::OsStr::new("/Feprogram with spaces.exe")));
    }
}
