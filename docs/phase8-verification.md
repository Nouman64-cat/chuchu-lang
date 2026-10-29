# Phase 8 verification report

**PHASE 8: PARTIALLY COMPLETE** — implementation is present; native validation
must pass on every required target before this status can become complete.

## Implementation

Shared compiler semantics remain unchanged. Native linking is isolated in
`src/native.rs`: Apple xcrun clang, Linux Clang with libm, and Windows x64
clang-cl/MSVC. Runtime write signatures match Unix and MSVC. Default outputs and
staging are platform-aware, and explicit output paths are preserved. Existing
executables are never deleted before publishing a successfully linked result.

The original native suites now run on all OSes, including the 22-case O0–O3
behavior matrix. Additional tests cover object format, runtime ABI, replacements,
missing linkers, extensionless outputs, invalid directories, and Windows file
locking. No language feature or optimization default was removed.

## Evidence (updated as verification completes)

| Host | Build | Unit tests | Integration/native execution | Hosted CI |
|---|---|---|---|---|
| macOS ARM64 | PASS | PASS | PASS | NOT VERIFIED |
| macOS Intel | NOT VERIFIED | NOT VERIFIED | NOT VERIFIED | NOT VERIFIED |
| Linux x64 GNU | PASS | PASS | PASS | PASS |
| Windows x64 MSVC | NOT VERIFIED | NOT VERIFIED | NOT VERIFIED | NOT VERIFIED |

macOS uses installed LLVM 23.1.4 and Rust 1.99.1. Source changes are validated by
build, full tests, warnings-denied Clippy, formatting, and native release smoke
programs. Python benchmark portability/regression checks cover six tests.

Linux hosted evidence: [native job at 4556aec](https://github.com/chuchu-lang/chuchu-core/actions/runs/40799074277/job/123202789214):
138 Rust tests, debug/release builds, formatting, warnings-denied Clippy, six
Python tests, 12 release example executions, benchmark smoke and packaging pass
with LLVM 23.1.4 / Rust 1.99.1 on the x64 GNU runner.

## Commands

Configure `LLVM_SYS_231_PREFIX` following the OS guide, then:

```text
cargo build --locked
cargo build --locked --release
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
python -m unittest discover -s benchmarks -p test_runner.py
```

Release smoke: `python .github/scripts/smoke.py --compiler target/release/cheeku`
on Unix, or use `target/release/cheeku.exe` on Windows. See
[macOS](build-macos.md), [Linux](build-linux.md), and [Windows](build-windows.md)
for exact platform setup and execution commands.

## Files

| File | Change |
|---|---|
| `Cargo.toml` | Declare Rust 1.88 minimum; existing LLVM/Inkwell policy retained. |
| `src/native.rs` | Platform linker drivers, libm/MSVC arguments, staging names and publication policy. |
| `src/main.rs` | Platform default output and CLI regression expectation. |
| `src/optimization.rs` | Windows x64 MSVC guard and native COFF relocation policy. |
| `src/codegen/program/runtime.rs` | Correct Unix write / MSVC _write ABI. |
| `tests/support/mod.rs` | Shared executable suffix and exact native newline helpers. |
| `tests/native_integration.rs` | Portable real execution, object format, ABI, paths, replacements and Windows locking. |
| `tests/phase6_native.rs` | Run existing language/runtime native tests on every OS. |
| `tests/optimization_integration.rs` | Run exact O0–O3 semantics matrix and compiler artifact checks on every OS. |
| `benchmarks/run.py` | Portable tool discovery, metadata, startup controls, suffixes and unavailable memory. |
| `benchmarks/test_runner.py` | Six benchmark portability and correctness regression tests. |
| `.github/scripts/provision-llvm.py` | Checksum-pinned SDK/source provisioning and platform loader configuration. |
| `.github/scripts/test_provision.py` | Regression test preventing SDK libc++ from overriding the macOS system runtime. |
| `.github/scripts/smoke.py` | Execute three representative release examples at all four optimization levels. |
| `.github/scripts/package-release.py` | Host-checked native archive labels, license, examples and dependency requirements. |
| `.github/workflows/cross-platform.yml` | Four native target jobs on pushes and PRs. |
| `.github/workflows/native-build.yml` | Shared build, tests, Clippy, smoke, benchmark, cache and packaging checks. |
| `.github/workflows/release.yml` | Draft release only after every native target succeeds. |
| `.github/docker/linux.Dockerfile` | Additional Ubuntu x64 container verification environment. |
| `.gitignore` | Ignore generated Python helper caches. |
| `README.md` | Native platform policy and guide links. |
| `docs/platform-support.md` | Target matrix, ABI, exact output names and replacement guarantees. |
| `docs/build-macos.md` | Apple Silicon/Intel setup, native commands and SDK loader guidance. |
| `docs/build-linux.md` | GNU/Clang/libm setup, commands and Docker verification. |
| `docs/build-windows.md` | MSVC/SDK/source LLVM prerequisites, PowerShell commands and CRT behavior. |
| `docs/phase8-audit.md` | Original portability assumptions and corresponding decisions. |
| `docs/phase8-collaboration.md` | Three-developer ownership, temporary branches and integration order. |
| `docs/release-runtime.md` | Archive labels and unbundled runtime/toolchain dependencies. |
| `docs/phase8-verification.md` | This implementation and evidence report. |

`Cargo.lock`, the frontend, array/arithmetic semantics, and examples are unchanged
from the completed `assign` migration at `3fe3d90`.

## Remaining limits

- Filesystem atomicity/durability is not promised across every filesystem.
- Optional IR/assembly artifacts remain nontransactional, matching Phase 7.
- Windows requires MSVC/SDK libraries and runtime; Unix distributions require
  matching LLVM shared libraries and system runtime dependencies.
- Linux musl, Windows ARM64/MinGW, 32-bit hosts and cross-OS compilation are not
  initial targets. Presence of unexecuted CI files is not a platform test pass.
- macOS uses an LLVM source build (X86 on Intel, AArch64 on Apple Silicon): the
  release has no Intel SDK, and its ARM64 SDK has only bitcode static archives.
- `inkwell` default features are disabled; only the host LLVM backend feature
  (`target-x86` / `target-aarch64`) is enabled, matching the single-backend SDKs.
- SDK cache keys: Linux, macOS Intel and Windows stay pinned to their existing
  recipe ids (caches stay valid); macOS ARM64 uses a new `source-dylib-<hash>` key.
- Saved Phase 7 measurements remain historical. New portable benchmark output
  records actual host/tool versions; no cross-hardware speed claim is made.

Recommended branches and PR order are in [collaboration](phase8-collaboration.md).
No release should be published until every required native job passes.
