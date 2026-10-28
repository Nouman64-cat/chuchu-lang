---
title: "Your first program"
description: "Write, compile, and run a Chuchu program using variables, functions, arrays, and show."
---

Create a file named `hello.chuchu`:

```chuchu title="hello.chuchu"
set x to 10
set y to 20
show x + y
```

Compile it, then run the executable separately:

```bash
cheeku hello.chuchu -o hello
./hello
```

```text title="Output"
30
```

`set` introduces a variable. `to` separates its name from its value. `show` prints a scalar value followed by a newline. Each statement is separated by a newline.

:::tip Using a compiler you have not installed
Replace `cheeku` with the path to your release binary, such as `./target/release/cheeku` from the compiler repository.
:::

## Change a value

A variable keeps the type inferred from its declaration. Use `reassign` to update it:

```chuchu
set counter to 0
while counter < 3 {
    show counter
    reassign counter to counter + 1
}
```

This prints `0`, `1`, and `2` on separate lines.

## Write a function

Function parameters and return values have explicit types:

```chuchu
function square(x: int) -> int {
    return x * x
}

show square(10)
```

The output is `100`. The argument and return value are integers.

## Work with a numerical array

```chuchu
set a to [1, 2, 3]
set b to [4, 5, 6]
show sum(a * b)
```

`a * b` multiplies corresponding elements, producing `[4, 10, 18]`. `sum` adds them and prints `32`.

## What happens when you compile?

Cheeku validates source syntax and types, generates LLVM IR, optimizes it, emits an object file, and links an executable. Successful compilation is normally quiet. The default optimization level is O2.

Continue with [variables and scope](../language/variables.md), or browse [complete examples](../guides/examples.md).
