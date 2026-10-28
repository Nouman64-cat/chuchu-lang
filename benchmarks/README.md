# Cheeku benchmarks

Current workloads use `assign` for variable declarations. Saved benchmark results
and historical profiling sources retain the syntax and source hashes from their
original runs; regenerate measurements before comparing with the current sources.

The runner requires Python's standard library, a release Cheeku compiler and the
same macOS Apple Silicon LLVM/Clang setup as the compiler. Rust and C references
run when `rustc` and `clang` are present; missing tools are recorded as skips.
Python references use the interpreter running the harness.

```sh
export LLVM_SYS_231_PREFIX="$(brew --prefix llvm)"
cargo build --release
python3 benchmarks/run.py --output benchmarks/results/latest.json
```

The last command runs the **full suite**, including O0/O1/O2/O3, optimized Rust/C,
Python, the array vectorization probe and compiler-throughput inputs. Compilation
is outside execution timing. Artifacts are isolated under `target/cheeku-bench`;
JSON and a Markdown summary go to the chosen result path. Choose a new output
name to retain earlier measurements. `--artifacts` may specify a new directory
that does not already exist. Do not run correctness builds concurrently with
performance measurements.

## Workloads and references

`manifest.json` records parameters and expected checksums. `generate.py` reproduces
all source files and the manifest; changing them invalidates old comparisons.

| Workload | Work performed |
|---|---|
| integer_loop | 3,000,000 checked integer sum iterations |
| float_loop | 1,000,000 dependent floating multiply/add steps |
| function_calls | 300,000 square-plus calls; inlining is permitted |
| recursion | Recursive Fibonacci of 28 |
| nested_loops | 500 × 500 checked integer product/sum iterations |
| array_multiply | 30,000 multiply/reduce/update iterations over 64 floats |
| array_reduce | 30,000 checked reduction/update iterations over 64 integers |
| array_copies | 30,000 value-copy swaps of 256 floats; only index 0 observed |
| compiler throughput | 32, 256, 1,024 declarations plus one output, every level |

All use current `function`, `reassign` and lowercase type syntax. No dynamic input,
strings, NumPy or new language features were added. Array copies have the same
value semantics in references (Python explicitly copies lists). **Array copies
is a dead-copy elimination benchmark**, not a measurement of 256-element memory
bandwidth: LLVM removes unused elements. All array working sets fit in cache.
IR/assembly evidence is saved and summarized. Retaining a loop alone does not
prove all source work survived. Static-input optimization is allowed and is
separate from dynamic arithmetic-throughput claims.

Reference flags: Rust `-O -C target-cpu=generic`; C `-O2 -ffp-contract=off`; Cheeku
uses its generic CPU default. C uses builtin checked add/sub/mul helpers; Rust
uses checked operations. The selected inputs never overflow. Python integers
are arbitrary precision, unlike checked i64; Python list allocation and runtime
costs are included. Floating outputs compare exactly as 15-digit decimals, not
with a masking tolerance. Benchmarks validate all checksums before recording
execution and again on every timed sample.

## Controls and regression reporting

Defaults: 7 executions, 3 compilations, 1 warmup. These can be changed with
`--repetitions`, `--compile-repetitions` and `--warmups`. Variants execute in
randomized rounds with seed 7. `--workloads integer_loop,array_multiply`,
`--levels O0,O2,O3`, `--languages chuchu,rust,c,python`, `--no-throughput` and
`--no-simd-probe` select subsets. `--cheeku` selects another release compiler.

```sh
python3 benchmarks/run.py --output benchmarks/results/new.json --baseline benchmarks/results/phase7-full.json
python3 benchmarks/compare.py benchmarks/results/phase7-full.json benchmarks/results/new.json --threshold 10 --noise-limit 0.15 --output benchmarks/results/comparison.json
python3 -m unittest discover -s benchmarks -p test_runner.py
```

The runner's `--baseline` writes a `.regressions.json` report; its threshold can be
changed with `--regression-threshold`. Standalone `compare.py` also accepts
`--fail-on-regression` for a dedicated-host performance gate. Correctness failures
always stop the runner; performance comparisons normally report without failing.
Metadata mismatches produce incomparable cases, not a fabricated speedup.

`--no-vectorize` turns off LLVM loop/SLP options, not every mechanism that creates
vectors. Both measured array variants still contain vector operations; the probe
does not establish an isolated SIMD speedup. IR/assembly compilation is separate
from timed compilation and uses a separate executable. Timing includes process
launch and final output capture, not compiler execution. Startup control samples
are recorded without subtraction. RSS comes from separate macOS `time -l` runs;
null means unavailable. See [measurement details](../docs/performance.md).

## Saved evidence

- `results/preoptimization-baseline.json`: actual original compiler measurements;
  different workload size and warm-process methodology, not an apples-to-apples
  full-suite baseline. Its harness is saved as `baseline_profile.rs.txt`.
- `results/phase7-full.json` / `.md`: full measured suite, all raw execution and
  compilation samples, stage times, hashes, sizes, memory and IR evidence.
- `results/phase7-repeat.json` / `.md` / `.regressions.json`: actual repeat of three
  workloads at O0/O2/O3, including the vectorization probe.
- [Phase 7 report](../docs/phase7-performance-report.md): findings and limitations.

No generated executables are stored outside ignored `target` directories.
