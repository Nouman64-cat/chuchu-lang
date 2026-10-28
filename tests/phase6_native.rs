#![cfg(target_os = "macos")]

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Case {
    directory: PathBuf,
}
impl Case {
    fn new() -> Self {
        for _ in 0..100 {
            let path = std::env::temp_dir().join(format!(
                "cheeku phase6 {} {}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self { directory: path },
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("Cannot create test directory: {error}"),
            }
        }
        panic!("Cannot allocate test directory");
    }
    fn compile(&self, source: &str) -> Output {
        let input = self.directory.join("case.chuchu");
        fs::write(&input, source).unwrap();
        Command::new(env!("CARGO_BIN_EXE_cheeku"))
            .arg(input)
            .current_dir(&self.directory)
            .output()
            .unwrap()
    }
    fn run(&self) -> Output {
        Command::new(self.directory.join("main")).output().unwrap()
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn execute(source: &str) -> Output {
    let case = Case::new();
    let compiled = case.compile(source);
    assert!(
        compiled.status.success(),
        "Source:\n{source}\n{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    assert!(compiled.stdout.is_empty());
    case.run()
}
fn succeeds(source: &str, expected: &str) {
    let output = execute(source);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, expected.as_bytes(), "{source}");
    assert!(output.stderr.is_empty());
}

#[test]
fn arithmetic_precedence_and_negation() {
    succeeds(
        "set a to 10\nset b to 20\nshow a + b * 2\nshow (a + b) * 2\nshow -a + b\nshow 20 / 2 / 2\nshow 20 - 3 - 2\nshow -7 / 2",
        "50\n60\n10\n5\n15\n-3\n",
    );
}
#[test]
fn signed_literal_boundaries() {
    succeeds(
        "show -9223372036854775808\nshow -0009223372036854775808\nshow 9223372036854775807",
        "-9223372036854775808\n-9223372036854775808\n9223372036854775807\n",
    );
}
#[test]
fn floats_and_numeric_promotion() {
    succeeds(
        "set pi to 3.14159\nset radius to 5.0\nshow pi * radius * radius\nshow 5 / 2.0\nshow 1 + 2.5\nshow -2.5 * 2\nshow 1e2 + 2.5e-1",
        "78.53975\n2.5\n3.5\n-5\n100.25\n",
    );
}
#[test]
fn bool_and_numeric_comparisons() {
    succeeds(
        "show true\nshow false\nshow 1 == 1.0\nshow 1 != 2\nshow 2 < 3\nshow 3 <= 3\nshow 4 > 3\nshow 4 >= 4\nshow true == false",
        "true\nfalse\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\nfalse\n",
    );
}
#[test]
fn conditionals_and_nested_branches() {
    succeeds(
        "set score to 85\nif score >= 80 {\nif score < 90 { show 1 } else { show 2 }\n} else { show 0 }\nif false { show 99 }\nshow 3",
        "1\n3\n",
    );
}
#[test]
fn reassignment_and_conditional_storage() {
    succeeds(
        "set x to 10\nx = 20\nif true { x = x + 5 } else { x = 0 }\nshow x\nset f to 1.0\nf = x\nshow f",
        "25\n25\n",
    );
}
#[test]
fn while_reevaluates_conditions() {
    succeeds(
        "set counter to 0\nwhile counter < 5 {\nshow counter\ncounter = counter + 1\n}",
        "0\n1\n2\n3\n4\n",
    );
}
#[test]
fn nested_loops_and_block_shadowing() {
    succeeds(
        "set total to 0\nset i to 0\nwhile i < 3 {\nset j to 0\nwhile j < 2 {\ntotal = total + 1\nj = j + 1\n}\ni = i + 1\n}\nset x to 10\nif true {\nset x to x + 20\nshow x\nx = 40\n}\nshow x\nshow total",
        "30\n10\n6\n",
    );
}
#[test]
fn forward_function_calls_and_multiple_parameters() {
    succeeds(
        "show add(square(3), 0.5)\nfn square(x: Int) -> Int { return x * x }\nfn add(x: Float, y: Float) -> Float { return x + y }\nfn positive(x: Int) -> Bool { return x > 0 }\nshow positive(2)",
        "9.5\ntrue\n",
    );
}
#[test]
fn recursive_functions_and_conditional_returns() {
    succeeds(
        "fn factorial(n: Int) -> Int {\nif n <= 1 { return 1 } else { return n * factorial(n - 1) }\n}\nshow factorial(6)",
        "720\n",
    );
}
#[test]
fn mutual_recursion_and_return_inside_loop() {
    succeeds(
        "fn even(n: Int) -> Bool {\nif n == 0 { return true } else { return odd(n - 1) }\n}\nfn odd(n: Int) -> Bool {\nif n == 0 { return false } else { return even(n - 1) }\n}\nfn first(n: Int) -> Int {\nwhile n > 0 { return n }\nreturn 0\n}\nshow even(8)\nshow first(3)",
        "true\n3\n",
    );
}
#[test]
fn integer_and_float_arrays() {
    succeeds(
        "set a to [10, 20, 30, 40]\nset b to [1.5, 2.5, 3.5]\nshow a[0]\nshow a[2]\nshow b[1]\nshow len(a)\nshow sum(b)",
        "10\n30\n2.5\n4\n7.5\n",
    );
    let maximum = format!(
        "set a to [{}]\nshow len(a)\nshow sum(a)",
        vec!["1"; 256].join(",")
    );
    succeeds(&maximum, "256\n256\n");
}
#[test]
fn array_assignment_copies_values() {
    succeeds(
        "set a to [1, 2]\nset b to a\na = [3, 4]\nshow b[0]\nshow a[0]\nb = b + a\nshow sum(b)",
        "1\n3\n10\n",
    );
}
#[test]
fn array_arithmetic_and_reductions() {
    succeeds(
        "set a to [1, 2, 3]\nset b to [4, 5, 6]\nset result to a * b\nshow sum(result)\nshow sum(a + b)\nset f to [1.5, 2.5] * [2.0, 4.0]\nshow sum(f)",
        "32\n21\n13\n",
    );
}
#[test]
fn numerical_builtins_and_nan_comparisons() {
    succeeds(
        "show abs(-7)\nshow abs(-3.5)\nshow sqrt(9.0)\nshow min(2, 3.5)\nshow max(2.0, 3)\nshow sqrt(-1.0) != sqrt(-1.0)\nshow min(sqrt(-1.0), 2.0)",
        "7\n3.5\n3\n2\n3\ntrue\n2\n",
    );
}
#[test]
fn loop_array_temporaries_have_bounded_lifetime() {
    succeeds(
        "set i to 0\nset total to 0\nwhile i < 1000 {\nset a to [1, 2, 3] * [4, 5, 6]\ntotal = total + sum(a)\ni = i + 1\n}\nshow total",
        "32000\n",
    );
}
#[test]
fn runtime_arithmetic_failures_are_diagnosed() {
    for (source, diagnostic) in [
        ("show 9223372036854775807 + 1", "integer overflow"),
        ("show -9223372036854775808 - 1", "integer overflow"),
        ("show 9223372036854775807 * 2", "integer overflow"),
        ("set x to -9223372036854775808\nshow -x", "integer overflow"),
        ("show 1 / 0", "division by zero"),
        ("show 1.0 / 0.0", "division by zero"),
        ("show -9223372036854775808 / -1", "division overflow"),
        ("show abs(-9223372036854775808)", "abs overflow"),
        ("show sum([9223372036854775807, 1])", "integer overflow"),
        (
            "set a to [9223372036854775807] + [1]\nshow a[0]",
            "integer overflow",
        ),
    ] {
        let output = execute(source);
        assert_eq!(output.status.code(), Some(1), "{source}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains(diagnostic), "{source}: {message}");
        assert!(message.contains(" at "));
    }
}
#[test]
fn runtime_bounds_checks_cover_negative_and_upper_indices() {
    for source in [
        "show [1, 2][-1]",
        "show [1, 2][2]",
        "set i to 3\nshow [1.5][i]",
    ] {
        let output = execute(source);
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("array index out of bounds"));
    }
}
#[test]
fn invalid_phase6_programs_preserve_existing_executable() {
    let case = Case::new();
    let executable = case.directory.join("main");
    fs::write(&executable, b"previous executable").unwrap();
    for source in [
        "show true + 1",
        "if 1 { show 1 }",
        "while 2.0 { show 1 }",
        "x = 1",
        "set x to 1\nx = false",
        "if true { set x to 1 }\nshow x",
        "fn f(x: Int) -> Int { return x }\nshow f(true)",
        "fn f() -> Int { show 1 }",
        "show missing(1)",
        "set a to [1, 2.0]",
        "show [1][true]",
        "set a to [1] + [1, 2]",
        "fn f(x: Array) -> Int { return 1 }",
        "show 1e9999",
        "show -9223372036854775809",
    ] {
        let output = case.compile(source);
        assert!(!output.status.success(), "{source}");
        assert!(!output.stderr.is_empty());
        assert_eq!(fs::read(&executable).unwrap(), b"previous executable");
    }
}
