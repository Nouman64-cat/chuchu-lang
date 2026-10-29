use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;
use std::process::ExitCode;

use chuchu::optimization::{BuildOptions, Cpu, InvalidOptimization};

const USAGE: &str = "Usage: cheeku <source.chuchu> [-O0|-O1|-O2|-O3] [-o <executable>] [--cpu generic|native] [--time-compilation] [--emit-ir <file>] [--emit-assembly <file>] [--no-vectorize]";

#[derive(Debug)]
enum CliError {
    Usage,
    Argument(String),
    Optimization(InvalidOptimization),
    Compile {
        input: PathBuf,
        source: chuchu::compiler::CompileError,
    },
}
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => write!(f, "{USAGE}"),
            Self::Argument(message) => write!(f, "{message}\n{USAGE}"),
            Self::Optimization(error) => write!(f, "{error}"),
            Self::Compile { input, source } => write!(f, "{}: {source}", input.display()),
        }
    }
}
#[derive(Debug)]
struct Invocation {
    input: PathBuf,
    output: PathBuf,
    options: BuildOptions,
    time: bool,
}

fn parse_args(
    arguments: impl IntoIterator<Item = OsString>,
) -> Result<Option<Invocation>, CliError> {
    let mut args = arguments.into_iter();
    let input = args.next().ok_or(CliError::Usage)?;
    if input == "--help" || input == "-h" {
        if args.next().is_some() {
            return Err(CliError::Usage);
        }
        return Ok(None);
    }
    if input.as_encoded_bytes().starts_with(b"-") {
        return Err(CliError::Argument(format!(
            "Invalid source argument: {}",
            input.to_string_lossy()
        )));
    }
    let mut invocation = Invocation {
        input: input.into(),
        output: chuchu::native::DEFAULT_EXECUTABLE_NAME.into(),
        options: BuildOptions::default(),
        time: false,
    };
    let mut seen = std::collections::HashSet::new();
    while let Some(option) = args.next() {
        let flag = option
            .to_str()
            .ok_or_else(|| CliError::Argument("Non-UTF-8 option".to_owned()))?;
        let key = if flag.starts_with("-O") {
            "optimization"
        } else {
            flag
        };
        if !seen.insert(key.to_owned()) {
            return Err(CliError::Argument(format!("Duplicate option: {flag}")));
        }
        match flag {
            "-o" | "--emit-ir" | "--emit-assembly" | "--cpu" => {
                let value = args
                    .next()
                    .filter(|value| !value.is_empty())
                    .ok_or(CliError::Usage)?;
                match flag {
                    "-o" => invocation.output = value.into(),
                    "--emit-ir" => invocation.options.emit_ir = Some(value.into()),
                    "--emit-assembly" => invocation.options.emit_assembly = Some(value.into()),
                    _ => {
                        invocation.options.cpu = match value.to_str() {
                            Some("native") => Cpu::Native,
                            Some("generic") => Cpu::Generic,
                            _ => {
                                return Err(CliError::Argument(
                                    "CPU must be generic or native".to_owned(),
                                ));
                            }
                        }
                    }
                }
            }
            "--time-compilation" => invocation.time = true,
            "--no-vectorize" => invocation.options.vectorize = false,
            flag if flag.starts_with("-O") => {
                invocation.options.optimization = flag.parse().map_err(CliError::Optimization)?
            }
            _ => return Err(CliError::Argument(format!("Unknown argument: {flag}"))),
        }
    }
    Ok(Some(invocation))
}
fn run() -> Result<(), CliError> {
    let Some(invocation) = parse_args(std::env::args_os().skip(1))? else {
        println!("{USAGE}");
        return Ok(());
    };
    let timings = chuchu::compiler::compile_file_with_options(
        &invocation.input,
        &invocation.output,
        &invocation.options,
    )
    .map_err(|source| CliError::Compile {
        input: invocation.input,
        source,
    })?;
    if invocation.time {
        eprintln!("{}", timings.json());
    }
    Ok(())
}
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("cheeku: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chuchu::optimization::Optimization;
    fn args(values: &[&str]) -> Result<Option<Invocation>, CliError> {
        parse_args(values.iter().map(OsString::from))
    }
    #[test]
    fn default_is_o2_and_existing_command_is_valid() {
        let invocation = args(&["input.chuchu"]).unwrap().unwrap();
        assert_eq!(invocation.options.optimization, Optimization::O2);
        assert_eq!(
            invocation.output,
            PathBuf::from(chuchu::native::DEFAULT_EXECUTABLE_NAME)
        );
    }
    #[test]
    fn output_and_optimization_accept_both_orders() {
        for values in [
            vec!["input.chuchu", "-O3", "-o", "my program"],
            vec!["input.chuchu", "-o", "my program", "-O3"],
        ] {
            let invocation = args(&values).unwrap().unwrap();
            assert_eq!(invocation.options.optimization, Optimization::O3);
            assert_eq!(invocation.output, PathBuf::from("my program"));
        }
    }
    #[test]
    fn rejects_invalid_or_duplicate_flags() {
        assert!(matches!(
            args(&["input.chuchu", "-O9"]),
            Err(CliError::Optimization(_))
        ));
        for values in [
            vec!["input.chuchu", "-O0", "-O2"],
            vec!["input.chuchu", "-o"],
            vec!["input.chuchu", "--cpu", "unknown"],
            vec!["input.chuchu", "--unknown"],
        ] {
            assert!(args(&values).is_err());
        }
    }
    #[test]
    fn diagnostics_are_explicit_options() {
        let invocation = args(&[
            "input.chuchu",
            "--time-compilation",
            "--emit-ir",
            "out.ll",
            "--emit-assembly",
            "out.s",
            "--cpu",
            "native",
            "--no-vectorize",
        ])
        .unwrap()
        .unwrap();
        assert!(invocation.time);
        assert_eq!(invocation.options.cpu, Cpu::Native);
        assert!(!invocation.options.vectorize);
        assert!(invocation.options.emit_ir.is_some());
    }
}
