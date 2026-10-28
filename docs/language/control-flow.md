---
title: "Control flow"
description: "Write Chuchu if/else branches and while loops with boolean conditions and lexical block scope."
---

## Conditions

`if` and `while` require a `bool` expression. Numbers are not implicitly treated as true or false.

## If and else

```chuchu
set score to 85
if score >= 80 {
    show 1
} else {
    show 0
}
```

This prints `1`. The `else` block is optional. Braces are required around each branch. Put a nested `if` inside an `else` block when you need another decision; `else if` shorthand is not supported.

```chuchu
set score to 85
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
set counter to 0
while counter < 5 {
    show counter
    reassign counter to counter + 1
}
```

This prints `0` through `4`. Update the values controlling your condition so the loop can finish.

## Nested loops

```chuchu
set i to 0
set total to 0
while i < 3 {
    set j to 0
    while j < 2 {
        reassign total to total + 1
        reassign j to j + 1
    }
    reassign i to i + 1
}
show total
```

The output is `6`. The inner `j` exists only within the outer loop body.

## Returning from a function

A `return` exits the enclosing function, including from inside a loop or branch. Functions must satisfy the compiler's [return rules](functions.md#returning-a-value).

`for`, `break`, `continue`, and logical condition operators are not implemented.
