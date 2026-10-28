# Benchmark results

Execution is wall time per process (ms), including startup; compilation excluded.

| Workload | chuchu-O0 | chuchu-O1 | chuchu-O2 | chuchu-O3 | rust | c | python |
|---|---:|---:|---:|---:|---:|---:|---:|
| integer_loop | 7.223 | 4.237 | 4.036 | 4.699 | 5.209 | 4.826 | 273.421 |
| float_loop | 8.439 | 6.555 | 6.307 | 5.545 | 6.022 | 5.301 | 96.466 |
| function_calls | 5.105 | 4.037 | 3.171 | 4.296 | 3.763 | 3.696 | 60.527 |
| recursion | 4.789 | 4.508 | 4.518 | 4.729 | 4.218 | 5.313 | 55.071 |
| nested_loops | 4.460 | 3.290 | 3.430 | 3.144 | 3.638 | 3.129 | 47.827 |
| array_multiply | 24.453 | 7.186 | 4.889 | 4.682 | 6.188 | 5.204 | 178.996 |
| array_reduce | 12.981 | 5.684 | 5.599 | 4.957 | 4.914 | 4.405 | 144.761 |
| array_copies | 11.929 | 4.147 | 3.593 | 3.558 | 3.452 | 5.341 | 46.401 |

## Individual measurements

| Workload / variant | Median ms | IQR ms | Compile ms | Bytes | Peak RSS bytes |
|---|---:|---:|---:|---:|---:|
| integer_loop / chuchu-O0 | 7.223 | 0.957 | 141.200 | 33512 | 1376256 |
| integer_loop / chuchu-O1 | 4.237 | 0.743 | 83.424 | 33512 | 1376256 |
| integer_loop / chuchu-O2 | 4.036 | 0.553 | 66.478 | 33512 | 1376256 |
| integer_loop / chuchu-O3 | 4.699 | 0.756 | 63.911 | 33512 | 1376256 |
| integer_loop / rust | 5.209 | 0.835 | 341.866 | 471040 | 1556480 |
| integer_loop / c | 4.826 | 0.694 | 167.061 | 33504 | 1376256 |
| integer_loop / python | 273.421 | 17.470 | N/A | N/A | 15122432 |
| float_loop / chuchu-O0 | 8.439 | 0.463 | 75.921 | 33512 | 1376256 |
| float_loop / chuchu-O1 | 6.555 | 0.535 | 65.955 | 33432 | 1359872 |
| float_loop / chuchu-O2 | 6.307 | 0.650 | 59.836 | 33432 | 1359872 |
| float_loop / chuchu-O3 | 5.545 | 0.330 | 67.057 | 33432 | 1359872 |
| float_loop / rust | 6.022 | 1.495 | 175.004 | 490592 | 1589248 |
| float_loop / c | 5.301 | 2.036 | 81.550 | 33440 | 1359872 |
| float_loop / python | 96.466 | 6.450 | N/A | N/A | 15138816 |
| function_calls / chuchu-O0 | 5.105 | 0.519 | 64.565 | 33576 | 1359872 |
| function_calls / chuchu-O1 | 4.037 | 0.536 | 66.755 | 33576 | 1376256 |
| function_calls / chuchu-O2 | 3.171 | 0.547 | 66.407 | 33576 | 1376256 |
| function_calls / chuchu-O3 | 4.296 | 0.890 | 95.902 | 33576 | 1376256 |
| function_calls / rust | 3.763 | 1.496 | 143.916 | 471040 | 1556480 |
| function_calls / c | 3.696 | 0.231 | 88.180 | 33504 | 1376256 |
| function_calls / python | 60.527 | 2.927 | N/A | N/A | 15171584 |
| recursion / chuchu-O0 | 4.789 | 0.655 | 65.558 | 33560 | 1376256 |
| recursion / chuchu-O1 | 4.508 | 0.197 | 62.101 | 33560 | 1376256 |
| recursion / chuchu-O2 | 4.518 | 0.478 | 63.092 | 33560 | 1376256 |
| recursion / chuchu-O3 | 4.729 | 0.874 | 57.410 | 33560 | 1392640 |
| recursion / rust | 4.218 | 1.327 | 143.031 | 471072 | 1556480 |
| recursion / c | 5.313 | 0.568 | 87.985 | 33512 | 1376256 |
| recursion / python | 55.071 | 0.882 | N/A | N/A | 15089664 |
| nested_loops / chuchu-O0 | 4.460 | 1.761 | 64.781 | 33512 | 1376256 |
| nested_loops / chuchu-O1 | 3.290 | 0.480 | 63.625 | 33512 | 1376256 |
| nested_loops / chuchu-O2 | 3.430 | 0.423 | 61.158 | 33512 | 1376256 |
| nested_loops / chuchu-O3 | 3.144 | 0.547 | 62.896 | 33512 | 1376256 |
| nested_loops / rust | 3.638 | 0.795 | 141.650 | 471040 | 1572864 |
| nested_loops / c | 3.129 | 0.157 | 84.854 | 33504 | 1376256 |
| nested_loops / python | 47.827 | 3.482 | N/A | N/A | 15089664 |
| array_multiply / chuchu-O0 | 24.453 | 0.373 | 71.046 | 33512 | 1359872 |
| array_multiply / chuchu-O1 | 7.186 | 0.323 | 92.762 | 33432 | 1376256 |
| array_multiply / chuchu-O2 | 4.889 | 0.222 | 93.974 | 33432 | 1376256 |
| array_multiply / chuchu-O3 | 4.682 | 0.429 | 89.530 | 33432 | 1376256 |
| array_multiply / rust | 6.188 | 1.038 | 192.003 | 490608 | 1589248 |
| array_multiply / c | 5.204 | 1.272 | 102.716 | 33568 | 1376256 |
| array_multiply / python | 178.996 | 5.089 | N/A | N/A | 15368192 |
| array_multiply / chuchu-O3-no-vectorize | 4.620 | 0.463 | 100.384 | 33432 | 1376256 |
| array_reduce / chuchu-O0 | 12.981 | 2.264 | 62.332 | 33512 | 1376256 |
| array_reduce / chuchu-O1 | 5.684 | 0.231 | 78.402 | 33512 | 1359872 |
| array_reduce / chuchu-O2 | 5.599 | 0.522 | 81.683 | 33512 | 1376256 |
| array_reduce / chuchu-O3 | 4.957 | 0.562 | 85.172 | 33512 | 1376256 |
| array_reduce / rust | 4.914 | 0.834 | 159.383 | 471040 | 1572864 |
| array_reduce / c | 4.405 | 1.598 | 83.932 | 33776 | 1376256 |
| array_reduce / python | 144.761 | 9.781 | N/A | N/A | 15286272 |
| array_copies / chuchu-O0 | 11.929 | 0.470 | 73.627 | 50024 | 1409024 |
| array_copies / chuchu-O1 | 4.147 | 0.776 | 71.480 | 33432 | 1376256 |
| array_copies / chuchu-O2 | 3.593 | 0.087 | 73.055 | 33432 | 1376256 |
| array_copies / chuchu-O3 | 3.558 | 0.210 | 72.098 | 33432 | 1376256 |
| array_copies / rust | 3.452 | 0.145 | 139.372 | 490608 | 1589248 |
| array_copies / c | 5.341 | 0.183 | 79.185 | 33664 | 1392640 |
| array_copies / python | 46.401 | 2.071 | N/A | N/A | 16138240 |

## Controlled optimization comparison

Ratios below are O0 median / optimized median (>1 favors optimized), not isolated kernel throughput.

| Workload | O0/O1 | O0/O2 | O0/O3 |
|---|---:|---:|---:|
| integer_loop | 1.70x | 1.79x | 1.54x |
| float_loop | 1.29x | 1.34x | 1.52x |
| function_calls | 1.26x | 1.61x | 1.19x |
| recursion | 1.06x | 1.06x | 1.01x |
| nested_loops | 1.36x | 1.30x | 1.42x |
| array_multiply | 3.40x | 5.00x | 5.22x |
| array_reduce | 2.28x | 2.32x | 2.62x |
| array_copies | 2.88x | 3.32x | 3.35x |

## IR inspection

- integer_loop / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=2, loads=5, stores=4, vector arithmetic IR/assembly=0/0.
- integer_loop / chuchu-O1: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- integer_loop / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- integer_loop / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- float_loop / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=2, loads=4, stores=4, vector arithmetic IR/assembly=0/0.
- float_loop / chuchu-O1: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- float_loop / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- float_loop / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- function_calls / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=3, loads=7, stores=5, vector arithmetic IR/assembly=0/0.
- function_calls / chuchu-O1: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- function_calls / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- function_calls / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- recursion / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=1, loads=4, stores=1, vector arithmetic IR/assembly=0/0.
- recursion / chuchu-O1: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- recursion / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- recursion / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- nested_loops / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=3, loads=8, stores=6, vector arithmetic IR/assembly=0/0.
- nested_loops / chuchu-O1: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- nested_loops / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- nested_loops / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- array_multiply / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=18, loads=23, stores=151, vector arithmetic IR/assembly=0/0.
- array_multiply / chuchu-O1: runtime loops/recursion retained; static inputs; allocas=7, loads=389, stores=322, vector arithmetic IR/assembly=0/0.
- array_multiply / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=1, loads=2, stores=32, vector arithmetic IR/assembly=64/64.
- array_multiply / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=64/64.
- array_multiply / chuchu-O3-no-vectorize: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=64/64.
- array_reduce / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=13, loads=17, stores=145, vector arithmetic IR/assembly=0/0.
- array_reduce / chuchu-O1: runtime loops/recursion retained; static inputs; allocas=4, loads=195, stores=193, vector arithmetic IR/assembly=0/0.
- array_reduce / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=4, loads=195, stores=161, vector arithmetic IR/assembly=0/0.
- array_reduce / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=3, loads=194, stores=97, vector arithmetic IR/assembly=0/0.
- array_copies / chuchu-O0: runtime loops/recursion retained; static inputs; allocas=8, loads=11, stores=522, vector arithmetic IR/assembly=0/0.
- array_copies / chuchu-O1: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- array_copies / chuchu-O2: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.
- array_copies / chuchu-O3: runtime loops/recursion retained; static inputs; allocas=0, loads=0, stores=0, vector arithmetic IR/assembly=0/0.

All raw samples, compilation stage timings, machine/toolchain metadata, startup controls, source hashes, memory measurements, and compiler-throughput samples are in the adjacent JSON. Static inputs, process startup, bounds/overflow semantics, and Python allocation/interpreter overhead limit cross-language conclusions. Memory is measured separately with macOS time -l; Python executable size is not comparable and is N/A.
