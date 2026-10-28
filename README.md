# chuchu-lang

Chuchu is a high-performance, statically typed programming language with automatic type inference, designed for simplicity and fast numerical computation, built in Rust.

Declare variables with `assign` and update them with `reassign`:

```chuchu
assign x to 5
reassign x to 6
show x
```

The previous `set x to 5` declaration syntax is no longer accepted.

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
# Prints: 35 (the current example reassigns x to 15)
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


## Optimization and performance

Cheeku defaults to `-O2`. Use `-O0` for the unoptimized IR baseline, `-O1` for
scalar optimization or `-O3` for LLVM's more aggressive standard pipeline.
Optimization and output flags can appear in either order after the source:

```sh
./target/release/cheeku examples/main.chuchu -O3 -o my_program
./target/release/cheeku examples/main.chuchu -o my_program -O0
./target/release/cheeku examples/main.chuchu --time-compilation
./target/release/cheeku examples/numerical.chuchu -O2 --emit-ir numerical.ll --emit-assembly numerical.s -o numerical
```

The default target CPU is generic for the detected host architecture. Explicit
`--cpu native` enables host-specific CPU features. Checked arithmetic, array
bounds and floating-point semantics are preserved at every level. The timing
option prints stage durations as JSON on stderr; normal compilation is quiet.
`--no-vectorize` disables loop/SLP options for investigation but does not guarantee
scalar machine code.

See [optimization/API details](docs/optimization.md),
[measurement methodology](docs/performance.md), the
[benchmark runner](benchmarks/README.md), and the
[Phase 7 results and limitations](docs/phase7-performance-report.md).
