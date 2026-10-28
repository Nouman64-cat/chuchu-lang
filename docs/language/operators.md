---
title: "Operators & expressions"
description: "Use Chuchu arithmetic, comparisons, precedence, grouping, and checked integer division."
---

## Operator precedence

From highest to lowest:

| Priority | Operators | Meaning |
|---|---|---|
| Highest | `f(...)`, `a[...]` | Named function call, array indexing |
| Unary | `-x` | Numeric negation |
| Multiplicative | `*`, `/` | Multiplication, division |
| Additive | `+`, `-` | Addition, subtraction |
| Comparison | `==`, `!=`, `<`, `<=`, `>`, `>=` | Comparison returning `bool` |

Binary operators at the same priority group left to right. Parentheses override precedence.

```chuchu
show 10 + 20 * 2
show (10 + 20) * 2
show 20 - 3 - 2
```

Output: `50`, `60`, `15`.

## Arithmetic

Scalar arithmetic accepts numbers. Mixed integer/float operands produce a float. Two integer operands produce an integer, including division:

```chuchu
show 5 / 2
show -7 / 2
show 5 / 2.0
```

Output: `2`, `-3`, `2.5`. Integer division truncates toward zero.

Integer overflow and division by zero are runtime errors. Dividing the minimum signed integer by `-1` also fails. Optimization levels preserve these checks.

## Comparisons

Numeric comparisons accept mixed integer/float operands and return a boolean. Booleans support only `==` and `!=`.

```chuchu
show 4 >= 4
show true == false
show 2 < 3.5
```

Output: `true`, `false`, `true`.

NaN follows ordered floating comparisons: equality and ordered comparisons with NaN are false, while `!=` with NaN is true. Comparison operators share a precedence level; use separate expressions rather than mathematical comparison chains such as `1 < x < 10`.

## Array operators

`+` and `*` operate element by element on arrays with identical numeric element types and lengths. Arrays do not support subtraction, division, comparison, or scalar broadcasting.

## Evaluation order

The current compiler lowers binary operands and function arguments from left to right. Array products are computed before a surrounding `sum` reduction, preserving runtime error order.

Logical `and`/`or`/`not`, modulo, exponentiation, and compound assignment are not implemented.
