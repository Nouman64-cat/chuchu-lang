# Benchmark results

Execution is wall time per process (ms), including startup; compilation excluded.

| Workload | chuchu-O0 | chuchu-O1 | chuchu-O2 | chuchu-O3 | rust | c | python |
|---|---:|---:|---:|---:|---:|---:|---:|
| integer_loop | 7.228 | 3.618 | 3.899 | 3.513 | 5.310 | 3.905 | 255.861 |
| float_loop | 8.332 | 5.449 | 5.435 | 4.964 | 5.330 | 4.672 | 98.013 |
| function_calls | 4.072 | 2.618 | 2.700 | 2.741 | 3.529 | 3.133 | 62.516 |
| recursion | 4.575 | 3.980 | 3.990 | 3.890 | 4.398 | 3.580 | 49.637 |
| nested_loops | 4.520 | 2.772 | 2.602 | 2.672 | 3.146 | 2.545 | 47.598 |
| array_multiply | 24.075 | 6.406 | 4.603 | 3.553 | 4.425 | 4.017 | 186.255 |
| array_reduce | 12.125 | 4.985 | 4.773 | 4.439 | 3.998 | 3.991 | 140.601 |
| array_copies | 11.740 | 2.759 | 2.760 | 2.719 | 3.467 | 5.238 | 45.447 |

## Individual measurements

| Workload / variant | Median ms | IQR ms | Compile ms | Bytes | Peak RSS bytes |
|---|---:|---:|---:|---:|---:|
| integer_loop / chuchu-O0 | 7.228 | 1.303 | 52.284 | 33512 | 1359872 |
| integer_loop / chuchu-O1 | 3.618 | 0.643 | 64.423 | 33512 | 1376256 |
| integer_loop / chuchu-O2 | 3.899 | 1.096 | 62.668 | 33512 | 1376256 |
| integer_loop / chuchu-O3 | 3.513 | 0.369 | 54.325 | 33512 | 1376256 |
| integer_loop / rust | 5.310 | 4.137 | 92.006 | 471040 | 1556480 |
| integer_loop / c | 3.905 | 0.563 | 57.964 | 33504 | 1376256 |
| integer_loop / python | 255.861 | 9.047 | N/A | N/A | 15138816 |
| float_loop / chuchu-O0 | 8.332 | 0.883 | 52.477 | 33512 | 1376256 |
| float_loop / chuchu-O1 | 5.449 | 0.717 | 52.668 | 33432 | 1359872 |
| float_loop / chuchu-O2 | 5.435 | 0.944 | 50.547 | 33432 | 1359872 |
| float_loop / chuchu-O3 | 4.964 | 0.457 | 53.138 | 33432 | 1359872 |
| float_loop / rust | 5.330 | 1.252 | 105.921 | 490592 | 1589248 |
| float_loop / c | 4.672 | 0.773 | 70.237 | 33440 | 1359872 |
| float_loop / python | 98.013 | 7.625 | N/A | N/A | 15171584 |
| function_calls / chuchu-O0 | 4.072 | 0.816 | 54.570 | 33576 | 1359872 |
| function_calls / chuchu-O1 | 2.618 | 0.341 | 60.636 | 33576 | 1376256 |
| function_calls / chuchu-O2 | 2.700 | 0.691 | 54.713 | 33576 | 1376256 |
| function_calls / chuchu-O3 | 2.741 | 0.350 | 54.599 | 33576 | 1376256 |
| function_calls / rust | 3.529 | 0.706 | 92.162 | 471040 | 1556480 |
| function_calls / c | 3.133 | 0.740 | 60.224 | 33504 | 1376256 |
| function_calls / python | 62.516 | 4.720 | N/A | N/A | 15122432 |
| recursion / chuchu-O0 | 4.575 | 0.789 | 49.681 | 33560 | 1376256 |
| recursion / chuchu-O1 | 3.980 | 0.622 | 51.902 | 33560 | 1376256 |
| recursion / chuchu-O2 | 3.990 | 0.368 | 62.801 | 33560 | 1376256 |
| recursion / chuchu-O3 | 3.890 | 1.669 | 52.747 | 33560 | 1376256 |
| recursion / rust | 4.398 | 1.879 | 96.518 | 471072 | 1556480 |
| recursion / c | 3.580 | 0.947 | 59.411 | 33512 | 1376256 |
| recursion / python | 49.637 | 5.047 | N/A | N/A | 15122432 |
| nested_loops / chuchu-O0 | 4.520 | 1.870 | 57.849 | 33512 | 1376256 |
| nested_loops / chuchu-O1 | 2.772 | 0.857 | 51.262 | 33512 | 1376256 |
| nested_loops / chuchu-O2 | 2.602 | 0.249 | 52.709 | 33512 | 1376256 |
| nested_loops / chuchu-O3 | 2.672 | 0.782 | 55.620 | 33512 | 1376256 |
| nested_loops / rust | 3.146 | 1.794 | 94.827 | 471040 | 1572864 |
| nested_loops / c | 2.545 | 0.376 | 55.941 | 33504 | 1376256 |
| nested_loops / python | 47.598 | 5.397 | N/A | N/A | 15237120 |
| array_multiply / chuchu-O0 | 24.075 | 2.591 | 55.451 | 33512 | 1376256 |
| array_multiply / chuchu-O1 | 6.406 | 1.157 | 100.636 | 33432 | 1376256 |
| array_multiply / chuchu-O2 | 4.603 | 0.740 | 81.162 | 33432 | 1376256 |
| array_multiply / chuchu-O3 | 3.553 | 0.884 | 84.092 | 33432 | 1376256 |
| array_multiply / rust | 4.425 | 0.575 | 115.159 | 490608 | 1589248 |
| array_multiply / c | 4.017 | 1.035 | 75.689 | 33568 | 1376256 |
| array_multiply / python | 186.255 | 72.910 | N/A | N/A | 15450112 |
| array_multiply / chuchu-O3-no-vectorize | 3.419 | 0.416 | 81.663 | 33432 | 1376256 |
| array_reduce / chuchu-O0 | 12.125 | 1.469 | 53.842 | 33512 | 1376256 |
| array_reduce / chuchu-O1 | 4.985 | 0.717 | 71.174 | 33512 | 1359872 |
| array_reduce / chuchu-O2 | 4.773 | 1.340 | 77.501 | 33512 | 1376256 |
| array_reduce / chuchu-O3 | 4.439 | 0.462 | 74.326 | 33512 | 1376256 |
| array_reduce / rust | 3.998 | 0.965 | 109.378 | 471040 | 1572864 |
| array_reduce / c | 3.991 | 0.403 | 97.122 | 33776 | 1376256 |
| array_reduce / python | 140.601 | 1.963 | N/A | N/A | 15269888 |
| array_copies / chuchu-O0 | 11.740 | 0.465 | 72.286 | 50024 | 1409024 |
| array_copies / chuchu-O1 | 2.759 | 0.429 | 66.788 | 33432 | 1376256 |
| array_copies / chuchu-O2 | 2.760 | 0.798 | 61.088 | 33432 | 1376256 |
| array_copies / chuchu-O3 | 2.719 | 0.383 | 67.133 | 33432 | 1376256 |
| array_copies / rust | 3.467 | 2.121 | 96.994 | 490608 | 1589248 |
| array_copies / c | 5.238 | 1.092 | 59.056 | 33664 | 1392640 |
| array_copies / python | 45.447 | 4.871 | N/A | N/A | 16121856 |

## Controlled optimization comparison

Ratios below are O0 median / optimized median (>1 favors optimized), not isolated kernel throughput.

| Workload | O0/O1 | O0/O2 | O0/O3 |
|---|---:|---:|---:|
| integer_loop | 2.00x | 1.85x | 2.06x |
| float_loop | 1.53x | 1.53x | 1.68x |
| function_calls | 1.56x | 1.51x | 1.49x |
| recursion | 1.15x | 1.15x | 1.18x |
| nested_loops | 1.63x | 1.74x | 1.69x |
| array_multiply | 3.76x | 5.23x | 6.78x |
| array_reduce | 2.43x | 2.54x | 2.73x |
| array_copies | 4.25x | 4.25x | 4.32x |

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
