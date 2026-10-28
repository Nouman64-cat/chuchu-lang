---
title: "Performance & benchmarks"
description: "Run Chuchu correctness-first benchmarks and interpret process timings, static inputs, vectorization, and regression reports."
---

The compiler repository includes eight numerical workloads, reference implementations in C/Rust/Python, a benchmark runner, and saved measurement data.

## Run the suite

From `chuchu-core`, with the [LLVM setup](../getting-started/installation.md) configured:

```bash
cargo build --release
python3 benchmarks/run.py --output benchmarks/results/my-run.json
```

The runner measures all four Chuchu optimization levels, optimized C and Rust references when available, Python references, a vectorization probe, and compiler-throughput inputs. The current harness is designed around the project's macOS environment.

Default sampling uses one warmup, seven execution samples, and three compilation samples. Every timed executable must produce its expected result. Compilation and execution are timed separately.

## Select a subset

```bash
python3 benchmarks/run.py --workloads integer_loop,array_multiply --levels O0,O2,O3 --languages chuchu --no-throughput --output benchmarks/results/subset.json
```

Workloads cover integer and float loops, function calls, recursion, nested loops, array multiplication, integer reduction, and array value copies.

## Interpret the results

- Execution time includes process startup and output capture; it is not an isolated kernel timer.
- Startup controls are recorded without subtraction.
- Inputs are static. Optimizers may inline functions, fold results, or remove unused array elements.
- The copy workload observes only element zero, so it measures dead-copy elimination rather than full-array bandwidth.
- All arrays fit within the 256-element limit; results do not establish large-memory bandwidth.
- Vector IR/assembly presence is structural evidence, not an isolated SIMD speedup.
- Python uses its own integer/list semantics and includes interpreter overhead.
- macOS peak RSS is a separate single process-level measurement.

Saved JSON and Markdown reports are historical measurements on a particular host, not guarantees for your machine. The narrative [Phase 7 report](https://github.com/chuchu-lang/chuchu-core/blob/main/docs/phase7-performance-report.md) and saved result files may describe different runs; compare timestamps, hashes, and raw data before quoting numbers.

## Compare runs

```bash
python3 benchmarks/compare.py benchmarks/results/phase7-full.json benchmarks/results/my-run.json --output benchmarks/results/comparison.json
```

The comparator checks environment, workload hashes, and compilation policy. With default thresholds, noisy data is labeled unreliable; separated quartile intervals and a slowdown above 10% are flagged as a possible regression. This is a screening heuristic, not a significance test.

An optional `--fail-on-regression` switch enables a gate for controlled machines. Ordinary correctness tests do not enforce timing thresholds.
