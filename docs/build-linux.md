# Build on Linux x64 GNU

Initial target: `x86_64-unknown-linux-gnu`, with Rust 1.88+ (CI pins 1.99.1),
LLVM 23.1.x, Clang, a GNU linker, and C runtime development files. Hosted CI validates
Ubuntu x64; the existence of this guide alone does not assert validation on every
distribution. See [platform support](platform-support.md) for validation evidence.

On Ubuntu, install prerequisites:

```sh
sudo apt-get update
sudo apt-get install -y build-essential clang python3 libtinfo6 libxml2-dev libzstd-dev zlib1g-dev xz-utils
```

Install Rust using [rustup](https://rustup.rs/), with the x64 GNU host toolchain.
For an exact LLVM 23.1.4 SDK, use Python 3.12+ and the checked-in provisioner:

```sh
python3 .github/scripts/provision-llvm.py --directory "$PWD/target/llvm-sdk"
export LLVM_SYS_231_PREFIX="$(cat target/llvm-sdk/prefix.txt)"
export PATH="$LLVM_SYS_231_PREFIX/bin:$PATH"
export LD_LIBRARY_PATH="$LLVM_SYS_231_PREFIX/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
"$LLVM_SYS_231_PREFIX/bin/llvm-config" --version
cargo build --locked
cargo build --locked --release
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
./target/release/cheeku examples/main.chuchu -o hello
./hello
# 35
python3 .github/scripts/smoke.py --compiler target/release/cheeku
```

Alternatively install `llvm-23-dev` and `clang-23` from the official
[apt.llvm.org](https://apt.llvm.org/) repository for your distribution, set the
prefix so that its `bin` directory contains `llvm-config-23` (Ubuntu: `/usr`),
and ensure its version is 23.1.x. That package
repository is rolling; it is not a reproducible exact-patch release source.
The included Ubuntu container uses this route and records the actual version.

Linux native linking includes `-lm`, which supplies sqrt/fabs/fmin/fmax. An
unresolved math symbol usually means a missing/incompatible C toolchain or a
custom driver that does not support the required options. A missing LLVM .so
at compiler startup requires the matching library installation and loader path.
The official SDK may require a recent glibc; older distributions may need an
LLVM source build. musl is not an initial supported configuration.

## Container verification

From the compiler repository:

```sh
docker build --platform linux/amd64 -t chuchu-linux-dev - < .github/docker/linux.Dockerfile
docker run --rm --platform linux/amd64 -v "$PWD:/workspace:ro" -e CARGO_TARGET_DIR=/tmp/chuchu-target chuchu-linux-dev cargo test --locked
```

This executes x64 Linux binaries under Docker's emulation on an ARM Mac. It
validates OS/object/runtime behavior, not native-hardware performance. The
container is a verification environment, not a released compiler distribution.
