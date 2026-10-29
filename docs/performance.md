# Performance measurement

Use a release compiler and measure compilation separately from executable
execution. Build instructions and one-command full benchmarks are in
[benchmarks/README.md](../benchmarks/README.md). The
[benchmark analysis](benchmark-results.md) records actual results and limits.

## Compilation timings

`cheeku source.chuchu -O2 --time-compilation -o program` prints one JSON object to
stderr after successful compilation, with durations in seconds:

- `lex_s`: tokenization.
- `parse_s`: complete AST parsing.
- `type_s`: semantic checking.
- `ir_s`: LLVM context creation and program lowering, including its verification.
- `target_setup_s`: target initialization, CPU selection and data layout.
- `optimization_s`: IR pipeline plus its verification (O0 measures verification).
- `object_s`: final verification, object emission and object-file write.
- `link_s`: launching and waiting for the system linker driver.
- `total_s`: complete compiler API work, including file read for file compilation.

These are actual monotonic `Instant` intervals, not estimates or proportional
allocations. Total includes orchestration, staging and cleanup and need not equal
the sum of stages. File-level total includes LLVM teardown; source-level total is
stamped before local teardown. Neither includes CLI startup, loading libraries or
printing timings. Benchmark compilation wall time measures the whole CLI process
and therefore differs. Optional IR/assembly diagnostics add work to total, but
are outside object/optimization stage intervals; timed benchmark compilations do
not request diagnostics. Diagnostic inspection is a separate build. Verification
is deliberately retained in release builds, including per-pass verification.

## Reproducibility and interpretation

The runner uses `perf_counter_ns`, one warmup, seven randomized round-robin
execution rounds and three compilation samples by default. It stores every
sample, median and inclusive quartiles/IQR. Every executable must produce the
expected checksum before timing, and each timed run is checked afterward.
Compilation and execution timings never overlap. Each run uses a unique artifact
directory under `target/cheeku-bench` and records machine/tool versions, compiler
and workload hashes, target policy, flags and workload parameters.

Execution numbers include process startup and output capture. Empty native and
Python startup controls are reported, but are **not subtracted**: the controls
are different executables and subtraction could manufacture misleading kernel
times. Many native workloads last only a few milliseconds, so small differences
and language rankings are inconclusive. Static inputs also let optimizers remove
work. IR inspection records reachable loop/recursion evidence and allocation,
load, store and vector arithmetic counts. This is structural evidence, not a
proof that every source operation survives. `array_copies` intentionally exposes
dead-copy elimination: only element zero is consumed, so optimized code is not
measuring full-array copy bandwidth. Other array workloads consume all elements.
All arrays are at most 256 elements and fit in cache; no DRAM bandwidth claim is
possible. The function-call workload permits inlining, so it measures function
optimization rather than mandatory out-of-line call overhead.

Rust/C use optimized builds with checked integer helpers and equivalent loop
algorithms. Python uses built-in integers/lists, with arbitrary precision and
interpreter/allocation overhead; no NumPy. All selected integer results fit i64.
Floating results are compared as identical 15-digit decimal values, allowing
notation differences but no broad tolerance. C disables floating contraction;
Cheeku does not enable fast math. These output checks do not establish bitwise
identity of all intermediates across languages.

Peak RSS is a separate macOS `/usr/bin/time -l` run in bytes, including runtime
and interpreter overhead. It is a single observation per case, not allocator
instrumentation. Compiler RSS is separately recorded. Unavailable measurements
are null/N/A; Python has no comparable compiled executable size or compile time.
Executable size includes different language runtimes and linking policies.

## Regression checks

`benchmarks/compare.py` matches workloads/variants, machine/tool versions, source
hashes and target/reference flags. It reports percent changes and IQRs. By default,
IQR/median above 15% is unreliable; otherwise a slowdown above 10% with separated
quartile intervals is a possible regression. Both limits are configurable.
This is a heuristic screening tool, not a significance test. It reports by default;
`--fail-on-regression` is reserved for controlled dedicated machines.

Ordinary `cargo test` and the cross-platform CI jobs check correctness rather
than timing thresholds. Run benchmarks on an idle machine, preserve raw results
and repeat possible regressions before attributing them to code changes. See
[platform support](platform-support.md) for native validation evidence.
