---
title: "Introduction"
description: "Learn what Chuchu is, how its native compiler works, and where to start."
---

Chuchu is a small, statically typed programming language built in Rust. Its compiler, **Cheeku**, turns `.chuchu` source files into native executables through LLVM.

The language combines readable statements with inferred local types, explicit function signatures, and checked numerical operations.

```chuchu
assign greeting_number to 42
show greeting_number
```

```text title="Output"
42
```

## A small vocabulary

| You want to… | Write… |
|---|---|
| Declare a variable | `assign count to 0` |
| Change its value | `reassign count to count + 1` |
| Print a value | `show count` |
| Make a decision | `if count > 0 { show count }` |
| Repeat work | `while count < 5 { reassign count to count + 1 }` |
| Define a function | `function double(x: int) -> int { return x * 2 }` |

## Choose a path

- **New to Chuchu?** [Install Cheeku](getting-started/installation.md), then [write your first program](getting-started/first-program.md).
- **Looking up syntax?** Start with [variables](language/variables.md), [types](language/types.md), or [functions](language/functions.md).
- **Working with numbers?** Explore [arrays](language/arrays.md) and [built-in functions](language/builtins.md).
- **Interested in the compiler?** Read about [architecture](project/architecture.md) and [optimization](tooling/optimization.md).

:::info Early development
These docs describe the current 0.1 codebase. Chuchu is evolving, and syntax and APIs may change. The validated native environment is macOS on Apple Silicon. See [language status](project/status.md) for supported features and limits.
:::

## Chuchu and Cheeku

**Chuchu** is the language and the Rust library/package name (`chuchu`). **Cheeku** is the command-line compiler (`cheeku`). Compiling a program creates an executable; it does not run the program automatically.
