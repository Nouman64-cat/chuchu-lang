# Native platform support

Cheeku uses one lexer, parser, AST, type checker, LLVM generator, and optimization
pipeline. Native means **build and run on the same operating system**. No
cross-compilation option is provided.

| Platform | Rust host | Implementation | Validation in this change |
|---|---|---|---|
| macOS Apple Silicon | `aarch64-apple-darwin` | xcrun clang / Mach-O | Local native tests pass |
| macOS Intel | `x86_64-apple-darwin` | xcrun clang / Mach-O | NOT VERIFIED; CI job provided |
| Linux x64 GNU | `x86_64-unknown-linux-gnu` | Clang + libm / ELF | Hosted native CI passes (138 Rust tests) |
| Windows x64 MSVC | `x86_64-pc-windows-msvc` | clang-cl + MSVC CRT / PE/COFF | NOT VERIFIED; CI job provided |

Implementation coverage is not evidence of validation. The matrix above must be
updated only after actual native execution. Windows ARM64/MinGW, Linux musl,
32-bit targets, and cross-OS compilation are outside the initial support scope.

## Common policy

- Rust 1.88+ (existing let-chain syntax); CI pins 1.99.1.
- LLVM **23.1.x**, Inkwell 0.11.0 / llvm-sys 231.0.1. CI pins the full SDK 23.1.4.
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

## Output safety

Objects and executables are staged beside the requested output. Linking failures
never delete the previous executable. Rust `fs::rename` replaces an existing
output on supported Unix/Windows systems without first removing it. A locked
Windows file can reject replacement; the error is returned and the old file is
retained. No universal filesystem atomicity or crash durability is promised.
Diagnostic IR/assembly files remain separately written, as in Phase 7.

Setup: [macOS](build-macos.md), [Linux](build-linux.md), [Windows](build-windows.md).
Distribution dependencies: [release runtime requirements](release-runtime.md).
