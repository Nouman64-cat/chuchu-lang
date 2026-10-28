# Phase 7 performance engineering report

## Scope and implementation

Phase 7 adds LLVM optimization and measured performance tooling to the existing
native compiler. No syntax, type system, AST, production lowering, runtime
arithmetic or array semantics changed. No Phase 8 features, GPU execution,
automatic parallelism, JIT, dependencies, commits, pushes or releases were added.

A small `BuildOptions` configuration flows from the CLI through compiler and
native APIs. Compatibility wrappers preserve existing signatures. O0 skips
optional IR passes; O1/O2/O3 use LLVM 23's standard new-pass-manager pipelines
`default<O1>`, `default<O2>`, `default<O3>` with corresponding target-machine
levels. Default is O2 and generic host-architecture CPU; explicit `--cpu native`
opts into host CPU features. Triple/data layout precede optimization. Modules are
verified after lowering, around optimization, after each pass and before emission.
Pass/target errors propagate without silently changing optimization policy.

`--time-compilation` reports real stage intervals on stderr. `--emit-ir` and
`--emit-assembly` expose optimized artifacts. Unknown/repeated flags fail;
`-o` and optimization flags work in either order after the source. Output paths
with spaces are supported. Source/artifact collisions are rejected. Existing
staged linking, temporary cleanup and executable preservation on failure remain.

### Work completed by stage

| Stage | Result |
|---|---|
| 7.1 audit | Inspected native/compiler/CLI and typed/runtime/array lowering; preserved local example change; captured original compiler baseline |
| 7.2 pipelines | O0–O3, target-aware passes, default O2, compatible APIs, structured errors |
| 7.3 scalar/functions | Verified local promotion, fewer loads/stores, simplification and inlining through standard LLVM pipelines |
| 7.4 memory/arrays | Measured temporary elimination and copy reduction; retained array cap 256 and checked evaluation order |
| 7.5 loops/SIMD | Inspected reachable loops/recursion and actual vector IR/ARM64 assembly; no isolated SIMD speedup claim |
| 7.6 compiler throughput | Independent timings, increasing input sizes; no speculative frontend rewrite where link/LLVM dominated |
| 7.7 benchmarks | Eight workloads, four Chuchu levels, optimized Rust/C, Python; real full run with checksums |
| 7.8 correctness | 22 native programs × four levels, exact stdout/stderr/status equivalence; all existing tests retained |
| 7.9 regressions | Machine-readable, noise-aware comparator; actual independent subset rerun |
| 7.10 documentation | Usage, APIs, methodology, raw results and this report |

## Audit and baseline integrity

The starting revision was `1eb8ea32729724de9997a3012ffece98484bd720`.
Initial Git status was clean. During the audit `examples/main.chuchu` acquired
`reassign x to 15`; that local edit was preserved. The initial `cargo test`
therefore exposed a stale parser fixture, so the initial suite cannot be reported
as green. Lexer/parser/native assertions were updated to verify the complete
current example, including exact assignment and expression spans. Assertions
were not removed or weakened. The latest local example produces **35**, despite
the assignment text's historical expectation of 30.

Before editing compiler source, the original release binary, library, IR and
object were retained in ignored `target/phase7-audit`. Recorded measurements and
the harness are in
[`preoptimization-baseline.json`](../benchmarks/results/preoptimization-baseline.json)
and [`baseline_profile.rs.txt`](../benchmarks/results/baseline_profile.rs.txt).
That genuine historical sample is a 200,000-iteration sum, five in-process
compilations and nine process executions. Its runtime median was 3.705 ms,
executable 33,520 bytes and RSS 1,392,640 bytes. Warm object emission including
target setup was about 0.74–0.85 ms; linking about 34–36 ms. The first cold LLVM
sample was much slower (IR 7.24 ms, object/target 35.32 ms). Original target policy
used host CPU features. There was no optional IR optimizer, so its optimizer time
is null, not an invented zero-stage measurement.

This is **not** directly comparable with the new full suite: workload size, CPU
policy, warm-process and stage definitions differ. Controlled new O0 comparisons
below hold those choices fixed and are the main performance evidence.

## Environment and reproducibility

Measured on Apple M2 Pro, 12 logical cores, 16 GiB RAM, ARM64;
macOS 26.6.2 (25G83). LLVM 23.1.4, Inkwell 0.11.0;
`rustc 1.99.1 (58cf8f9c3 2026-10-13)`; Apple Clang 21.0.0 (clang-2100.3.34.2);
Python 3.14.6. Measurements began at `2026-10-28T14:35:47.036580+00:00`.
Compiler SHA-256: `079cbdfa1e12f9c7f781e6f9207860232613118a8858e714147202f04546636a`. The final release rebuild has the
same hash as the measured binary.

```sh
export LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"
cargo build --release
python3 benchmarks/run.py --cheeku ./target/release/cheeku --output benchmarks/results/phase7-full.json
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
[`phase7-full.json`](../benchmarks/results/phase7-full.json).
The complete per-variant table including IQR, compile time, size and RSS is in
[`phase7-full.md`](../benchmarks/results/phase7-full.md).

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
python3 benchmarks/run.py --cheeku ./target/release/cheeku --workloads integer_loop,float_loop,array_multiply --levels O0,O2,O3 --languages chuchu --repetitions 7 --compile-repetitions 3 --no-throughput --output benchmarks/results/phase7-repeat.json --baseline benchmarks/results/phase7-full.json
```

[`phase7-repeat.regressions.json`](../benchmarks/results/phase7-repeat.regressions.json)
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

## Correctness and final validation

All commands completed successfully on this macOS Apple Silicon installation:

```sh
cargo fmt --all
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
python3 -m unittest discover -s benchmarks -p test_runner.py
```

**132 Rust tests passed**, zero failures/ignored: 53 library unit tests, four CLI
unit tests and 75 integration tests. The Python comparator suite passed three
tests covering regression direction, noise/overlap and metadata mismatch.
Formatting, warnings-denied Clippy and the release build passed. Inkwell and
Homebrew LLVM dynamic discovery worked; no new linking workaround was needed.

The native matrix compiles and runs **22 cases × O0/O1/O2/O3 = 88 executions**,
comparing exact stdout, stderr and exit code against O0. Cases include arithmetic
precedence, reassignment/branches, nested loops, nested calls, recursion and
mutual recursion, arrays/copies/indexing/element-wise products/reductions, float
rounding, signed zero, NaN comparison/minimum, i64 minimum, empty input, signed
add/sub/mul/neg/abs/sum overflow, MIN/-1 division, integer/float division by zero,
and both array bounds. Error cases must exit 1 and report runtime diagnostics;
O3 preserves those guards. Checked product/reduction evaluation order is exercised.
Other integration tests cover invalid modules/errors, actual pass execution,
artifact protection, CLI diagnostics and output paths containing spaces. Existing
Phase 1–6 frontend/codegen/native coverage remains enabled.

Release examples were compiled into `target/phase7-manual` and executed:

- `main.chuchu` at O0/O1/O2/O3: `35\n` each, empty stderr, success.
- `syntax_smoke.chuchu` at O3: `25, 100, 4.5, true, 0, 1, 2` on separate lines.
- `numerical.chuchu` at O3: `32, 3, 7` on separate lines.

The expected 35 follows the preserved local reassignment; it is not a compiler
behavior change. Native test and final command logs remain in ignored `target`;
the permanent tests, benchmark JSON and report provide reproducible evidence.

## Remaining limitations and future work

- Validated on one macOS ARM64 host only. No Linux/Windows portability or hosted
  CI claim; no existing workflow was present and no untested LLVM-23 bootstrap
  workflow was added.
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

## Modified and created files

The local edit to `examples/main.chuchu` was preserved, not authored as a Phase 7
change. The following implementation, fixture, documentation and evidence files
were modified or created (all paths relative to repository root):

- `.gitignore`
- `README.md`
- `benchmarks/README.md`
- `benchmarks/chuchu/array_copies.chuchu`
- `benchmarks/chuchu/array_multiply.chuchu`
- `benchmarks/chuchu/array_reduce.chuchu`
- `benchmarks/chuchu/float_loop.chuchu`
- `benchmarks/chuchu/function_calls.chuchu`
- `benchmarks/chuchu/integer_loop.chuchu`
- `benchmarks/chuchu/nested_loops.chuchu`
- `benchmarks/chuchu/recursion.chuchu`
- `benchmarks/compare.py`
- `benchmarks/generate.py`
- `benchmarks/manifest.json`
- `benchmarks/reference/c/array_copies.c`
- `benchmarks/reference/c/array_multiply.c`
- `benchmarks/reference/c/array_reduce.c`
- `benchmarks/reference/c/float_loop.c`
- `benchmarks/reference/c/function_calls.c`
- `benchmarks/reference/c/integer_loop.c`
- `benchmarks/reference/c/nested_loops.c`
- `benchmarks/reference/c/recursion.c`
- `benchmarks/reference/python/array_copies.py`
- `benchmarks/reference/python/array_multiply.py`
- `benchmarks/reference/python/array_reduce.py`
- `benchmarks/reference/python/float_loop.py`
- `benchmarks/reference/python/function_calls.py`
- `benchmarks/reference/python/integer_loop.py`
- `benchmarks/reference/python/nested_loops.py`
- `benchmarks/reference/python/recursion.py`
- `benchmarks/reference/rust/array_copies.rs`
- `benchmarks/reference/rust/array_multiply.rs`
- `benchmarks/reference/rust/array_reduce.rs`
- `benchmarks/reference/rust/float_loop.rs`
- `benchmarks/reference/rust/function_calls.rs`
- `benchmarks/reference/rust/integer_loop.rs`
- `benchmarks/reference/rust/nested_loops.rs`
- `benchmarks/reference/rust/recursion.rs`
- `benchmarks/results/baseline_profile.rs.txt`
- `benchmarks/results/phase7-full.json`
- `benchmarks/results/phase7-full.md`
- `benchmarks/results/phase7-repeat.json`
- `benchmarks/results/phase7-repeat.md`
- `benchmarks/results/phase7-repeat.regressions.json`
- `benchmarks/results/preoptimization-baseline.json`
- `benchmarks/run.py`
- `benchmarks/test_runner.py`
- `docs/optimization.md`
- `docs/performance.md`
- `docs/phase7-performance-report.md`
- `src/compiler.rs`
- `src/lib.rs`
- `src/main.rs`
- `src/native.rs`
- `src/optimization.rs`
- `tests/lexer_integration.rs`
- `tests/native_integration.rs`
- `tests/optimization_integration.rs`
- `tests/parser_integration.rs`

No Cargo dependency changes were required. Benchmark binaries, IR/assembly
artifacts and validation logs remain under ignored `target` directories; saved
JSON/Markdown evidence is intentionally included for review. No Git commit,
push, release or Phase 8 work was performed.
