---
title: "Examples"
description: "Run complete Chuchu examples for arithmetic, recursion, numerical arrays, and nested loops."
---

Each example below is a complete program. Save it to a `.chuchu` file, then compile and run it:

```bash
cheeku example.chuchu -o example
./example
```

## Circle area

```chuchu
set pi to 3.14159
set radius to 5.0
set area to pi * radius * radius
show area
```

Output: `78.53975`.

## Factorial

```chuchu
function factorial(n: int) -> int {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}
show factorial(5)
```

Output: `120`.

## Dot product

```chuchu
set a to [1, 2, 3]
set b to [4, 5, 6]
show sum(a * b)
```

Output: `32`. Multiplication creates an element-wise product; reduction sums it.

## A typed function with widening

```chuchu
function half(x: float) -> float {
    return x / 2.0
}
show half(9)
```

Output: `4.5`. The integer argument widens to float.

## Nested iteration

```chuchu
set i to 0
set total to 0
while i < 3 {
    set j to 0
    while j < 2 {
        reassign total to total + i * j
        reassign j to j + 1
    }
    reassign i to i + 1
}
show total
```

Output: `3`.

## More examples in the repository

The [compiler examples directory](https://github.com/chuchu-lang/chuchu-core/tree/main/examples) contains arithmetic, conditions, loops, functions, floats, arrays, and a syntax smoke test. The [benchmark sources](https://github.com/chuchu-lang/chuchu-core/tree/main/benchmarks/chuchu) exercise longer numerical workloads.

For a complete list of supported constructs, see [language status](../project/status.md).
