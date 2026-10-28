---
title: "Arrays"
description: "Create fixed-size numerical arrays in Chuchu, index safely, copy values, and compute element-wise products and reductions."
---

Arrays are fixed-size numerical values. A literal contains **1–256 elements**, all integers or all floats:

```chuchu
set numbers to [10, 20, 30, 40]
set weights to [1.5, 2.5, 3.5]
show numbers[2]
show weights[1]
```

Output: `30`, `2.5`.

## Indexing

Indices are zero-based integers. Negative indices and indices greater than or equal to the length fail at runtime with an array bounds diagnostic.

```chuchu
set numbers to [10, 20, 30]
set index to 1
show numbers[index]
```

There is no element assignment syntax such as `numbers[0] = 5`. Use whole-array reassignment to replace values.

## Value copies

Assigning an array copies its value. Changing one variable does not change another:

```chuchu
set a to [1, 2]
set b to a
reassign a to [3, 4]
show b[0]
show a[0]
```

Output: `1`, `3`. Reassignment requires the original array's element type and length.

## Element-wise arithmetic

`+` adds corresponding elements; `*` multiplies them. Both arrays must have identical types and lengths.

```chuchu
set a to [1, 2, 3]
set b to [4, 5, 6]
set product to a * b
show sum(product)
show sum(a + b)
```

Output: `32`, `21`. Integer operations are checked element by element.

## Length and reduction

`len(array)` returns its length as an integer. `sum(array)` adds elements in order and returns the scalar element type.

```chuchu
set values to [1.5, 2.5, 3.5]
show len(values)
show sum(values)
```

Output: `3`, `7.5`.

:::note Printing arrays
`show` accepts scalars, so `show values` is a type error. Print an indexed element, `len(values)`, or `sum(values)` instead.
:::

## Current limits

Empty, mixed numeric, boolean, and nested arrays are rejected. Arrays cannot grow, cannot broadcast against scalars, and cannot be annotated in function signatures. The 256-element cap is enforced by the checker and code generator.

The current compiler stores arrays and temporaries in function-entry stack allocations. LLVM optimization may simplify or eliminate them; this is an implementation detail, not a heap allocation API.
