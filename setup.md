# Setup guide

Build and run Chuchu on macOS (Apple Silicon or Intel), Linux x64, or Windows
x64 with the MSVC toolchain.

## Requirements

Install Git, [Rust through rustup](https://rustup.rs/), and Python 3.12 or newer.
The commands below use Rust 1.99.1 and LLVM 23.1.4 to match CI. The LLVM
provisioner downloads verified files and builds LLVM from source on macOS and
Windows; the first setup can take considerable time. Keep the SDK directory
for future builds.

Clone the repository:

```sh
git clone https://github.com/chuchu-lang/chuchu-core.git
cd chuchu-core
```

Run the commands for your operating system from this directory.

## macOS

Install Xcode command line tools:

```sh
xcode-select --install
```

Install [Homebrew](https://brew.sh/) if needed, then install the prerequisites:

```sh
brew install python cmake ninja
rustup toolchain install 1.99.1 --profile minimal --component rustfmt,clippy
rustup override set 1.99.1
```

Provision LLVM and configure the current terminal:

```sh
python3 .github/scripts/provision-llvm.py --directory "$PWD/target/llvm-sdk"
export LLVM_SYS_231_PREFIX="$(cat target/llvm-sdk/prefix.txt)"

"$LLVM_SYS_231_PREFIX/bin/llvm-config" --version
xcrun clang --version
```

LLVM should report `23.1.4`. Do not set `DYLD_LIBRARY_PATH`; the compiler's build
script configures its LLVM library search path.

More details: [macOS build guide](docs/build-macos.md).

## Linux

These package commands are for Ubuntu/Debian. Other distributions need equivalent
packages. Ensure `python3 --version` reports Python 3.12 or newer before running
the provisioner.

```sh
sudo apt-get update
sudo apt-get install -y \
  git build-essential clang python3 \
  libtinfo6 libxml2-dev libzstd-dev zlib1g-dev xz-utils

rustup toolchain install 1.99.1 --profile minimal --component rustfmt,clippy
rustup override set 1.99.1
```

Provision LLVM and configure the current terminal:

```sh
python3 .github/scripts/provision-llvm.py --directory "$PWD/target/llvm-sdk"
export LLVM_SYS_231_PREFIX="$(cat target/llvm-sdk/prefix.txt)"
export PATH="$LLVM_SYS_231_PREFIX/bin:$PATH"
export LD_LIBRARY_PATH="$LLVM_SYS_231_PREFIX/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

"$LLVM_SYS_231_PREFIX/bin/llvm-config" --version
clang --version
```

LLVM should report `23.1.4`. Older distributions may need a different LLVM
installation method; see the [Linux build guide](docs/build-linux.md).

## Windows

Install:

1. Git for Windows and Python 3.12 or newer.
2. Visual Studio Build Tools with **Desktop development with C++**, including
   the x64 MSVC tools and a Windows SDK.
3. Rust through rustup, selecting the **x64 MSVC** toolchain.

Open **Developer PowerShell for Visual Studio**, configured for x64, and change
to the cloned repository. Then run:

```powershell
rustup toolchain install 1.99.1-x86_64-pc-windows-msvc `
  --profile minimal --component rustfmt,clippy
rustup override set 1.99.1-x86_64-pc-windows-msvc

python -m pip install cmake ninja
python .github/scripts/provision-llvm.py --directory "$PWD/target/llvm-sdk"

$env:LLVM_SYS_231_PREFIX = (Get-Content target/llvm-sdk/prefix.txt).Trim()
$env:PATH = "$env:LLVM_SYS_231_PREFIX\bin;$env:PATH"

& "$env:LLVM_SYS_231_PREFIX\bin\llvm-config.exe" --version
clang-cl.exe --version
```

LLVM should report `23.1.4`. Use the developer shell for builds so MSVC and Windows
SDK libraries can be found. The small LLVM installer alone is insufficient for
the current compiler build.

More details: [Windows build guide](docs/build-windows.md).

## Build and test

After completing your OS setup, run:

```sh
cargo build --locked --release
cargo test --locked
```

Compile and execute an example on macOS/Linux:

```sh
./target/release/cheeku examples/main.chuchu -o target/hello
./target/hello
```

On Windows:

```powershell
.\target\release\cheeku.exe examples/main.chuchu -o target/hello.exe
.\target\hello.exe
```

Expected output:

```text
35
```

For development checks, also run:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
```

## Opening a new terminal

Environment variables above apply only to the current terminal. Reapply your OS's
`LLVM_SYS_231_PREFIX` and applicable `PATH`/`LD_LIBRARY_PATH` assignments before
building in a new terminal. On Windows, reopen Developer PowerShell first.

For troubleshooting, use the linked OS build guides. Contribution rules are in
[CONTRIBUTING.md](CONTRIBUTING.md).
