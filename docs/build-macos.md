# Build on macOS

Apple Silicon is locally validated and both macOS targets pass hosted native CI.
Use a Rust toolchain matching your hardware:
`aarch64-apple-darwin` or `x86_64-apple-darwin`. Rust 1.88+ is required; CI pins
1.99.1. Install Rust through [rustup](https://rustup.rs/).

Install Xcode command line tools, then LLVM 23.1.x through Homebrew:

```sh
xcode-select --install
brew install llvm
export LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"
"$LLVM_SYS_231_PREFIX/bin/llvm-config" --version
xcrun clang --version
cargo build --locked
cargo build --locked --release
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
```

The reported LLVM version must start with `23.1.`. Homebrew's moving `llvm`
formula may eventually provide another major release. Use the pinned full SDK
provisioner instead if needed (Python 3.12+):

```sh
python3 .github/scripts/provision-llvm.py --directory "$PWD/target/llvm-sdk"
export LLVM_SYS_231_PREFIX="$(cat target/llvm-sdk/prefix.txt)"
```

Do not set `DYLD_LIBRARY_PATH`: `build.rs` embeds the SDK `lib` directory as an
rpath in `cheeku`. rustc loads its own `@rpath/libLLVM.dylib`, so a global
loader path substitutes Chuchu's LLVM into rustc and crashes it (SIGSEGV in
`llvm_util::init`).

On Apple Silicon and Intel, install CMake and Ninja (`brew install cmake ninja`);
the provisioner builds LLVM 23.1.4 from checksum-verified official source as a
`libLLVM` dylib with only the host backend (AArch64 or X86). The official ARM64
SDK is not used: it ships no `libLLVM` dylib and its static archives are LTO
bitcode that Apple's linker rejects ("Unknown attribute kind"). This is a
substantial source build.

Compile and execute:

```sh
./target/release/cheeku examples/main.chuchu -o hello
./hello
# 35
python3 .github/scripts/smoke.py --compiler target/release/cheeku
```

To install the command on PATH, run `cargo install --path . --locked` with the
same LLVM environment. Missing Clang requires a working `xcrun clang`; LLVM
discovery failures require a full matching SDK prefix. A loader error requires
the corresponding LLVM shared libraries, not merely a renamed compiler binary.
