---
title: "Optimization"
description: "Choose Cheeku O0\u2013O3 optimization levels while preserving checked arithmetic and floating-point semantics."
---

Cheeku uses LLVM's standard optimization pipelines. The default is **O2**, with a generic CPU for the detected host architecture.

| Level | IR pipeline | Intended use |
|---|---|---|
| `-O0` | No optional IR passes | Inspect initial lowering and establish a baseline |
| `-O1` | `default<O1>` | Scalar optimization and local promotion |
| `-O2` | `default<O2>` | General optimized builds |
| `-O3` | `default<O3>` | More aggressive standard optimization |

```bash
cheeku program.chuchu -O0 -o program_debug
cheeku program.chuchu -O3 -o program_fast
```

O0 still lowers LLVM instructions into machine code. O3 is not guaranteed to produce faster or smaller executables than O2.

## What is preserved

Checked integer arithmetic, division guards, bounds checks, array value semantics, and floating reduction order remain part of production lowering. Cheeku does not enable unsafe fast-math transformations.

LLVM can eliminate a check or unused computation when it can prove the transformation safe. The compiler verifies modules before and after optimization and enables per-pass verification. Verification establishes IR validity; behavior is separately exercised by native tests.

## CPU policy

`--cpu generic` uses the detected host architecture without enabling host feature extensions. `--cpu native` enables LLVM's detected host CPU and features:

```bash
cheeku program.chuchu -O3 --cpu native -o program
```

An executable built with native features may require those features on another machine. Neither setting is a cross-compilation facility or a guarantee of operating-system portability.

## Vectorization

O2 and O3 enable loop and SLP vectorization plus loop interleaving. `--no-vectorize` disables those options for investigation, but other LLVM transformations can still form vector operations. It does **not** guarantee scalar assembly.

## Inspect before drawing conclusions

```bash
cheeku program.chuchu -O2 --emit-ir program.ll --emit-assembly program.s -o program
```

Static inputs may be folded or simplified. A smaller IR file is not automatically a faster program, and finding vector instructions does not isolate their runtime benefit. See [benchmark methodology](performance.md).
