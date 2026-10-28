#![cfg(target_os = "macos")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use chuchu::codegen::{generate_module, generate_program};
use chuchu::compiler::{CompileError, compile_source};
use chuchu::native::{NativeError, emit_object, link_object};
use inkwell::context::Context;

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        for _ in 0..100 {
            let id = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path =
                std::env::temp_dir().join(format!("cheeku test {} {id}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("Cannot create test directory: {error}"),
            }
        }
        panic!("Cannot create unique test directory");
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    fn compiler(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cheeku"));
        command.current_dir(&self.0);
        command
    }

    fn assert_no_temporary_objects(&self) {
        for entry in fs::read_dir(&self.0).unwrap() {
            let name = entry.unwrap().file_name();
            assert!(!name.to_string_lossy().starts_with(".cheeku-"));
            assert!(!name.to_string_lossy().ends_with(".o"));
        }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "status: {:?}, stderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn run_executable(path: &Path, expected: &str) {
    assert!(path.is_file());
    let output = Command::new(path)
        .output()
        .expect("Native executable should run");
    assert_success(&output);
    assert_eq!(output.stdout, expected.as_bytes());
    assert!(output.stderr.is_empty());
}

#[test]
fn compiles_example_to_default_executable_without_running_it() {
    let directory = TestDirectory::new();
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/main.chuchu");
    let output = directory.compiler().arg(example).output().unwrap();
    assert_success(&output);
    assert!(
        output.stdout.is_empty(),
        "Compilation must not run the program"
    );
    run_executable(&directory.path("main"), "35\n");
    directory.assert_no_temporary_objects();
}

#[test]
fn compiles_positive_language_cases() {
    for (source, expected) in [
        ("assign x to 5\nshow x", "5\n"),
        ("show 42", "42\n"),
        ("show 10 + 20", "30\n"),
        ("assign x to 10\nassign y to x + 20\nshow y", "30\n"),
        ("show 10 + 20 + 30", "60\n"),
        ("assign x to 10\nshow x\nshow x + 20\nshow 0", "10\n30\n0\n"),
        ("show 9223372036854775807", "9223372036854775807\n"),
        ("", ""),
        ("\n\n", ""),
    ] {
        let directory = TestDirectory::new();
        let input = directory.path("case.chuchu");
        let executable = directory.path("program");
        fs::write(&input, source).unwrap();
        let output = directory
            .compiler()
            .arg(&input)
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
        assert_success(&output);
        assert!(output.stdout.is_empty());
        run_executable(&executable, expected);
        directory.assert_no_temporary_objects();
    }
}

#[test]
fn supports_spaces_and_shell_characters_in_paths() {
    let directory = TestDirectory::new();
    let input = directory.path("source ; $(echo bad).chuchu");
    let executable = directory.path("output ; $(echo bad)");
    fs::write(&input, "show 42").unwrap();
    let output = directory
        .compiler()
        .arg(&input)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert_success(&output);
    run_executable(&executable, "42\n");
    directory.assert_no_temporary_objects();
}

#[test]
fn rejects_invalid_sources_without_creating_executables() {
    for (source, diagnostic) in [
        ("set x to 5", "Expected statement"),
        ("show unknown", "Undefined variable"),
        ("assign x to 1\nassign x to 2", "Duplicate declaration"),
        ("show @", "Invalid source character"),
        ("assign x 10", "Expected 'to'"),
        ("show", "Expected expression"),
        ("show 1 show 2", "Expected newline"),
        ("show 9223372036854775808", "out-of-range"),
    ] {
        let directory = TestDirectory::new();
        let input = directory.path("invalid.chuchu");
        let executable = directory.path("main");
        fs::write(&input, source).unwrap();
        let output = directory.compiler().arg(&input).output().unwrap();
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(diagnostic),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!executable.exists());
        assert!(output.stdout.is_empty());
        directory.assert_no_temporary_objects();
    }
}

#[test]
fn pipeline_validates_semantics_before_llvm_generation() {
    let directory = TestDirectory::new();
    let output = directory.path("main");
    assert!(matches!(
        compile_source("show 9223372036854775808\nshow unknown", &output),
        Err(CompileError::Type(_))
    ));
    assert!(!output.exists());
}

#[test]
fn validates_cli_arguments_and_source_paths() {
    let directory = TestDirectory::new();
    for (args, diagnostic) in [
        (vec![], "Usage:"),
        (vec!["missing.chuchu"], "Cannot read source"),
        (vec!["wrong.txt"], "Expected a .chuchu"),
        (vec!["--unknown"], "Invalid source argument"),
        (vec!["input.chuchu", "-o"], "Usage:"),
        (vec!["input.chuchu", "--bad"], "Unknown argument"),
        (vec!["input.chuchu", "-o", "output", "extra"], "Usage:"),
    ] {
        let output = directory.compiler().args(args).output().unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains(diagnostic));
        assert!(!directory.path("main").exists());
    }
}

#[test]
fn preserves_existing_files_on_failure_and_protects_source() {
    let directory = TestDirectory::new();
    let input = directory.path("input.chuchu");
    let output = directory.path("main");
    fs::write(&input, "show unknown").unwrap();
    fs::write(&output, b"previous executable").unwrap();
    let result = directory.compiler().arg(&input).output().unwrap();
    assert!(!result.status.success());
    assert_eq!(fs::read(&output).unwrap(), b"previous executable");

    fs::write(&input, "show 42").unwrap();
    let result = directory
        .compiler()
        .arg(&input)
        .arg("-o")
        .arg(&input)
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("overwrite source"));
    assert_eq!(fs::read_to_string(&input).unwrap(), "show 42");
}

#[test]
fn emits_host_object_and_reports_emission_errors() {
    let directory = TestDirectory::new();
    let context = Context::create();
    let module = generate_module(&context).unwrap();
    let object = directory.path("native.o");
    emit_object(&module, &object).expect("Object should emit");
    assert!(!fs::read(&object).unwrap().is_empty());
    assert!(!module.get_triple().as_str().to_bytes().is_empty());
    assert!(!module.get_data_layout().as_str().to_bytes().is_empty());
    assert!(matches!(
        emit_object(&module, &directory.path("missing/native.o")),
        Err(NativeError::Filesystem { .. })
    ));

    let invalid = context.create_module("invalid");
    let function = invalid.add_function("main", context.i32_type().fn_type(&[], false), None);
    context.append_basic_block(function, "unterminated");
    assert!(matches!(
        emit_object(&invalid, &directory.path("invalid.o")),
        Err(NativeError::Verification(_))
    ));
    assert!(!directory.path("invalid.o").exists());
}

#[test]
fn reports_linker_failure_with_status_and_diagnostics() {
    let directory = TestDirectory::new();
    let object = directory.path("invalid.o");
    let output = directory.path("program");
    fs::write(&object, b"not an object file").unwrap();
    match link_object(&object, &output) {
        Err(NativeError::LinkerFailed { status, stderr }) => {
            assert!(status.is_some_and(|status| status != 0));
            assert!(!stderr.is_empty());
        }
        result => panic!("Expected linker failure, got {result:?}"),
    }
    assert!(!output.exists());
}

#[test]
fn reports_unavailable_linker_and_cleans_temporary_files() {
    let directory = TestDirectory::new();
    let input = directory.path("input.chuchu");
    fs::write(&input, "show 42").unwrap();
    let output = directory
        .compiler()
        .arg(input)
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Cannot start the system Clang"));
    assert!(!directory.path("main").exists());
    directory.assert_no_temporary_objects();
}

#[test]
fn links_an_empty_program_to_a_successful_native_executable() {
    let directory = TestDirectory::new();
    let context = Context::create();
    let module = generate_program(&context, &[]).unwrap();
    let object = directory.path("empty.o");
    let output = directory.path("empty");
    emit_object(&module, &object).unwrap();
    link_object(&object, &output).unwrap();
    run_executable(&output, "");
}
