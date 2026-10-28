# Benchmark results

Execution is wall time per process (ms), including startup; compilation excluded.

| Workload | chuchu-O0 | chuchu-O1 | chuchu-O2 | chuchu-O3 | rust | c | python |
|---|---:|---:|---:|---:|---:|---:|---:|
| integer_loop | 8.001 | 4.365 | 5.034 | 4.756 | 4.658 | 4.712 | 261.405 |
| float_loop | 8.739 | 6.082 | 5.905 | 5.465 | 5.896 | 5.337 | 91.509 |
| function_calls | 4.155 | 3.630 | 3.626 | 3.447 | 3.575 | 3.578 | 61.651 |
| recursion | 5.686 | 4.709 | 4.306 | 4.517 | 4.820 | 4.180 | 51.523 |
| nested_loops | 3.947 | 3.316 | 3.570 | 4.050 | 3.414 | 3.489 | 43.209 |
| array_multiply | 23.813 | 6.940 | 4.792 | 4.438 | 4.754 | 5.013 | 183.108 |
| array_reduce | 12.470 | 5.749 | 5.261 | 5.205 | 4.624 | 4.360 | 142.902 |
| array_copies | 12.531 | 3.290 | 3.387 | 3.505 | 3.483 | 5.341 | 48.464 |

## Individual measurements

| Workload / variant | Median ms | IQR ms | Compile ms | Bytes | Peak RSS bytes |
|---|---:|---:|---:|---:|---:|
| integer_loop / chuchu-O0 | 8.001 | 1.492 | 70.871 | 33512 | 1359872 |
| integer_loop / chuchu-O1 | 4.365 | 0.410 | 62.007 | 33512 | 1376256 |
| integer_loop / chuchu-O2 | 5.034 | 1.076 | 65.768 | 33512 | 1376256 |
| integer_loop / chuchu-O3 | 4.756 | 0.855 | 62.817 | 33512 | 1376256 |
| integer_loop / rust | 4.658 | 0.886 | 142.543 | 471040 | 1556480 |
| integer_loop / c | 4.712 | 1.256 | 91.441 | 33504 | 1376256 |
| integer_loop / python | 261.405 | 10.132 | N/A | N/A | 15106048 |
| float_loop / chuchu-O0 | 8.739 | 0.707 | 58.655 | 33512 | 1376256 |
| float_loop / chuchu-O1 | 6.082 | 0.603 | 64.824 | 33432 | 1359872 |
| float_loop / chuchu-O2 | 5.905 | 0.914 | 59.891 | 33432 | 1359872 |
| float_loop / chuchu-O3 | 5.465 | 0.379 | 59.500 | 33432 | 1376256 |
| float_loop / rust | 5.896 | 0.800 | 138.838 | 490592 | 1589248 |
| float_loop / c | 5.337 | 0.492 | 76.386 | 33440 | 1359872 |
| float_loop / python | 91.509 | 6.328 | N/A | N/A | 15106048 |
| function_calls / chuchu-O0 | 4.155 | 0.179 | 63.764 | 33576 | 1359872 |
| function_calls / chuchu-O1 | 3.630 | 0.407 | 75.165 | 33576 | 1376256 |
| function_calls / chuchu-O2 | 3.626 | 0.170 | 62.683 | 33576 | 1376256 |
| function_calls / chuchu-O3 | 3.447 | 0.633 | 61.870 | 33576 | 1376256 |
| function_calls / rust | 3.575 | 0.665 | 136.796 | 471040 | 1556480 |
| function_calls / c | 3.578 | 0.392 | 79.669 | 33504 | 1376256 |
| function_calls / python | 61.651 | 1.406 | N/A | N/A | 15171584 |
| recursion / chuchu-O0 | 5.686 | 0.711 | 72.639 | 33560 | 1376256 |
| recursion / chuchu-O1 | 4.709 | 0.570 | 63.468 | 33560 | 1376256 |
| recursion / chuchu-O2 | 4.306 | 0.388 | 62.430 | 33560 | 1376256 |
| recursion / chuchu-O3 | 4.517 | 0.686 | 61.480 | 33560 | 1376256 |
| recursion / rust | 4.820 | 1.203 | 144.956 | 471072 | 1556480 |
| recursion / c | 4.180 | 0.776 | 83.308 | 33512 | 1376256 |
| recursion / python | 51.523 | 4.462 | N/A | N/A | 15155200 |
| nested_loops / chuchu-O0 | 3.947 | 0.459 | 61.731 | 33512 | 1376256 |
| nested_loops / chuchu-O1 | 3.316 | 0.332 | 65.914 | 33512 | 1376256 |
| nested_loops / chuchu-O2 | 3.570 | 0.836 | 65.004 | 33512 | 1376256 |
| nested_loops / chuchu-O3 | 4.050 | 1.458 | 60.362 | 33512 | 1376256 |
| nested_loops / rust | 3.414 | 0.104 | 148.906 | 471040 | 1572864 |
| nested_loops / c | 3.489 | 0.431 | 75.092 | 33504 | 1376256 |
| nested_loops / python | 43.209 | 2.639 | N/A | N/A | 15138816 |
| array_multiply / chuchu-O0 | 23.813 | 1.240 | 64.654 | 33512 | 1359872 |
| array_multiply / chuchu-O1 | 6.940 | 0.939 | 83.336 | 33432 | 1376256 |
| array_multiply / chuchu-O2 | 4.792 | 0.491 | 85.724 | 33432 | 1376256 |
| array_multiply / chuchu-O3 | 4.438 | 0.427 | 113.560 | 33432 | 1376256 |
| array_multiply / rust | 4.754 | 0.911 | 155.851 | 490608 | 1589248 |
| array_multiply / c | 5.013 | 0.435 | 83.018 | 33568 | 1376256 |
| array_multiply / python | 183.108 | 9.087 | N/A | N/A | 15433728 |
| array_multiply / chuchu-O3-no-vectorize | 4.213 | 0.294 | 94.947 | 33432 | 1392640 |
| array_reduce / chuchu-O0 | 12.470 | 0.741 | 62.978 | 33512 | 1376256 |
| array_reduce / chuchu-O1 | 5.749 | 0.639 | 81.078 | 33512 | 1359872 |
| array_reduce / chuchu-O2 | 5.261 | 0.227 | 86.785 | 33512 | 1376256 |
| array_reduce / chuchu-O3 | 5.205 | 0.975 | 90.864 | 33512 | 1376256 |
| array_reduce / rust | 4.624 | 0.254 | 172.209 | 471040 | 1572864 |
| array_reduce / c | 4.360 | 0.279 | 78.250 | 33776 | 1376256 |
| array_reduce / python | 142.902 | 10.366 | N/A | N/A | 15302656 |
| array_copies / chuchu-O0 | 12.531 | 0.922 | 67.987 | 50024 | 1409024 |
| array_copies / chuchu-O1 | 3.290 | 0.450 | 70.786 | 33432 | 1376256 |
| array_copies / chuchu-O2 | 3.387 | 0.584 | 75.375 | 33432 | 1376256 |
| array_copies / chuchu-O3 | 3.505 | 0.675 | 71.829 | 33432 | 1376256 |
| array_copies / rust | 3.483 | 0.425 | 144.734 | 490608 | 1589248 |
| array_copies / c | 5.341 | 0.423 | 72.878 | 33664 | 1392640 |
| array_copies / python | 48.464 | 4.257 | N/A | N/A | 16056320 |

## Controlled optimization comparison

Ratios below are O0 median / optimized median (>1 favors optimized), not isolated kernel throughput.

| Workload | O0/O1 | O0/O2 | O0/O3 |
|---|---:|---:|---:|
| integer_loop | 1.83x | 1.59x | 1.68x |
| float_loop | 1.44x | 1.48x | 1.60x |
| function_calls | 1.14x | 1.15x | 1.21x |
| recursion | 1.21x | 1.32x | 1.26x |
| nested_loops | 1.19x | 1.11x | 0.97x |
| array_multiply | 3.43x | 4.97x | 5.37x |
| array_reduce | 2.17x | 2.37x | 2.40x |
| array_copies | 3.81x | 3.70x | 3.58x |

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
