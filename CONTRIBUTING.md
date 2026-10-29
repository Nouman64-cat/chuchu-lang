# Contributing to Chuchu

Use one shared compiler implementation for all supported operating systems.
Keep platform-specific linking and runtime code isolated, and preserve language
semantics and runtime safety checks when making portability changes.

## Development setup

Start with the [setup guide](setup.md) to install prerequisites, build the
compiler, and run an example on your operating system.

Follow the [macOS](docs/build-macos.md), [Linux](docs/build-linux.md), or
[Windows](docs/build-windows.md) guide for Rust, LLVM and linker prerequisites.
The [platform support guide](docs/platform-support.md) describes the supported
hosts, runtime ABI and executable output policy.

Before opening a pull request, run:

```sh
cargo fmt --all -- --check
cargo build --locked
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
python3 -m unittest discover -s benchmarks -p test_runner.py
python3 -m unittest discover -s .github/scripts -p 'test_*.py'
python3 .github/scripts/smoke.py --compiler target/release/cheeku
```

On Windows, use `python` and `target/release/cheeku.exe` for the Python commands.
Tests must compile and execute native programs, checking output and error
behavior. Preserve the O0–O3 behavior matrix and add regression coverage for
compiler or portability fixes. Performance methodology is documented in
[performance measurement](docs/performance.md).

## Pull requests and review

Create short-lived feature branches and keep pull requests focused. Coordinate
changes to shared interfaces such as `link_object`, executable output handling
and runtime function signatures before overlapping work. Review shared compiler
and runtime changes across the supported platforms rather than maintaining
separate operating-system branches.

Require all four native target checks for `main` branch protection. The
`Cross-platform native compiler` workflow runs on pushes and pull requests;
its jobs cover macOS ARM64, macOS Intel, Linux x64 GNU and Windows x64 MSVC.
Do not bypass failing jobs or treat one host's results as validation of another.

## SDK caches and releases

LLVM provisioning and release packaging are maintained in `.github/scripts`.
SDK cache recipe identifiers are selected in `.github/workflows/native-build.yml`.
Update a platform's recipe identifier when its SDK contents change, and never
reuse a cache built with a different recipe. Documentation-only changes do not
require rebuilding LLVM.

Tag a release only after the revision passes every required native check. The
release workflow rebuilds and retests each target before creating draft archives.
Review the [runtime requirements](docs/release-runtime.md) before publishing.
