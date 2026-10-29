---
title: "Control flow"
description: "Write Chuchu if/else branches and while loops with boolean conditions and lexical block scope."
---

## Conditions

`if` and `while` require a `bool` expression. Numbers are not implicitly treated as true or false.

## If and else

```chuchu
assign score 85
if score >= 80 {
    show 1
} else {
    show 0
}
```

This prints `1`. The `else` block is optional. Braces are required around each branch. Put a nested `if` inside an `else` block when you need another decision; `else if` shorthand is not supported.

```chuchu
assign score 85
if score >= 90 {
    show 3
} else {
    if score >= 80 {
        show 2
    } else {
        show 1
    }
}
```

## While loops

A `while` loop evaluates its condition before each iteration:

```chuchu
assign counter 0
while counter < 5 {
    show counter
    reassign counter counter + 1
}
```

This prints `0` through `4`. Update the values controlling your condition so the loop can finish.

## Nested loops

```chuchu
assign i 0
assign total 0
while i < 3 {
    assign j 0
    while j < 2 {
        reassign total total + 1
        reassign j j + 1
    }
    reassign i i + 1
}
show total
```

The output is `6`. The inner `j` exists only within the outer loop body.

## Returning from a function

A `return` exits the enclosing function, including from inside a loop or branch. Functions must satisfy the compiler's [return rules](functions.md#returning-a-value).

`for`, `break`, `continue`, and logical condition operators are not implemented.
