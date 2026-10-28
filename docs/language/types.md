---
title: "Types & literals"
description: "Understand Chuchu integer, floating-point, boolean, and fixed-size array types and numeric conversion rules."
---

Chuchu checks types before generating LLVM code. Variables infer their types; function signatures use lowercase `int`, `float`, and `bool` annotations.

| Type | Examples | Current backend representation |
|---|---|---|
| `int` | `0`, `42`, `-7` | Signed 64-bit integer |
| `float` | `1.0`, `3.14159`, `2e-3` | 64-bit floating point |
| `bool` | `true`, `false` | Boolean |
| Numeric array | `[1, 2]`, `[1.5, 2.5]` | Fixed-size value containing 1–256 elements |

## Integers

Integer literals use decimal digits. Negation uses unary `-`. The backend accepts values from `-9223372036854775808` through `9223372036854775807`.

Arithmetic is checked: overflowing addition, subtraction, multiplication, negation, or `abs` terminates the executable with a runtime diagnostic. Out-of-range integer literals are rejected during code generation.

This width describes the current implementation; it is not a promise about every future language version.

## Floating point

A decimal point or exponent produces a float:

```chuchu
show 1.25
show 2e3
show 2.5e-1
```

Float literals must parse as finite `f64` values. Malformed or non-finite literals such as `1e` and `1e9999` are rejected. Numeric separators such as `1_000` are unsupported.

Runtime floating-point calculations can produce infinity or NaN. For example, `sqrt(-1.0)` produces NaN. Division by positive or negative zero is checked and fails at runtime.

## Numeric widening

Mixed scalar arithmetic and comparisons convert integer operands to floats:

```chuchu
show 1 + 2.5
show 1 == 1.0
```

Output: `3.5`, then `true`. Integer-to-float conversion is also accepted for float function parameters, float return values, and reassignment to an existing float variable. Large integers can lose precision when converted to `f64`.

There is no implicit float-to-int conversion or explicit cast syntax.

## Arrays

Array type identity includes element type and length. `[1, 2]` and `[1, 2, 3]` have different types, as do `[1, 2]` and `[1.0, 2.0]`. Array elements must all have the same numeric type; arrays do not widen mixed elements.

Empty arrays, boolean arrays, nested arrays, and arrays above 256 elements are unsupported. Array annotations in function signatures are also unsupported. See [arrays](arrays.md).

## Output formatting

`show` prints one scalar followed by a newline:

- Integers use signed decimal output.
- Floats use C `printf` format `%.15g`; `2.0` can print as `2`.
- Booleans print `true` or `false`.

Strings, characters, null values, objects, and user-defined types are not currently implemented.
