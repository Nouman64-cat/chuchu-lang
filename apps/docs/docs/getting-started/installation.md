---
title: "Installation"
description: "Build and install the Cheeku compiler on macOS with Rust, LLVM 23.1, and Xcode command line tools."
---

The currently validated setup is **macOS on Apple Silicon**. Cheeku requires Rust with Edition 2024 support, LLVM **23.1.x**, and a system Clang linker.

## 1. Prepare the tools

Install [Rust](https://www.rust-lang.org/tools/install) and [Homebrew](https://brew.sh/) if you do not already have them. Install the Xcode command line tools when needed:

```bash
xcode-select --install
```

Check that Rust and Clang are available:

```bash
rustc --version
cargo --version
xcrun clang --version
```

## 2. Install and check LLVM

```bash
brew install llvm
export LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"
"$LLVM_SYS_231_PREFIX/bin/llvm-config" --version
```

The reported LLVM version must be **23.1.x**. The compiler's Inkwell configuration targets that version specifically. If Homebrew provides a different LLVM version, use a matching LLVM installation and set `LLVM_SYS_231_PREFIX` to its installation prefix.

Keep the environment variable set whenever you build, test, or install Cheeku. To persist it, add the export to your shell configuration after confirming that the prefix points to the correct version.

## 3. Build Cheeku

```bash
git clone https://github.com/chuchu-lang/chuchu-core.git
cd chuchu-core
cargo build --release
```

The compiler is available at `./target/release/cheeku`.

```bash
./target/release/cheeku --help
```

## 4. Install the command

From the `chuchu-core` directory, with the same LLVM environment configured:

```bash
cargo install --path .
cheeku --help
```

Ensure Cargo's binary directory is on your `PATH`. A standard Rust installation uses `~/.cargo/bin`.

## 5. Compile the included example

```bash
cheeku examples/main.chuchu -o hello
./hello
```

```text title="Output"
35
```

The included example declares `x = 10` and `y = 20`, then reassigns `x` to `15` before printing their sum.

## Troubleshooting installation

| Symptom | Check |
|---|---|
| LLVM cannot be found | Set `LLVM_SYS_231_PREFIX` and confirm `llvm-config` reports 23.1.x. |
| Clang cannot start | Confirm `xcrun clang --version` works and command line tools are installed. |
| `cheeku` is not found | Use `./target/release/cheeku` or add Cargo's binary directory to `PATH`. |
| Native output fails on another machine | Build on the destination platform; this is currently a host-native compiler. Avoid `--cpu native` when host-specific features are unwanted. |

Linux and Windows are not validated by this project. See [platform status](../project/status.md) before attempting a port.

**Next:** [Write your first program](first-program.md).
