---
title: "Errors & diagnostics"
description: "Understand Chuchu compile-time errors, byte spans, runtime overflow, division by zero, and array bounds diagnostics."
---

Cheeku reports compile-time errors to stderr and exits unsuccessfully. Successful compilation is normally quiet.

## Source locations

Diagnostics use zero-based UTF-8 **byte offsets**, displayed as `start..end`. The start is inclusive and the end is exclusive. These are not line and column numbers.

```chuchu
show missing
```

The identifier occupies bytes `5..12`. A diagnostic points at that range. Runtime messages also include a source byte range.

## Common compile-time errors

| Problem | Example | How to fix it |
|---|---|---|
| Undefined variable | `show missing` | Declare it first, or pass it as a function argument. |
| Duplicate declaration | Two `assign x ...` statements in one scope | Use `reassign`, or choose another name. |
| Invalid condition | `if 1 { show 1 }` | Use a boolean comparison such as `1 > 0`. |
| Type mismatch | Reassigning `2.5` to an integer variable | Preserve the declared type. |
| Invalid array | `[1, 2.0]` | Use homogeneous integer or float elements. |
| Missing return | A function without a value on all required paths | Return from both branches or add a fallback return. |
| Invalid source character | A string, comment marker, or unsupported symbol | Use the current language syntax. |
| Statement separator | `show 1 show 2` | Place each statement on its own line. |

These examples illustrate invalid code; they are not runnable programs.

## Runtime failures

A program can compile successfully but fail when executed. Checked failures write a message beginning with `cheeku runtime:` to stderr and exit with status `1`.

```chuchu
assign divisor 0
show 1 / divisor
```

This compiles, but running it reports division by zero. Other checked failures include:

- Signed integer addition, subtraction, multiplication, negation, and absolute-value overflow.
- Minimum integer divided by `-1`.
- Floating division by positive or negative zero.
- Negative or out-of-range array indices.
- Overflow during integer array arithmetic or reduction.

Checks remain active at O0, O1, O2, and O3.

## Build and linker errors

Compiler setup errors are separate from source errors. An LLVM discovery failure occurs while building Cheeku; a missing Clang linker prevents producing native executables. See [installation troubleshooting](../getting-started/installation.md#troubleshooting-installation).

The compiler stages its executable and replaces the requested output only after linking succeeds. Optional IR and assembly files are written separately and can remain after a later failure. Input/artifact collisions are rejected by the compiler APIs.
