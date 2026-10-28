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
