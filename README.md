# chuchu-lang

Chuchu is a high-performance, statically typed programming language with automatic type inference, designed for simplicity and fast numerical computation, built in Rust.

Declare variables with `assign` and update them with `reassign`:

```chuchu
assign x 5
reassign x 6
show x
```

The older `assign x to 5` / `reassign x to 6` form is still accepted.

## Native build and platform status

New contributors should start with the [setup guide](setup.md).

One shared Rust/LLVM compiler targets its own host operating system. Rust 1.88+
and LLVM **23.1.x** are required; CI pins Rust 1.99.1 and the full LLVM 23.1.4 SDK.
The crate is `chuchu`, and the compiler executable is `cheeku` (`cheeku.exe` on
Windows). There is no cross-OS `--target` option.

- [macOS setup](docs/build-macos.md): Apple Silicon and Intel; LLVM is built from verified source as a shared library.
- [Linux x64 GNU setup](docs/build-linux.md): Clang/ELF and libm.
- [Windows x64 MSVC setup](docs/build-windows.md): clang-cl, source-built static LLVM, VS Build Tools and SDK.
- [Platform policy and current evidence](docs/platform-support.md).

On macOS with matching Homebrew LLVM:

```sh
export LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"
cargo build --locked --release
./target/release/cheeku examples/main.chuchu -o hello
./hello
# 35
```

On Windows, after following the SDK/developer-shell guide:

```powershell
.\target\release\cheeku.exe examples/main.chuchu -o hello.exe
.\hello.exe
# 35
```

Default output is `main` on Unix and `main.exe` on Windows. `-o` preserves the
exact filename on every OS; use `.exe` explicitly for normal Windows execution.
Source/type validation precedes LLVM generation. Compilation does not run the
program. Temporary artifacts are cleaned up; an existing executable is replaced
only after successful linking, without deleting it first. Windows locked-file
failures are reported and retain the original output. Optional IR/assembly files
are still published separately.

`show` prints int/float/bool scalars through the host CRT. Checked overflow,
division and bounds failures go to stderr with source spans and exit 1. The same
portable integration suites exercise all OSes; see their actual validation status
before claiming compatibility.

## CI and distribution

`.github/workflows/cross-platform.yml` builds/tests macOS ARM64, macOS Intel,
Linux x64 GNU, and Windows x64 MSVC. SDK downloads are pinned and SHA-256 checked;
Both macOS targets and Windows build LLVM from the same verified source release
(the official ARM64 macOS SDK ships only bitcode archives that Apple's linker
rejects); built SDKs are cached per platform. Every job runs native programs and
packages only after its tests pass. The tag-triggered release workflow
requires all four jobs and creates a draft, not an unreviewed public release.

Archives require external toolchains/runtimes; see
[release requirements](docs/release-runtime.md). [Contributing](CONTRIBUTING.md)
describes the development workflow and required checks. All four targets pass
[hosted native CI](https://github.com/chuchu-lang/chuchu-core/actions/runs/40815235497); no release has been published yet.

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
[benchmark results and limitations](docs/benchmark-results.md).
