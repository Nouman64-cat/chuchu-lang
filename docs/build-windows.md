# Build on Windows x64 MSVC

**Implementation and CI configuration are provided; Windows execution is not yet
locally verified.** Only `x86_64-pc-windows-msvc` is in the initial scope.

Install:

1. Rust 1.88+ through [rustup](https://rustup.rs/) (CI pins 1.99.1), selecting
   the x64 MSVC host toolchain.
2. Visual Studio 2022 Build Tools or Visual Studio with **Desktop development
   with C++**, including x64 libraries and a Windows SDK.
3. Python 3.12+ and the **full LLVM 23.1.4 Windows SDK**, including llvm-config.exe,
   headers, static `.lib` libraries, and clang-cl.exe. The small LLVM installer
   alone is insufficient for building the Rust compiler.

Open **Developer PowerShell for VS 2022**, configured for x64. Provision the full
checksum-verified SDK and build:

```powershell
python .github/scripts/provision-llvm.py --directory "$PWD/target/llvm-sdk"
$env:LLVM_SYS_231_PREFIX = (Get-Content target/llvm-sdk/prefix.txt).Trim()
$env:PATH = "$env:LLVM_SYS_231_PREFIX\bin;$env:PATH"
& "$env:LLVM_SYS_231_PREFIX\bin\llvm-config.exe" --version
clang-cl.exe --version
cargo build --locked
cargo build --locked --release
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
.\target\release\cheeku.exe examples/main.chuchu -o hello.exe
.\hello.exe
# 35
python .github/scripts/smoke.py --compiler target/release/cheeku.exe
```

Without `-o`, output is `main.exe`. Explicit names are never modified: `-o hello`
creates exactly `hello`, while `-o hello.exe` creates `hello.exe`. Use the latter
for normal PowerShell/process execution. Spaces and drive paths are passed as
separate process arguments; quote paths in PowerShell as usual.

## Linking and runtime

The compiler uses clang-cl's MSVC linker mode, x64 console subsystem, dynamic
MSVC CRT policy (`/MD`), `msvcrt.lib`, and `legacy_stdio_definitions.lib` for
LLVM-emitted printf calls. Visual Studio/SDK libraries must be discoverable
through the developer environment. The generated PE/COFF object uses the
host's native LLVM data layout. `_write` uses 32-bit count/result types.

The current llvm-sys dependency explicitly rejects dynamic LLVM linking on
MSVC. Its existing prefer-dynamic policy falls back to static LLVM libraries;
this is why the full SDK is required. No LLVM version downgrade is introduced.
The compiler and generated executables still require the compatible Microsoft
Visual C++ runtime; this is not a promise of a dependency-free executable.

## Troubleshooting

- Missing `link.exe`, SDK libraries, or UCRT: use an x64 developer shell and
  install the C++/Windows SDK workloads.
- Missing llvm-config or LLVM .lib files: use the full SDK, not only clang's
  installer, and confirm `LLVM_SYS_231_PREFIX` points to its root.
- Cannot replace output: stop the running generated program; Windows can lock
  executable files. The compiler returns the error without deleting the old file.
- A configured `CHEEKU_LINKER` must point to a single `clang-cl.exe` path, without
  flags or shell quoting in the environment variable itself.
