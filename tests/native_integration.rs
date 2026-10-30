use support::{executable_name, native_output};
mod support;
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
            assert!(!name.to_string_lossy().ends_with(".obj"));
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
    assert_eq!(output.stdout, native_output(expected).as_slice());
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
    run_executable(
        &directory.path(chuchu::native::DEFAULT_EXECUTABLE_NAME),
        "35\n",
    );
    directory.assert_no_temporary_objects();
}

#[test]
fn compiles_positive_language_cases() {
    for (source, expected) in [
        ("assign x as 5\nshow x", "5\n"),
        ("show 42", "42\n"),
        ("show 10 + 20", "30\n"),
        ("assign x as 10\nassign y as x + 20\nshow y", "30\n"),
        (
            "assign x as 10\nassign y as 20\n\nshow(x + y)\n\nreassign x as 30\n\nshow(x + y)\n",
            "30\n50\n",
        ),
        ("show 10 + 20 + 30", "60\n"),
        ("assign x as 10\nshow x\nshow x + 20\nshow 0", "10\n30\n0\n"),
        ("show 9223372036854775807", "9223372036854775807\n"),
        ("", ""),
        ("\n\n", ""),
    ] {
        let directory = TestDirectory::new();
        let input = directory.path("case.chuchu");
        let executable = directory.path(&executable_name("program"));
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
    let executable = directory.path(&executable_name("output ; $(echo bad)"));
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
        ("assign x as 1\nassign x as 2", "Duplicate declaration"),
        ("show @", "Invalid source character"),
        ("assign x as", "Expected expression"),
        ("assign x 5", "Expected 'as'"),
        ("assign x as 1\nreassign x 5", "Expected 'as'"),
        ("assign x to 5", "Expected 'as'"),
        ("assign x as 1\nreassign x to 5", "Expected 'as'"),
        ("reassign x as", "Expected expression"),
        ("assign 5 10", "Expected identifier"),
        ("reassign 5 1", "Expected identifier"),
        ("assign x as 1\nreassign y as 2", "Undefined variable"),
        ("show", "Expected expression"),
        ("show 1 show 2", "Expected newline"),
        ("show 9223372036854775808", "out-of-range"),
    ] {
        let directory = TestDirectory::new();
        let input = directory.path("invalid.chuchu");
        let executable = directory.path(chuchu::native::DEFAULT_EXECUTABLE_NAME);
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
    let output = directory.path(chuchu::native::DEFAULT_EXECUTABLE_NAME);
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
        assert!(
            !directory
                .path(chuchu::native::DEFAULT_EXECUTABLE_NAME)
                .exists()
        );
    }
}

#[test]
fn preserves_existing_files_on_failure_and_protects_source() {
    let directory = TestDirectory::new();
    let input = directory.path("input.chuchu");
    let output = directory.path(chuchu::native::DEFAULT_EXECUTABLE_NAME);
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
    let bytes = fs::read(&object).unwrap();
    assert!(!bytes.is_empty());
    #[cfg(target_os = "linux")]
    assert_eq!(&bytes[..4], b"\x7fELF");
    #[cfg(target_os = "windows")]
    assert_eq!(&bytes[..2], &[0x64, 0x86]); // COFF IMAGE_FILE_MACHINE_AMD64
    #[cfg(target_os = "macos")]
    assert_eq!(&bytes[..4], &[0xcf, 0xfa, 0xed, 0xfe]); // 64-bit little-endian Mach-O
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
    let output = directory.path(&executable_name("program"));
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
        .env("CHEEKU_LINKER", directory.path("missing linker"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Cannot start the system Clang"));
    assert!(
        !directory
            .path(chuchu::native::DEFAULT_EXECUTABLE_NAME)
            .exists()
    );
    directory.assert_no_temporary_objects();
}

#[test]
fn links_an_empty_program_to_a_successful_native_executable() {
    let directory = TestDirectory::new();
    let context = Context::create();
    let module = generate_program(&context, &[]).unwrap();
    let object = directory.path("empty.o");
    let output = directory.path(&executable_name("empty"));
    emit_object(&module, &object).unwrap();
    link_object(&object, &output).unwrap();
    run_executable(&output, "");
}

#[test]
fn replaces_existing_executable_and_preserves_it_on_linker_failure() {
    let directory = TestDirectory::new();
    let input = directory.path("input.chuchu");
    let output = directory.path(&executable_name("replace me"));
    fs::write(&input, "show 1").unwrap();
    assert_success(
        &directory
            .compiler()
            .arg(&input)
            .arg("-o")
            .arg(&output)
            .output()
            .unwrap(),
    );
    run_executable(&output, "1\n");
    fs::write(&input, "show 2").unwrap();
    assert_success(
        &directory
            .compiler()
            .arg(&input)
            .arg("-o")
            .arg(&output)
            .output()
            .unwrap(),
    );
    run_executable(&output, "2\n");
    let prior = fs::read(&output).unwrap();
    let failed = directory
        .compiler()
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .env("CHEEKU_LINKER", directory.path("absent linker"))
        .output()
        .unwrap();
    assert!(!failed.status.success());
    assert_eq!(fs::read(&output).unwrap(), prior);
    run_executable(&output, "2\n");
    directory.assert_no_temporary_objects();
}

#[test]
fn explicit_extensionless_output_is_preserved_and_invalid_paths_fail() {
    let directory = TestDirectory::new();
    let input = directory.path("input.chuchu");
    fs::write(&input, "show 42").unwrap();
    let exact = directory.path("exact-name");
    assert_success(
        &directory
            .compiler()
            .arg(&input)
            .arg("-o")
            .arg(&exact)
            .output()
            .unwrap(),
    );
    assert!(exact.is_file());
    assert!(!directory.path("exact-name.exe").exists());
    for output in [directory.path("absent/hello.exe"), directory.0.clone()] {
        let failed = directory
            .compiler()
            .arg(&input)
            .arg("-o")
            .arg(output)
            .output()
            .unwrap();
        assert!(!failed.status.success());
    }
    directory.assert_no_temporary_objects();
}

#[test]
fn runtime_write_signature_matches_host_crt() {
    let context = Context::create();
    let statements = chuchu::parser::Parser::new(chuchu::lexer::tokenize("show 1 / 0").unwrap())
        .parse_program()
        .unwrap();
    let module = generate_program(&context, &statements).unwrap();
    let name = if cfg!(windows) { "_write" } else { "write" };
    let count_type = if cfg!(windows) {
        context.i32_type()
    } else {
        context.i64_type()
    };
    let write = module.get_function(name).unwrap();
    assert_eq!(write.get_type().get_return_type(), Some(count_type.into()));
    assert_eq!(write.get_type().get_param_types()[2], count_type.into());
    module.verify().unwrap();
}

#[cfg(windows)]
#[test]
fn locked_destination_is_not_deleted_on_publication_failure() {
    use std::os::windows::fs::OpenOptionsExt;
    let directory = TestDirectory::new();
    let input = directory.path("input.chuchu");
    let output = directory.path(&executable_name("locked"));
    fs::write(&input, "show 7").unwrap();
    assert_success(
        &directory
            .compiler()
            .arg(&input)
            .arg("-o")
            .arg(&output)
            .output()
            .unwrap(),
    );
    let original = fs::read(&output).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&output)
        .unwrap();
    fs::write(&input, "show 8").unwrap();
    let result = directory
        .compiler()
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(!result.status.success());
    drop(lock);
    assert_eq!(fs::read(&output).unwrap(), original);
    run_executable(&output, "7\n");
    directory.assert_no_temporary_objects();
}
