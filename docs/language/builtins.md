---
title: "Built-in functions"
description: "Reference for Chuchu abs, sqrt, min, max, sum, len, and scalar output with show."
---

Built-ins are called by name in expressions. Their names cannot be used for user function declarations.

| Function | Accepted input | Result | Behavior |
|---|---|---|---|
| `abs(x)` | `int` or `float` | Same scalar type | Absolute value |
| `sqrt(x)` | `float` | `float` | Square root |
| `min(a, b)` | Numeric scalars | `float` if either input is float, otherwise `int` | Minimum |
| `max(a, b)` | Numeric scalars | `float` if either input is float, otherwise `int` | Maximum |
| `len(a)` | Numeric array | `int` | Number of elements |
| `sum(a)` | Numeric array | Scalar element type | Ordered sum |

## Examples

```chuchu
show abs(-7)
show abs(-3.5)
show sqrt(9.0)
show min(2, 3.5)
show max(2.0, 3)
show len([1, 2, 3])
show sum([1, 2, 3])
```

```text title="Output"
7
3.5
3
2
3
3
6
```

## Edge cases

- `sqrt` requires an actual float argument: `sqrt(9)` is a type error; use `sqrt(9.0)`.
- `sqrt` of a negative float produces NaN.
- Floating `min` and `max` lower to C `fmin` and `fmax`. With one NaN argument and one numeric argument, they return the numeric value.
- `abs` of the minimum signed integer fails because its positive value cannot fit in the current integer representation.
- Integer `sum` checks every addition for overflow.
- `len` still evaluates its array argument, so it does not suppress failures in an expression that constructs the array.

## Show

`show expression` is a statement, not a function call. It accepts integer, float, or boolean scalars and prints a trailing newline. See [output formatting](types.md#output-formatting).

There is no standard library module/import system, input function, or file I/O API in the current language.
