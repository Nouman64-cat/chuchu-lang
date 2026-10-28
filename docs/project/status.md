---
title: "Language status"
description: "See implemented Chuchu features, current limitations, platform support, and documentation version policy."
---

These docs describe the **0.1 development codebase**. The package reports `0.1.0`; this does not imply a published binary distribution or a stable language specification.

## Implemented

- Inferred local variables and typed reassignment.
- Signed integers, floats, booleans, and homogeneous fixed-size numeric arrays.
- Arithmetic, comparisons, grouping, and unary negation.
- `if` / `else`, `while`, lexical scope, and block shadowing.
- Top-level typed functions, forward calls, recursion, and mutual recursion.
- Numerical built-ins: `abs`, `sqrt`, `min`, `max`, `sum`, `len`.
- Scalar output through `show`.
- Native object emission and executable linking.
- O0–O3 optimization, CPU selection, timing JSON, IR and assembly output.
- Runtime checks for integer arithmetic, division, and array bounds.

## Current limits

| Area | Status |
|---|---|
| Platforms | Validated on macOS Apple Silicon; Linux/Windows unvalidated |
| Arrays | 1–256 homogeneous numeric elements; no element assignment |
| Function signatures | Scalar parameter and return annotations only |
| Text | No strings, characters, or comments |
| Control flow | No `for`, `break`, `continue`, or logical operators |
| Modules and data | No imports, user-defined types, dynamic collections, or file I/O API |
| Runtime | No language-level input API, JIT, GPU execution, or automatic parallelism |
| Tooling | No official language server or browser playground in this codebase |

These are implementation boundaries, not announced roadmap commitments.

## Documentation versions

The site currently documents one development line. We will only need separate versioned docs when released language versions have differences worth maintaining. Docusaurus supports snapshots without requiring them from the start.

When you encounter a mismatch, check your compiler checkout and open an issue in the [compiler repository](https://github.com/chuchu-lang/chuchu-core/issues) or [documentation repository](https://github.com/chuchu-lang/chuchu-lang-docs/issues).
