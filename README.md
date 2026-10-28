# chuchu-lang
Chuchu is a high-performance, statically typed programming language with automatic type inference, designed for simplicity and fast numerical computation, built in Rust.

## LLVM setup on macOS

Cheeku uses LLVM 23.1 through Inkwell, preferring dynamic linking. With LLVM
installed through Homebrew, configure discovery in the shell before running Cargo:

```sh
export LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"
cargo test
```

The Homebrew LLVM installation must provide LLVM 23.1.x. The prefix is resolved
locally rather than stored as a machine-specific path in the repository.

## Build and run Cheeku

Native linking on macOS requires Xcode or the Xcode command line tools, including
`xcrun clang`. The Rust library and Cargo package remain named `chuchu`; the compiler
binary is named `cheeku`.

```sh
export LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"
cargo build --release
./target/release/cheeku examples/main.chuchu
./main
# Prints: 30
```

The default executable is `main` in the current working directory. Choose another
output with `-o`:

```sh
./target/release/cheeku examples/main.chuchu -o my_program
./my_program
```

To make the `cheeku` command available on your Cargo binary path, run
`cargo install --path .` with the same LLVM environment variable set. Then use
`cheeku examples/main.chuchu`.

Cheeku validates the source through the lexer, parser, and type checker before
generating LLVM IR. Locals use scoped entry-block storage for reassignment and
control flow. `show` uses type-aware C `printf` output for the scalar int, float,
and bool types.
LLVM emits a host-native object with its detected target triple and
data layout; system Clang links it. Compilation never runs the generated program.
Temporary objects are cleaned up, and the requested output is replaced only after
linking succeeds.

## Phase 6 language features

- int, float, and bool types, with automatic local type inference and int-to-float promotion.
- Arithmetic `+`, `-`, `*`, `/`, unary `-`, parentheses, and comparisons.
- Braced `if`/`else` and `while`, reassignment, lexical scopes, and inner shadowing.
- Top-level functions with typed parameters/results, forward calls, and recursion.
- Homogeneous fixed-length numerical arrays, checked indexing, array copying,
  element-wise `+` and `*`, and `sum`/`len`.
- Scalar numerical built-ins `abs`, `sqrt`, `min`, and `max`.

```text
function square(x: int) -> int {
    return x * x
}
set numbers to [1, 2, 3]
show sum(numbers * numbers)
show square(10)
```

This prints `14` and `100`. More runnable programs are in `examples/arithmetic.chuchu`,
`conditions.chuchu`, `loops.chuchu`, `functions.chuchu`, `arrays.chuchu`,
`floats.chuchu`, and `numerical.chuchu` (all under `examples/`).

The provisional v0.2 policy checks signed i64 arithmetic and bounds at runtime.
Failures print byte spans to stderr and exit with status 1. Float uses binary64 and
15-significant-digit output. Arrays have value-copy semantics, use bounded stack
storage, and are limited to 256 numerical elements. Function annotations are scalar;
array parameters/results, dynamic resizing, and closures are unsupported. Unbounded
recursion may exhaust the native stack. No SIMD or performance guarantee is claimed.

See [the v0.2 language specification](docs/language-spec-v0.2.md) for grammar,
numeric semantics, scopes, memory behavior, and provisional design decisions.
Entry-block storage is suitable for later LLVM promotion; Phase 7 optimization
work is intentionally deferred. Phase 5 integer-only lowering helpers remain
available for compatibility; the complete typed source-language API is
`codegen::generate_program`, used by `compiler::compile_source`/`compile_file`.

## Validation

With LLVM configured, run:

```sh
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Native integration tests are gated to macOS and exercise real linking and execution
in isolated temporary directories. They require the system development tools.
