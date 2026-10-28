---
title: "Compiler CLI"
description: "Reference every Cheeku compiler flag: optimization, output paths, CPU policy, timings, LLVM IR, and assembly."
---

```text
cheeku <source.chuchu> [-O0|-O1|-O2|-O3] [-o <executable>] [--cpu generic|native] [--time-compilation] [--emit-ir <file>] [--emit-assembly <file>] [--no-vectorize]
```

The input must have a `.chuchu` extension. Options follow the source path; output and optimization flags can appear in either order. Unknown flags and repeated options are errors.

| Option | Default | Purpose |
|---|---|---|
| `-o <path>` | `main` in the working directory | Executable destination |
| `-O0`, `-O1`, `-O2`, `-O3` | `-O2` | Optimization level |
| `--cpu generic` | Selected | Generic CPU for detected host architecture |
| `--cpu native` | Off | Host-specific CPU/features |
| `--time-compilation` | Off | Stage timing JSON on stderr |
| `--emit-ir <path>` | Off | LLVM IR after optional optimization |
| `--emit-assembly <path>` | Off | Target assembly |
| `--no-vectorize` | Off | Disable loop/SLP vectorizer and interleaving options |
| `--help`, `-h` | — | Print usage when passed alone |

## Compile and run

```bash
cheeku hello.chuchu -O2 -o hello
./hello
```

Compilation does not execute the program. Paths containing spaces are supported when quoted in your shell:

```bash
cheeku "my source.chuchu" -o "my program"
./"my program"
```

## Inspect generated code

```bash
cheeku numerical.chuchu -O3 --emit-ir numerical.ll --emit-assembly numerical.s -o numerical
```

Artifact paths must differ from the input and each other. Existing source/artifact aliases are checked using path normalization and canonicalization.

## Compilation timing

```bash
cheeku hello.chuchu --time-compilation -o hello
```

One JSON object is written to stderr after success. Values are measured seconds:

| Key | Stage |
|---|---|
| `lex_s` | Tokenization |
| `parse_s` | Parsing |
| `type_s` | Type checking |
| `ir_s` | LLVM generation and its verification |
| `target_setup_s` | Native target configuration |
| `optimization_s` | Optional passes and verification |
| `object_s` | Final verification and object emission/write |
| `link_s` | System linker invocation |
| `total_s` | Compiler API work |

Total time need not equal the sum of stages. CLI process startup and timing output are outside API total. See [performance](performance.md) for measurement details.

## Exit behavior

`--help` and successful compilation return status `0`. Argument or compilation failures return a failure status and print a diagnostic. Generated runtime check failures separately return status `1` when the executable runs.
