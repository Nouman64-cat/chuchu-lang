mod support;
use chuchu::codegen::generate_program_with_options;
use chuchu::compiler::{CompileError, compile_file_with_options, compile_source_with_options};
use chuchu::lexer::tokenize;
use chuchu::native::debug_symbol_path;
use chuchu::optimization::{BuildOptions, DebugInfo, Optimization};
use chuchu::parser::Parser;
use chuchu::source::SourceFile;
use inkwell::context::Context;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "cheeku debug {} {}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn clean(&self) {
        assert!(fs::read_dir(&self.0).unwrap().all(|entry| {
            !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".cheeku-")
        }));
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn options() -> BuildOptions {
    BuildOptions {
        debug_info: DebugInfo::Full,
        optimization: Optimization::O0,
        ..BuildOptions::default()
    }
}

#[test]
fn debug_builds_preserve_success_and_runtime_failures_at_every_level() {
    let directory = Directory::new();
    let input = directory.0.join("case.chuchu");
    let baseline = directory.0.join(support::executable_name("baseline"));
    let output = directory.0.join(support::executable_name("debug"));
    for source in [
        SOURCE,
        "show 9223372036854775807 + 1",
        "assign a [1, 2]\nshow a[2]",
        "show 1 / 0",
    ] {
        fs::write(&input, source).unwrap();
        compile_file_with_options(&input, &baseline, &BuildOptions::default()).unwrap();
        let expected = Command::new(&baseline).output().unwrap();
        for optimization in [
            Optimization::O0,
            Optimization::O1,
            Optimization::O2,
            Optimization::O3,
        ] {
            compile_file_with_options(
                &input,
                &output,
                &BuildOptions {
                    optimization,
                    ..options()
                },
            )
            .unwrap();
            let actual = Command::new(&output).output().unwrap();
            assert_eq!(actual.status.code(), expected.status.code());
            assert_eq!(actual.stdout, expected.stdout);
            assert_eq!(actual.stderr, expected.stderr);
        }
    }
    directory.clean();
}

#[test]
fn debug_lowering_reports_invalid_positions_and_drops_builders_safely() {
    for text in [
        "show unknown".to_owned(),
        format!("{}show 1", " ".repeat(65536)),
    ] {
        let context = Context::create();
        let statements = Parser::new(tokenize(&text).unwrap())
            .parse_program()
            .unwrap();
        let source = SourceFile::new(std::env::temp_dir().join("invalid.chuchu"), text);
        assert!(
            generate_program_with_options(&context, &statements, Some(&source), &options())
                .is_err()
        );
    }
    let context = Context::create();
    let source = SourceFile::new(std::env::temp_dir().join("empty.chuchu"), String::new());
    generate_program_with_options(&context, &[], Some(&source), &options())
        .unwrap()
        .verify()
        .unwrap();
}

const SOURCE: &str = "function twice(n: int) -> int {\n    return n * 2\n}\nassign x 3\nassign ratio 1.5\nassign ready true\nassign values [2, 4, 6]\nwhile x < 5 {\n    reassign x x + 1\n}\nif ready {\n    assign x twice(x)\n    show x\n}\nshow x\nshow ratio\nshow values[1]\n";

#[test]
fn metadata_has_real_source_scopes_types_and_storage_at_every_level() {
    let statements = Parser::new(tokenize(SOURCE).unwrap())
        .parse_program()
        .unwrap();
    let source = SourceFile::new(std::env::temp_dir().join("debug.chuchu"), SOURCE.into());
    for optimization in [
        Optimization::O0,
        Optimization::O1,
        Optimization::O2,
        Optimization::O3,
    ] {
        let context = Context::create();
        let options = BuildOptions {
            optimization,
            ..options()
        };
        let module =
            generate_program_with_options(&context, &statements, Some(&source), &options).unwrap();
        module.verify().unwrap();
        let ir = module.print_to_string().to_string();
        for metadata in [
            "DICompileUnit",
            "DISubprogram",
            "DILexicalBlock",
            "DILocalVariable",
            "DICompositeType",
            "DILocation",
            "debug.chuchu",
            "chuchu.twice",
            "arg: 1",
            "name: \"Bool\", size: 8",
            "count: 3",
        ] {
            assert!(ir.contains(metadata), "Missing {metadata}: {ir}");
        }
        assert!(
            module
                .get_function("main")
                .unwrap()
                .get_subprogram()
                .is_some()
        );
        let machine = chuchu::optimization::target_machine(&module, &options).unwrap();
        chuchu::optimization::optimize(&module, &machine, &options).unwrap();
        module.verify().unwrap();
        assert_eq!(
            ir.contains("isOptimized: true"),
            optimization != Optimization::O0
        );
    }
}

#[test]
fn debug_binaries_execute_after_staging_cleanup_and_can_be_replaced() {
    let directory = Directory::new();
    let input = directory.0.join("source with spaces.chuchu");
    let output = directory.0.join(support::executable_name("program"));
    fs::write(&input, SOURCE.replace('\n', "\r\n")).unwrap();
    compile_file_with_options(&input, &output, &options()).unwrap();
    directory.clean();
    let run = Command::new(&output).output().unwrap();
    assert!(run.status.success());
    assert_eq!(run.stdout, support::native_output("10\n5\n1.5\n4\n"));
    if let Some(symbols) = debug_symbol_path(&output) {
        assert!(symbols.exists());
        #[cfg(target_os = "windows")]
        assert!(
            fs::read(symbols)
                .unwrap()
                .starts_with(b"Microsoft C/C++ MSF 7.00")
        );
    }
    fs::write(&input, "show 42\n").unwrap();
    compile_file_with_options(&input, &output, &options()).unwrap();
    assert_eq!(
        Command::new(&output).output().unwrap().stdout,
        support::native_output("42\n")
    );
    directory.clean();
}

#[test]
fn anonymous_debug_source_and_conflicting_sidecars_are_rejected() {
    let directory = Directory::new();
    let output = directory.0.join("output");
    assert!(matches!(
        compile_source_with_options("show 1", &output, &options()),
        Err(CompileError::MissingDebugSource)
    ));
    assert!(!output.exists());
    let input = directory.0.join("input.chuchu");
    fs::write(&input, "show 1").unwrap();
    if let Some(symbols) = debug_symbol_path(&output) {
        let options = BuildOptions {
            emit_ir: Some(symbols),
            ..options()
        };
        assert!(matches!(
            compile_file_with_options(&input, &output, &options),
            Err(CompileError::ConflictingOutputs(_))
        ));
        assert!(!output.exists());
    }
}

#[cfg(target_os = "macos")]
#[test]
fn failed_symbol_tool_preserves_previous_build_and_cleans_staging() {
    let directory = Directory::new();
    let input = directory.0.join("input.chuchu");
    let output = directory.0.join("program");
    fs::write(&input, "show 1").unwrap();
    compile_file_with_options(&input, &output, &options()).unwrap();
    let previous = fs::read(&output).unwrap();
    let symbols = debug_symbol_path(&output).unwrap();
    let previous_dwarf = fs::read(symbols.join("Contents/Resources/DWARF/program")).unwrap();
    fs::write(&input, "show 2").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_cheeku"))
        .arg(&input)
        .arg("--debug")
        .arg("-o")
        .arg(&output)
        .env("CHEEKU_DSYMUTIL", directory.0.join("missing-dsymutil"))
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert_eq!(fs::read(&output).unwrap(), previous);
    assert_eq!(
        fs::read(symbols.join("Contents/Resources/DWARF/program")).unwrap(),
        previous_dwarf
    );
    directory.clean();
}

#[test]
fn named_source_must_match_disk_and_debug_program_o_output_does_not_collide() {
    let directory = Directory::new();
    let input = directory.0.join("input.chuchu");
    fs::write(&input, "show 1").unwrap();
    let output = directory.0.join("program.o");
    let wrong = SourceFile::new(input.clone(), "show 2".into());
    assert!(
        chuchu::compiler::compile_named_source_with_options(&wrong, &output, &options()).is_err()
    );
    compile_file_with_options(&input, &output, &options()).unwrap();
    directory.clean();
    let result = Command::new(&output).output().unwrap();
    assert!(result.status.success());
    assert_eq!(result.stdout, support::native_output("1\n"));
}
