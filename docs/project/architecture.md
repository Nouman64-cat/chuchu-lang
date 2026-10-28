---
title: "Compiler architecture"
description: "Trace Chuchu source through lexing, parsing, semantic analysis, LLVM generation, optimization, and native linking."
---

Cheeku is the CLI binary of the Rust package `chuchu`. The compiler library exposes source/file compilation APIs as well as lower-level frontend and backend modules.

```text
.chuchu source
    ↓ lexer
Tokens with byte spans
    ↓ parser
Abstract syntax tree
    ↓ type checker + symbol table
Validated program
    ↓ typed code generation
LLVM module
    ↓ target configuration + optimization
Host-native object
    ↓ Clang linker
Executable
```

## Frontend

| Module | Responsibility |
|---|---|
| `token.rs`, `lexer.rs` | Keywords, literals, symbols, newline tokens, and byte spans |
| `ast.rs`, `parser.rs` | Statements, precedence-based expressions, blocks, calls, and indexing |
| `types.rs`, `symbol_table.rs` | Type identity and lexical name lookup |
| `type_checker.rs` | Inference, signatures, conditions, arrays, scope, and return validation |

Function signatures are collected before bodies are checked. Function bodies use isolated symbol environments, so they do not capture top-level variables.

## Typed code generation

`codegen/program.rs` lowers validated statements into LLVM. Integers use `i64`, floats use `f64`, booleans use `i1`, and arrays use fixed-size LLVM array types.

Mutable locals and temporary storage are allocated in function entry blocks. Lexical binding maps track scope. `codegen/program/arrays.rs` implements array construction, value copies, element-wise loops, indexing, and ordered reduction. `runtime.rs` emits overflow/division/bounds guards and scalar output.

`codegen.rs` also retains low-level compatibility helpers from earlier compiler phases. Their raw integer addition helper does not implement source-language checked arithmetic; production compilation uses `generate_program` and the typed lowering.

## Optimization and native output

`optimization.rs` configures the native target triple/data layout and LLVM's standard O1–O3 pass pipelines. O0 skips optional IR passes.

`native.rs` emits objects and invokes Clang. On macOS it runs `xcrun clang`. Executables are staged in a temporary directory beside the destination, then renamed after successful linking.

`compiler.rs` orchestrates the pipeline, protects source/artifact paths, maps errors, and records timings. `main.rs` parses CLI options and formats diagnostics.

## Library entry points

- `compile_source` / `compile_file`: default build policy.
- `compile_source_with_options` / `compile_file_with_options`: explicit `BuildOptions` and returned timings.
- `generate_program`: LLVM generation from a semantically validated AST.
- Native emission APIs: lower-level, caller-managed artifact paths.

Refer to the [Rust source](https://github.com/chuchu-lang/chuchu-core/tree/main/src) for exact signatures. The Rust API evolves alongside the language.
