---
title: "Functions"
description: "Define typed Chuchu functions, call forward declarations, use recursion, and return values on every path."
---

## Define and call a function

```chuchu
function add(x: int, y: int) -> int {
    return x + y
}

show add(10, 20)
```

The output is `30`. Every parameter requires a type annotation, and every function declares its return type after `->`. Supported annotations are `int`, `float`, and `bool`.

Calls require the declared number of arguments. A float parameter accepts an integer argument through numeric widening. There are no default parameters, overloads, or variadic user functions.

Function calls are expressions. To observe a result, use it in a declaration, reassignment, `show`, condition, return, or another expression. Standalone expression statements such as `add(1, 2)` are not supported.

## Forward calls and recursion

Top-level signatures are registered before bodies are checked, so a function can be called before its declaration. Direct recursion and mutual recursion are supported.

```chuchu
show factorial(5)

function factorial(n: int) -> int {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}
```

The output is `120`. Large factorials can exceed the checked integer range.

## Returning a value

Every function returns a value. A return expression must match the declared type, allowing integer-to-float widening.

The checker accepts a direct return or an `if` whose two branches both return, including returns later in the body. It does not prove that a loop always runs or that a condition is always true. Include a fallback return after a loop when needed:

```chuchu
function first(n: int) -> int {
    while n > 0 {
        return n
    }
    return 0
}
show first(3)
```

`return` is valid only inside a function. There is no `void` return type or bare `return`.

## Function scope

Functions must be declared at the top level. They cannot capture top-level variables or be nested. Parameters and local declarations belong to the function's own scope.

Arrays can be local variables inside functions, but array parameter and return annotations are not currently supported.

## Function names

Duplicate function names are rejected. Built-in and runtime names are reserved for function declarations:

`abs`, `sqrt`, `min`, `max`, `sum`, `len`, `main`, `printf`, `write`, `exit`, `fmin`, `fmax`, and `fabs`.

Functions are called by name; first-class function values and closures are not implemented.
