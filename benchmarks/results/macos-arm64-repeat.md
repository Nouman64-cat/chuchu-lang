# Benchmark results

Execution is wall time per process (ms), including startup; compilation excluded.

| Workload | chuchu-O0 | chuchu-O1 | chuchu-O2 | chuchu-O3 | rust | c | python |
|---|---:|---:|---:|---:|---:|---:|---:|
| integer_loop | 6.376 | N/A | 3.537 | 3.593 | N/A | N/A | N/A |
| float_loop | 7.734 | N/A | 4.603 | 4.853 | N/A | N/A | N/A |
| array_multiply | 23.227 | N/A | 4.167 | 3.490 | N/A | N/A | N/A |

## Individual measurements

| Workload / variant | Median ms | IQR ms | Compile ms | Bytes | Peak RSS bytes |
|---|---:|---:|---:|---:|---:|
| integer_loop / chuchu-O0 | 6.376 | 1.060 | 59.555 | 33512 | 1359872 |
| integer_loop / chuchu-O2 | 3.537 | 0.534 | 54.462 | 33512 | 1376256 |
| integer_loop / chuchu-O3 | 3.593 | 0.404 | 51.827 | 33512 | 1376256 |
| float_loop / chuchu-O0 | 7.734 | 1.014 | 57.676 | 33512 | 1376256 |
| float_loop / chuchu-O2 | 4.603 | 0.620 | 54.313 | 33432 | 1359872 |
| float_loop / chuchu-O3 | 4.853 | 0.432 | 55.140 | 33432 | 1376256 |
| array_multiply / chuchu-O0 | 23.227 | 1.914 | 56.851 | 33512 | 1376256 |
| array_multiply / chuchu-O2 | 4.167 | 0.264 | 82.114 | 33432 | 1376256 |
| array_multiply / chuchu-O3 | 3.490 | 0.872 | 76.330 | 33432 | 1376256 |
| array_multiply / chuchu-O3-no-vectorize | 3.527 | 0.395 | 79.618 | 33432 | 1376256 |

## Controlled optimization comparison

Ratios below are O0 median / optimized median (>1 favors optimized), not isolated kernel throughput.

| Workload | O0/O1 | O0/O2 | O0/O3 |
|---|---:|---:|---:|
| integer_loop | N/A | 1.80x | 1.77x |
| float_loop | N/A | 1.68x | 1.59x |
| array_multiply | N/A | 5.57x | 6.65x |

## IR inspection

- integer_loop / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=2, loads=5, stores=4, vector arithmetic IR/assembly=0/0.
- integer_loop / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- integer_loop / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- float_loop / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=2, loads=4, stores=4, vector arithmetic IR/assembly=0/0.
- float_loop / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- float_loop / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- array_multiply / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=18, loads=23, stores=151, vector arithmetic IR/assembly=0/0.
- array_multiply / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=1, loads=2, stores=32, vector arithmetic IR/assembly=64/64.
- array_multiply / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=64/64.
- array_multiply / chuchu-O3-no-vectorize: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=64/64.

All raw samples, compilation stage timings, machine/toolchain metadata, startup controls, source hashes, memory measurements, and compiler-throughput samples are in the adjacent JSON. Static inputs, process startup, bounds/overflow semantics, and Python allocation/interpreter overhead limit cross-language conclusions. Memory is measured separately with macOS time -l; Python executable size is not comparable and is N/A.
