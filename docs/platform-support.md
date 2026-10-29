# Native platform support

Cheeku uses one lexer, parser, AST, type checker, LLVM generator, and optimization
pipeline. Native means **build and run on the same operating system**. No
cross-compilation option is provided.

| Platform | Rust host | Implementation | Validation |
|---|---|---|---|
| macOS Apple Silicon | `aarch64-apple-darwin` | xcrun clang / Mach-O | Hosted native CI passes; local native tests pass |
| macOS Intel | `x86_64-apple-darwin` | xcrun clang / Mach-O | Hosted native CI passes |
| Linux x64 GNU | `x86_64-unknown-linux-gnu` | Clang + libm / ELF | Hosted native CI passes |
| Windows x64 MSVC | `x86_64-pc-windows-msvc` | clang-cl + MSVC CRT / PE/COFF | Hosted native CI passes |

Hosted evidence: [all four native jobs at `5bbad2c`](https://github.com/chuchu-lang/chuchu-core/actions/runs/40815235497) — build, tests,
warnings-denied Clippy, release smoke, benchmark smoke and packaging.

Implementation coverage is not evidence of validation. The matrix above must be
updated only after actual native execution. Windows ARM64/MinGW, Linux musl,
32-bit targets, and cross-OS compilation are outside the initial support scope.

## Common policy

- Rust 1.88+ (existing let-chain syntax); CI pins 1.99.1.
- LLVM **23.1.x**, Inkwell 0.11.0 / llvm-sys 231.0.1. CI pins the full SDK 23.1.4.
- Inkwell's default `target-all` is disabled. Each host enables only its own LLVM
  backend (`target-x86` on x86_64, `target-aarch64` on aarch64), matching the
  single-backend SDKs. Enabling every backend makes the linker require
  initializers for backends the SDK does not contain.
- On macOS, `build.rs` embeds the LLVM `lib` directory as an rpath. Do not set
  `DYLD_LIBRARY_PATH`: rustc loads its own `@rpath/libLLVM.dylib`, and a global
  path substitutes Chuchu's LLVM into rustc and crashes it.
- O2 and generic host CPU remain defaults. All O0–O3 checks remain enabled.
- `assign` declarations and `reassign` updates are unchanged.
- Default executable: `main` on macOS/Linux, `main.exe` on Windows.
- Explicit `-o` paths are used **exactly as given**, including extensionless names.
  On Windows prefer `.exe` so shells and process launchers can execute the result.
- Clang tools must be available, along with the OS's C runtime development files.
- `CHEEKU_LINKER` optionally selects one compiler-driver executable path. It is
  never parsed as a shell command: use Clang on Unix or clang-cl on Windows, not
  link.exe/ld or a command with embedded flags.

Windows uses `_write(int, pointer, unsigned int) -> int`; supported Unix hosts use
`write(int, pointer, size_t) -> ssize_t` with 64-bit size/result. `printf` uses
`%lld` for i64 on every host. Windows CRT output uses native CRLF line endings;
portable tests check those bytes explicitly. Runtime errors still write to stderr
and exit 1 with source byte spans.

## Line endings

Windows checkouts may use CRLF. The lexer treats `\r` as whitespace and `\n` as
the statement separator, so LF and CRLF sources parse to the same program. Source
spans are byte offsets into the **original** text, so a CRLF file's offsets are
larger than the same program's LF offsets and error spans stay exact. Tests that
hardcode offsets from a checked-out fixture normalize it to LF first, and
dedicated tests cover CRLF spans, blank lines and lexical errors.

## Output safety

Objects and executables are staged beside the requested output. Linking failures
never delete the previous executable. Rust `fs::rename` replaces an existing
output on supported Unix/Windows systems without first removing it. A locked
Windows file can reject replacement; the error is returned and the old file is
retained. No universal filesystem atomicity or crash durability is promised.
Diagnostic IR/assembly files remain separately written, as in Phase 7.

Setup: [macOS](build-macos.md), [Linux](build-linux.md), [Windows](build-windows.md).
Distribution dependencies: [release runtime requirements](release-runtime.md).
