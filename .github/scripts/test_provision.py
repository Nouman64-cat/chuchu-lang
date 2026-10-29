"""Regression tests for SDK activation without downloading a toolchain."""
import importlib.util
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('provision', Path(__file__).with_name('provision-llvm.py'))
provision = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(provision)


class ActivationTests(unittest.TestCase):
    @unittest.skipIf(os.name == 'nt', 'Unix loader paths and symlinks')
    def test_macos_loader_path_exposes_llvm_without_sdk_libcxx(self):
        with tempfile.TemporaryDirectory() as directory:
            prefix = Path(directory) / 'sdk'
            (prefix / 'lib').mkdir(parents=True)
            for name in ['libLLVM.dylib', 'libLLVM.23.1.dylib', 'libc++.1.dylib']:
                (prefix / 'lib' / name).touch()
            env_file, path_file = Path(directory) / 'env', Path(directory) / 'path'
            with patch.dict(os.environ, {'GITHUB_ENV': str(env_file), 'GITHUB_PATH': str(path_file)}, clear=True), \
                    patch.object(provision.platform, 'system', return_value='Darwin'), \
                    patch.object(provision.subprocess, 'check_output', return_value='23.1.4\n'):
                provision.activate(prefix)
                provision.activate(prefix)  # Cached activation is idempotent.
            runtime = prefix / 'chuchu-runtime-lib'
            self.assertEqual(sorted(p.name for p in runtime.iterdir()), ['libLLVM.23.1.dylib', 'libLLVM.dylib'])
            self.assertTrue((runtime / 'libLLVM.dylib').resolve().is_file())
            self.assertIn(f'DYLD_LIBRARY_PATH={runtime}:', env_file.read_text())
            self.assertNotIn(f'DYLD_LIBRARY_PATH={prefix / "lib"}', env_file.read_text())


if __name__ == '__main__':
    unittest.main()
