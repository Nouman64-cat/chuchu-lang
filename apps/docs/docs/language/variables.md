---
title: "Variables & scope"
description: "Declare and reassign Chuchu variables, understand lexical scope, and write valid identifiers."
---

## Declare a variable

Use `assign` for new declarations. The earlier `set x to 5` spelling is no longer accepted. To update a declared variable, continue to use `reassign x to 6`.

Use `assign name to expression`:

```chuchu
assign count to 10
assign ratio to 2.5
assign ready to true
```

The initializer determines the variable's type. Local declarations do not use type annotations. A variable must be declared before it is read.

## Reassign a value

```chuchu
assign count to 10
reassign count to count + 5
show count
```

The output is `15`. Reassignment must preserve the declared type, with one widening conversion: an `int` can be assigned to an existing `float` variable.

```chuchu
assign measurement to 1.0
reassign measurement to 2
show measurement
```

Assigning a float to an integer variable is a type error. Declaring the same name twice in the same scope is also an error; use `reassign` to update an existing variable.

## Lexical scope

Each `if` branch and `while` body introduces a scope. A block can read and reassign an enclosing variable. A new declaration can shadow its name:

```chuchu
assign x to 10
if true {
    assign x to x + 20
    show x
    reassign x to 40
}
show x
```

This prints `30`, then `10`. The inner initializer reads the outer `x` before the inner binding is introduced. The inner reassignment changes only the inner binding.

Names declared inside a block are unavailable outside that block. Functions have their own scope and cannot capture top-level variables; pass values as arguments instead.

## Identifiers and reserved words

Names start with an ASCII letter or underscore and continue with ASCII letters, digits, or underscores. Names are case-sensitive.

Examples: `total`, `total_1`, `_value`, `squarePlus`.

The lexer reserves `assign`, `to`, `show`, `true`, `false`, `if`, `else`, `while`, `function`, `reassign`, and `return`. Function names have [additional restrictions](functions.md#function-names).

## Whitespace and statements

Separate statements with newlines. Braces delimit blocks; indentation is for readability. A single statement may appear inline inside braces:

```chuchu
if true { show 42 }
```

Semicolons and comments are not currently supported. Do not include `//` or `#` comment lines in `.chuchu` source files.
