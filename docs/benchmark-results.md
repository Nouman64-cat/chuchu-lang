# Benchmark results and analysis

These historical measurements describe the compiler and workload revisions
recorded below. Saved results retain their original samples, source hashes and
toolchain metadata. They are not measurements of every subsequent compiler
revision; use the [benchmark runner](../benchmarks/README.md) for fresh results.

## Environment and reproducibility

Measured on Apple M2 Pro, 12 logical cores, 16 GiB RAM, ARM64;
macOS 26.6.2 (25G83). LLVM 23.1.4, Inkwell 0.11.0;
`rustc 1.99.1 (58cf8f9c3 2026-10-13)`; Apple Clang 21.0.0 (clang-2100.3.34.2);
Python 3.14.6. Measurements began at `2026-10-28T14:35:47.036580+00:00`.
Compiler SHA-256: `079cbdfa1e12f9c7f781e6f9207860232613118a8858e714147202f04546636a`. This hash identifies the
measured binary.

```sh
export LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"
cargo build --release
python3 benchmarks/run.py --cheeku ./target/release/cheeku --output benchmarks/results/macos-arm64-full.json
```

The full run completed: 57 workload/variant entries, seven execution samples
each (399 timed executions), three compilation samples per compiled variant,
one warmup, no skipped references, and 12 compiler-throughput entries. Timed
executions validate expected checksums; initial and warmup checks also passed.
Compilation, diagnostic generation and RSS sampling are outside execution timers.
The vectorization probe is the extra 57th variant. Compilation wall times include
CLI startup and linking; execution includes native/interpreter startup and final
output capture. Full parameters, raw samples, compiler stages, flags, hashes,
inspection paths and memory observations are in
[`macos-arm64-full.json`](../benchmarks/results/macos-arm64-full.json).
The complete per-variant table including IQR, compile time, size and RSS is in
[`macos-arm64-full.md`](../benchmarks/results/macos-arm64-full.md).

References use identical algorithms/inputs; Rust `-O -C target-cpu=generic`, C
`-O2 -ffp-contract=off`, Python standard lists and integers. C/Rust integer helpers
check overflow; these workloads stay within i64. Python is arbitrary precision,
and interpreter/list costs differ. Floating outputs compare as identical
15-digit decimal values, allowing scientific notation without broad tolerance.
No NumPy, artificially slow native references or printing inside hot loops.

## Execution results

Milliseconds per process, **median / IQR**. Compilation excluded. Ratios are
process-level comparisons, not isolated kernel throughput or universal language
rankings. IQR is q3 minus q1 with inclusive quartiles.

| Workload | O0 | O1 | O2 | O3 | Rust | C | Python |
|---|---:|---:|---:|---:|---:|---:|---:|
| integer_loop | 7.228 / 1.303 | 3.618 / 0.643 | 3.899 / 1.096 | 3.513 / 0.369 | 5.310 / 4.137 | 3.905 / 0.563 | 255.861 / 9.047 |
| float_loop | 8.332 / 0.883 | 5.449 / 0.717 | 5.435 / 0.944 | 4.964 / 0.457 | 5.330 / 1.252 | 4.672 / 0.773 | 98.013 / 7.625 |
| function_calls | 4.072 / 0.816 | 2.618 / 0.341 | 2.700 / 0.691 | 2.741 / 0.350 | 3.529 / 0.706 | 3.133 / 0.740 | 62.516 / 4.720 |
| recursion | 4.575 / 0.789 | 3.980 / 0.622 | 3.990 / 0.368 | 3.890 / 1.669 | 4.398 / 1.879 | 3.580 / 0.947 | 49.637 / 5.047 |
| nested_loops | 4.520 / 1.870 | 2.772 / 0.857 | 2.602 / 0.249 | 2.672 / 0.782 | 3.146 / 1.794 | 2.545 / 0.376 | 47.598 / 5.397 |
| array_multiply | 24.075 / 2.591 | 6.406 / 1.157 | 4.603 / 0.740 | 3.553 / 0.884 | 4.425 / 0.575 | 4.017 / 1.035 | 186.255 / 72.910 |
| array_reduce | 12.125 / 1.469 | 4.985 / 0.717 | 4.773 / 1.340 | 4.439 / 0.462 | 3.998 / 0.965 | 3.991 / 0.403 | 140.601 / 1.963 |
| array_copies | 11.740 / 0.465 | 2.759 / 0.429 | 2.760 / 0.798 | 2.719 / 0.383 | 3.467 / 2.121 | 5.238 / 1.092 | 45.447 / 4.871 |

The native empty-process control was 3.066 ms median, 0.645 ms IQR; Python's empty
interpreter was 23.615 / 1.123 ms. These are not subtracted. Several native results
are close to that noise floor; sub-millisecond differences and rankings among
Rust/C/Chuchu should not be treated as established language advantages.

| Workload | O0/O2 median ratio | Interpretation |
|---|---:|---|
| integer_loop | 1.85× | Scalar locals promoted; startup significant |
| float_loop | 1.53× | Dependent floating loop retained |
| function_calls | 1.51× | Inlining allowed, not pure call overhead |
| recursion | 1.15× | Recursion retained; small difference |
| nested_loops | 1.74× | Nested control flow simplified |
| array_multiply | 5.23× | Float element-wise vector arithmetic observed |
| array_reduce | 2.54× | Checked integer reduction; scalar checks retained |
| array_copies | 4.25× | Unused array elements eliminated, not bandwidth |

The largest useful arithmetic result is array multiplication: 24.075 → 4.603 ms
at O2 (5.23× median ratio), 3.553 ms at O3 (6.78×). O0 and O2 quartile ranges are
well separated, and the independent repeat also preserves a large difference.
These ratios include process overhead and are not a SIMD-only attribution.
Integer reduction is 12.125 → 4.773 ms, with substantial optimized-run variation.
The 11.740 → 2.760 ms array-copy result measures dead-element elimination:
**only element zero is observed**, so LLVM can eliminate most of the 256-element
copy. It is not full-array memory-copy throughput. The two arithmetic array
workloads consume all elements, but 64-element working sets fit in cache.

Python was slower for these selected workloads, with interpreter/list overhead
included. Native comparisons vary: C's array-reduction median is 3.991 ms versus
Chuchu O2's 4.773 ms, and float-loop medians are 4.672 versus 5.435 ms. Rust's
array-multiply median is 4.425 ms versus Chuchu O2's 4.603 ms. Variation limits
these fine-grained comparisons. There is no general claim that Chuchu beats
optimized C or Rust, and O3 is not uniformly better than O2.

## Compilation cost, executable size and memory

Compilation wall milliseconds, median of three full CLI invocations. Every
number includes the system linker. O2 sizes and RSS are in bytes; RSS is a
separate single macOS `time -l` run, including runtime overhead.

| Workload | O0 compile | O1 compile | O2 compile | O3 compile | O2 executable bytes | O2 peak RSS bytes |
|---|---:|---:|---:|---:|---:|---:|
| integer_loop | 52.284 | 64.423 | 62.668 | 54.325 | 33512 | 1376256 |
| float_loop | 52.477 | 52.668 | 50.547 | 53.138 | 33432 | 1359872 |
| function_calls | 54.570 | 60.636 | 54.713 | 54.599 | 33576 | 1376256 |
| recursion | 49.681 | 51.902 | 62.801 | 52.747 | 33560 | 1376256 |
| nested_loops | 57.849 | 51.262 | 52.709 | 55.620 | 33512 | 1376256 |
| array_multiply | 55.451 | 100.636 | 81.162 | 84.092 | 33432 | 1376256 |
| array_reduce | 53.842 | 71.174 | 77.501 | 74.326 | 33512 | 1376256 |
| array_copies | 72.286 | 66.788 | 61.088 | 67.133 | 33432 | 1376256 |

Optimization has an observable compile-time cost: array multiplication is
55.451 → 81.162 ms O0→O2 (+46.4%), reduction 53.842 → 77.501 ms (+43.9%). These
are measured tradeoffs with only three compilation samples; system linking is
noisy. Array copies shrink from 50,024 bytes at O0 to 33,432 at O2 (33.2% smaller).
Most small executables have unchanged file size because of Mach-O layout/granularity.

Across workloads, Chuchu O2 runtime RSS was 1,359,872–1,376,256 bytes, C
1,359,872–1,392,640, Rust 1,556,480–1,589,248, Python 15,122,432–16,121,856.
These include different runtimes/interpreters and are not evidence of general
allocator superiority. No heap allocator was introduced: current fixed arrays
start in entry-block storage and can be scalarized. O2 compiler RSS was
41,074,688–50,921,472 bytes versus O0 37,797,888–38,305,792. Rust reference
compilers used 91,865,088–96,632,832 bytes and C 39,763,968–42,024,960.
Rust executable sizes were roughly 471–491 KB versus roughly 33.4 KB C/Chuchu;
link/runtime policies differ. Python executable size/compile time are N/A.

### Stage timings

O2 stage medians in milliseconds (each stage's median independently, so columns
need not sum to median total). Target setup is separate from IR/optimization.

| Workload | Lex | Parse | Type | IR | Target | Optimize | Object | Link | API total |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| integer_loop | 0.003 | 0.005 | 0.004 | 0.154 | 0.156 | 2.183 | 2.228 | 40.767 | 46.182 |
| float_loop | 0.003 | 0.004 | 0.005 | 0.134 | 0.114 | 1.686 | 1.682 | 34.791 | 38.903 |
| function_calls | 0.003 | 0.006 | 0.005 | 0.156 | 0.135 | 2.422 | 2.249 | 36.457 | 42.474 |
| recursion | 0.003 | 0.006 | 0.005 | 0.155 | 0.153 | 2.208 | 2.220 | 42.703 | 50.064 |
| nested_loops | 0.003 | 0.005 | 0.004 | 0.140 | 0.128 | 2.220 | 2.225 | 35.503 | 40.752 |
| array_multiply | 0.015 | 0.018 | 0.008 | 0.268 | 0.124 | 18.866 | 9.159 | 38.517 | 67.546 |
| array_reduce | 0.014 | 0.018 | 0.005 | 0.254 | 0.141 | 13.448 | 10.488 | 38.241 | 64.165 |
| array_copies | 0.037 | 0.036 | 0.011 | 0.426 | 0.122 | 10.401 | 1.708 | 35.293 | 49.176 |

The external CLI timer also includes process loading and shutdown. Linker work
dominates small programs; array IR optimization and object generation become
material. Per-pass verification is included in optimization and retained for
correctness; no separate claim about its cost is made.

Compiler-throughput O2 inputs contain 33/257/1,025 statements. Stage medians ms:

| Statements | Source bytes | Lex | Parse | Type | IR | Optimize | Object | Link | Full CLI |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 33 | 569 | 0.008 | 0.009 | 0.008 | 0.100 | 1.203 | 1.627 | 38.318 | 54.568 |
| 257 | 4914 | 0.042 | 0.037 | 0.032 | 0.314 | 1.541 | 1.669 | 47.103 | 69.011 |
| 1025 | 20323 | 0.114 | 0.122 | 0.092 | 0.787 | 2.522 | 1.751 | 44.722 | 64.811 |

The largest frontend still spends about 0.33 ms across lex/parse/type checking.
Measurements do not justify redesigning token allocation, scope maps or AST
ownership here. No speculative frontend micro-optimization was added. Linker
startup and optimization/IR growth are more credible future targets; increasing
inputs are mostly declarations, not a representative large-codebase stress test.

## Confirmed IR, loop and SIMD findings

Static IR instruction counts O0 → O2; these are not dynamic operation counts.

| Workload | Allocas | Loads | Stores | IR lines |
|---|---:|---:|---:|---:|
| integer_loop | 2 → 0 | 5 → 0 | 4 → 0 | 70 → 52 |
| float_loop | 2 → 0 | 4 → 0 | 4 → 0 | 58 → 30 |
| function_calls | 3 → 0 | 7 → 0 | 5 → 0 | 109 → 87 |
| recursion | 1 → 0 | 4 → 0 | 1 → 0 | 89 → 58 |
| nested_loops | 3 → 0 | 8 → 0 | 6 → 0 | 115 → 66 |
| array_multiply | 18 → 1 | 23 → 2 | 151 → 32 | 410 → 501 |
| array_reduce | 13 → 4 | 17 → 195 | 145 → 161 | 414 → 615 |
| array_copies | 8 → 0 | 11 → 0 | 522 → 0 | 1101 → 31 |

Scalar locals are promoted, control flow simplified and temporary arrays reduced
by standard LLVM pipelines. Function-call IR is simplified and can inline; the
recursive workload retains recursion. CFG inspection found a loop/recursion
reachable from `main` in each optimized Chuchu workload; none was classified as a
wholly constant result. This structural check does not prove every original
source operation remains. In particular, array copies fall from 1,101 to 31 IR
lines as dead elements disappear. Integer reduction unrolling increases static
load/store/IR counts even while execution improves; fewer IR lines is not itself
a performance objective.

Float array multiplication has **64 vector arithmetic IR operations and 64
AArch64 vector arithmetic instructions** in O2/O3 diagnostic artifacts, including
`fmul.2d`/`fadd.2d` forms. Counts are static, not measured hardware vector utilization.
The loop is retained and repeated array updates/reduction remain. O2 leaves one
alloca; O3 eliminates all entry allocations in this workload. The checked integer
reduction has no corresponding vector arithmetic; overflow guards and ordered
reduction legitimately constrain SIMD. The dependent scalar float loop is also
not vectorized. No fast math or unchecked indexing/arithmetic was used.

The O3 `--no-vectorize` probe also contains 64 vector operations/instructions,
because disabling loop/SLP options does not prevent all standard LLVM vector
formation. Its 3.419 ms median (0.416 IQR) versus O3's 3.553 (0.884 IQR) cannot
establish a SIMD speedup. An independently verified scalar lowering or pass
isolation experiment is still needed for that attribution. No custom array/reduce
fusion was added: it could change overflow order and diagnostics, and standard
LLVM already gives measurable allocation and arithmetic improvements.

## Repeatability and regressions

An actual second run, same compiler hash and source/tool/target policy, executed:

```sh
python3 benchmarks/run.py --cheeku ./target/release/cheeku --workloads integer_loop,float_loop,array_multiply --levels O0,O2,O3 --languages chuchu --repetitions 7 --compile-repetitions 3 --no-throughput --output benchmarks/results/macos-arm64-repeat.json --baseline benchmarks/results/macos-arm64-full.json
```

[`macos-arm64-repeat.regressions.json`](../benchmarks/results/macos-arm64-repeat.regressions.json)
uses a 10% threshold and IQR/median noise limit 15%. Results:

| Workload/variant | Baseline ms | Repeat ms | Change | Classification |
|---|---:|---:|---:|---|
| integer_loop / chuchu-O0 | 7.228 | 6.376 | -11.8% | unreliable/noisy |
| integer_loop / chuchu-O2 | 3.899 | 3.537 | -9.3% | unreliable/noisy |
| integer_loop / chuchu-O3 | 3.513 | 3.593 | +2.3% | unchanged/overlapping intervals |
| float_loop / chuchu-O0 | 8.332 | 7.734 | -7.2% | unchanged/overlapping intervals |
| float_loop / chuchu-O2 | 5.435 | 4.603 | -15.3% | unreliable/noisy |
| float_loop / chuchu-O3 | 4.964 | 4.853 | -2.2% | unchanged/overlapping intervals |
| array_multiply / chuchu-O0 | 24.075 | 23.227 | -3.5% | unchanged/overlapping intervals |
| array_multiply / chuchu-O2 | 4.603 | 4.167 | -9.5% | unreliable/noisy |
| array_multiply / chuchu-O3 | 3.553 | 3.490 | -1.8% | unreliable/noisy |
| array_multiply / chuchu-O3-no-vectorize | 3.419 | 3.527 | +3.2% | unchanged/overlapping intervals |

Five comparisons overlap/are unchanged and five are unreliable/noisy. No reliable
runtime regression or improvement was classified between identical compiler
runs. Large apparent gains in noisy rows are deliberately not labeled wins.
Compilation-time increases above are real observed optimization costs; they are
not hidden by the runtime-only regression classifier. No general absence of
regressions beyond these workloads/this host is claimed. Comparator thresholds
are configurable and ordinary correctness tests have no timing gates.

## Remaining limitations and future work

- These measurements cover one macOS ARM64 host. Compiler correctness on
  other platforms is tracked separately in [platform support](platform-support.md);
  these timings do not establish cross-platform performance.
- Several native times are near process startup, with material variation. Static
  inputs and inlining/dead elements prevent broad dynamic throughput claims.
  No genuinely DRAM-sized array workload is possible under the current 256 limit.
- SIMD presence is confirmed; isolated SIMD speedup is unmeasured. The toggle
  cannot supply a proven scalar control for this workload.
- RSS is one process-level observation, not allocator/stack profiling. Code size
  reflects runtime/link policies. No hardware counters or energy measurements.
- No evidence justified custom array fusion, unsafe numerical shortcuts, scope
  data-structure rewrites or a new memory model. These were intentionally omitted.
- Future experiments can isolate a scalar vectorization control, use longer
  repeated runs on a dedicated host, profile LLVM pass/verification costs, and
  study linker startup and richer compiler inputs. None requires changing syntax
  or removing checks; none is presented as implemented here.
