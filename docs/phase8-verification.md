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
| Linux x64 GNU | NOT VERIFIED | NOT VERIFIED | NOT VERIFIED | NOT VERIFIED |
| Windows x64 MSVC | NOT VERIFIED | NOT VERIFIED | NOT VERIFIED | NOT VERIFIED |

macOS uses installed LLVM 23.1.4 and Rust 1.99.1. Source changes are validated by
build, full tests, warnings-denied Clippy, formatting, and native release smoke
programs. Python benchmark portability/regression checks cover six tests.

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

- `Cargo.toml`: declare the existing code's Rust minimum (1.88).
- `src/native.rs`, `src/main.rs`: driver selection, default/staged filenames,
  linker diagnostics, libm/MSVC policy, safe-publication documentation.
- `src/optimization.rs`: native Windows x64 MSVC guard, COFF relocation policy.
- `src/codegen/program/runtime.rs`: platform CRT write ABI.
- `tests/support/mod.rs`, native/phase6/optimization integration tests: shared
  execution, exact newline bytes, new portability regression coverage.
- `benchmarks/run.py`, `benchmarks/test_runner.py`: portable discovery, metadata,
  executable names, startup controls, unavailable memory, x86 inspection/tests.
- `.github/scripts/`: pinned SDK provisioning, native smoke and packaging.
- `.github/workflows/`: four-target CI, shared native checks, gated draft releases.
- `.github/docker/linux.Dockerfile`: Linux x64 container validation environment.
- `README.md`, platform/build/audit/release/collaboration/report documents:
  setup, ABI decisions, runtime dependencies, ownership, limitations, evidence.
- `.gitignore`: ignore generated Python helper caches.

## Remaining limits

- Filesystem atomicity/durability is not promised across every filesystem.
- Optional IR/assembly artifacts remain nontransactional, matching Phase 7.
- Windows requires MSVC/SDK libraries and runtime; Unix distributions require
  matching LLVM shared libraries and system runtime dependencies.
- Linux musl, Windows ARM64/MinGW, 32-bit hosts and cross-OS compilation are not
  initial targets. Presence of unexecuted CI files is not a platform test pass.
- Intel macOS uses an LLVM source build because this release has no Intel SDK.
- Saved Phase 7 measurements remain historical. New portable benchmark output
  records actual host/tool versions; no cross-hardware speed claim is made.

Recommended branches and PR order are in [collaboration](phase8-collaboration.md).
No release should be published until every required native job passes.
