# LLVM optimization

Cheeku uses LLVM 23.1's [new pass manager](https://llvm.org/docs/NewPassManager.html)
through Inkwell 0.11.0. Source syntax and runtime checks are unchanged.

| Flag | Optional IR pipeline | Target-machine level |
|---|---|---|
| `-O0` | None | None |
| `-O1` | `default<O1>` | Less |
| `-O2` (default) | `default<O2>` | Default |
| `-O3` | `default<O3>` | Aggressive |

O0 skips optional IR optimization; LLVM still lowers instructions into machine
code. O1 promotes locals and simplifies scalar code. O2 and O3 enable LLVM loop
and SLP vectorization and loop interleaving. O3 permits more aggressive standard
transformations; it is not guaranteed to be faster or smaller.

```sh
cheeku examples/main.chuchu -O0 -o main_debug
cheeku examples/main.chuchu -o main_fast -O3
cheeku examples/numerical.chuchu -O2 --emit-ir numerical.ll --emit-assembly numerical.s -o numerical
cheeku examples/main.chuchu --time-compilation
```

Flags follow the source path; optimization and output flags can appear in either
order. Unknown flags, repeated options and unsupported optimization levels fail
before compilation. Normal successful compilation is quiet. Diagnostic files are
explicit opt-ins; IR is dumped after optimization. Compiler APIs reject artifact
paths that alias the input or each other, including canonical symlink aliases.
Executable publication still uses a temporary directory beside the destination
and renames only after successful linking. Diagnostic files themselves are not
transactionally published and can remain if later linking fails.

## Target policy and safety

The native target is initialized and its detected triple and data layout are
installed before passes run. The default CPU is `generic`, without host feature
extensions. `--cpu native` explicitly selects LLVM's detected host CPU/features;
that executable may require those features on another machine. This is host
compilation, not a cross-compilation facility. Only macOS Apple Silicon was
validated here; the default does not establish portability across operating
systems or all ARM machines. Linking retains the existing `xcrun clang` driver.
LLVM discovery remains `LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"` in the shell.

Modules are verified after generation, before and after passes, and before object
emission. Each IR pass is also configured to verify its output. Errors propagate
through structured optimization, native and compiler errors; a failed pipeline
never falls back to a lower level. Verification checks IR validity, not source
semantic equivalence; the native correctness matrix covers observable behavior.

Checked signed arithmetic, division guards, array bounds, value-copy semantics
and evaluation order remain in the production lowering. No unsafe fast-math or
unjustified `nsw`/`nuw` annotations were added. LLVM may infer flags or remove checks
when it can prove safety. Floating reductions retain their original order; no
custom fusion of checked array multiplication and reduction was introduced,
because changing which overflow happens first would change diagnostics.

`--no-vectorize` disables the loop/SLP vectorizer and interleaving options for
experiments. It is **not a guarantee of scalar machine code**: other standard LLVM
transformations can form vector operations. In particular, the measured float
array workload still contained vectors with this option. It cannot isolate a
SIMD speedup in those results.

## Library API

`optimization::BuildOptions` holds the level, CPU, vectorization policy and
optional diagnostic paths. Existing `compile_source`, `compile_file`,
`emit_object` and `build_executable` signatures remain available as default-O2
wrappers. Their `_with_options` variants return measured stage durations.
`generate_program` and the typed lowering modules remain unchanged. Optimizing
an existing `Module` mutates it in place; create a fresh module when comparing
levels. The low-level native APIs accept caller-managed artifact paths; use the
compiler APIs for source/artifact collision validation.

See [performance methodology](performance.md) and the
[benchmark analysis](benchmark-results.md).
