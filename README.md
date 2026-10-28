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
generating LLVM IR. Immutable declarations bind names directly to LLVM SSA values.
`show` prints a signed integer using the C runtime's `printf` and a shared `%lld\n`
format string. LLVM emits a host-native object with its detected target triple and
data layout; system Clang links it. Compilation never runs the generated program.
Temporary objects are cleaned up, and the requested output is replaced only after
linking succeeds.

The initial `Int` backend representation is provisionally `i64`. Integer literals
are range checked; addition does not assert `nsw` or `nuw`. Language-level overflow
semantics and a long-term integer width remain undecided. This compiler currently
supports host-native compilation, immutable `set` declarations, integer addition,
and `show`; it does not provide reassignment or nested scopes.

## Validation

With LLVM configured, run:

```sh
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Native integration tests are gated to macOS and exercise real linking and execution
in isolated temporary directories. They require the system development tools.
