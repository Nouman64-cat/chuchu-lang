use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "Usage: cheeku <source.chuchu> [-o <executable>]";

fn run() -> Result<(), String> {
    let mut arguments = std::env::args_os().skip(1);
    let input = arguments.next().ok_or_else(|| USAGE.to_owned())?;
    if input == "--help" || input == "-h" {
        if arguments.next().is_some() {
            return Err(USAGE.to_owned());
        }
        println!("{USAGE}");
        return Ok(());
    }
    if input.as_encoded_bytes().starts_with(b"-") {
        return Err(format!(
            "Invalid source argument: {}\n{USAGE}",
            input.to_string_lossy()
        ));
    }
    let output = match arguments.next() {
        None => PathBuf::from("main"),
        Some(option) if option == "-o" => {
            let path = arguments
                .next()
                .filter(|path| path != OsStr::new(""))
                .ok_or_else(|| USAGE.to_owned())?;
            PathBuf::from(path)
        }
        Some(option) => {
            return Err(format!(
                "Unknown argument: {}\n{USAGE}",
                option.to_string_lossy()
            ));
        }
    };
    if arguments.next().is_some() {
        return Err(USAGE.to_owned());
    }
    let input = PathBuf::from(input);
    chuchu::compiler::compile_file(&input, &output)
        .map_err(|error| format!("{}: {error}", input.display()))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("cheeku: {message}");
            ExitCode::FAILURE
        }
    }
}
