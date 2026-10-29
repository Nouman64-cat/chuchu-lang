#!/usr/bin/env python3
"""Compile and execute representative examples on the native host."""
from pathlib import Path
import argparse
import os
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--compiler', type=Path, required=True)
    args = parser.parse_args()
    compiler = args.compiler.resolve()
    root = Path(__file__).resolve().parents[2]
    expected = {'main': '35\n', 'numerical': '32\n3\n7\n',
                'syntax_smoke': '25\n100\n4.5\ntrue\n0\n1\n2\n'}
    with tempfile.TemporaryDirectory(prefix='cheeku smoke ') as directory:
        for name, output in expected.items():
            for level in ['-O0', '-O1', '-O2', '-O3']:
                executable = Path(directory) / (name + ('.exe' if os.name == 'nt' else ''))
                subprocess.run([compiler, root / 'examples' / (name + '.chuchu'), level, '-o', executable], check=True)
                result = subprocess.run([executable], capture_output=True, text=True, check=True)
                if result.stdout != output or result.stderr:
                    raise RuntimeError(f'{name} {level}: {result.stdout!r} / {result.stderr!r}')
    print('PASS: 3 examples at O0/O1/O2/O3, 12 native executions')


if __name__ == '__main__':
    main()
