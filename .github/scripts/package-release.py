#!/usr/bin/env python3
"""Package a tested native compiler. LLVM is not bundled; see runtime requirements."""
import argparse
from pathlib import Path
import platform
import os
import shutil
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--target', required=True)
    args = parser.parse_args()
    host = subprocess.check_output(['rustc', '-vV'], text=True).split('host: ')[1].splitlines()[0]
    if host != args.target:
        raise RuntimeError(f'Cannot label {host} as {args.target}')
    labels = {'aarch64-apple-darwin': 'macos-arm64', 'x86_64-apple-darwin': 'macos-x64',
              'x86_64-unknown-linux-gnu': 'linux-x64', 'x86_64-pc-windows-msvc': 'windows-x64'}
    name = 'cheeku-' + labels[host]
    root = Path(__file__).resolve().parents[2]
    target_dir = Path(os.environ.get('CARGO_TARGET_DIR', root / 'target')).resolve()
    binary = target_dir / 'release' / ('cheeku.exe' if platform.system() == 'Windows' else 'cheeku')
    subprocess.run([binary, '--help'], check=True)
    output = target_dir / 'distributions'
    output.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory() as temp:
        package = Path(temp) / name
        package.mkdir()
        shutil.copy2(binary, package / binary.name)
        shutil.copy2(root / 'LICENSE', package / 'LICENSE')
        shutil.copy2(root / 'docs/release-runtime.md', package / 'RUNTIME-REQUIREMENTS.md')
        shutil.copytree(root / 'examples', package / 'examples')
        archive = shutil.make_archive(str(output / name), 'zip' if platform.system() == 'Windows' else 'gztar', temp, name)
    print(archive)


if __name__ == '__main__':
    main()
