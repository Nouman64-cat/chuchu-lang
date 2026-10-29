# Phase 8 compatibility audit

Starting compiler revision: `3fe3d90` (the completed `assign` migration).

| Assumption found | Implementation response |
|---|---|
| macOS `xcrun clang`, generic `clang` everywhere else | Isolated platform driver selection; Windows clang-cl in an x64 MSVC environment |
| Linux numerical symbols linked without libm | Add `-lm` on Linux |
| Unix `write` declared with i64 result/count | Windows `_write` uses i32 result/count; Unix retains i64 on supported 64-bit hosts |
| Default/staged executable always `main`/`program` | Platform default and staging `.exe`; explicit output names remain exact |
| Existing executable publication uses Rust `fs::rename` | Retain same-filesystem replacement; never delete destination first; add replacement/failure tests |
| Native tests excluded outside macOS | Run the same suites on all three OSes with exact CRT newline expectations |
| All tests execute extensionless paths | Executable suffix helpers for actual program execution |
| Benchmarks call brew/sysctl/sw_vers/true unconditionally | Conditional metadata, unavailable measurements as null, compiled native startup control |
| SIMD inspection counts only AArch64 instructions | Also count x86 packed floating arithmetic |
| LLVM shared preference assumed usable on Windows | Existing llvm-sys rejects MSVC shared linking and falls back to native source-built static libraries |
| Windows SDK embeds an S: drive zstd path | Rebuild pinned LLVM source without optional compression/XML, retain SDK clang-cl |
| macOS SDK loader path exposes bundled libc++ | Expose only LLVM dylibs through a dedicated loader directory |
| No CI, package workflow, or OS setup guides | Pinned SDK provisioning, build/test matrix, test-gated packaging, explicit dependency requirements |

The lexer/parser/AST/type checker and checked arithmetic/array lowering are shared.
`printf` uses i64 with `%lld`, f64 with `%.15g`, and pointers with `%s`.
User-function bool parameters/results remain internal LLVM i1 values; no boolean
is passed as an unpromoted C vararg. There are no external array ABI boundaries.

Native target triple and data layout are detected for each compiler host. No
`--target` option or cross-OS code generation is introduced. On Windows, only the
x64 MSVC environment is in scope. Unix PIC is retained; COFF uses LLVM's default
relocation mode.

Rust documents `fs::rename` as replacing an existing destination, including its
Windows implementation. A locked destination can fail; the compiler returns the
error without deleting the old executable. This is not a claim of universal
filesystem atomicity or crash durability.

References: [Rust rename](https://doc.rust-lang.org/std/fs/fn.rename.html),
[Microsoft _write ABI](https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/write),
[LLVM build configuration](https://llvm.org/docs/CMake.html).
