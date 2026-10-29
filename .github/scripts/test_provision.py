"""Regression tests for SDK activation without downloading a toolchain."""
import importlib.util
import os
from pathlib import Path
import tempfile
import tomllib
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('provision', Path(__file__).with_name('provision-llvm.py'))
provision = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(provision)


class ActivationTests(unittest.TestCase):
    @unittest.skipIf(os.name == 'nt', 'Unix loader paths')
    def test_macos_never_exports_a_global_loader_path(self):
        with tempfile.TemporaryDirectory() as directory:
            prefix = Path(directory) / 'sdk'
            (prefix / 'lib').mkdir(parents=True)
            (prefix / 'lib' / 'libLLVM.dylib').touch()
            env_file, path_file = Path(directory) / 'env', Path(directory) / 'path'
            with patch.dict(os.environ, {'GITHUB_ENV': str(env_file), 'GITHUB_PATH': str(path_file)}, clear=True), \
                    patch.object(provision.platform, 'system', return_value='Darwin'), \
                    patch.object(provision.subprocess, 'check_output', return_value='23.1.4\n'):
                provision.activate(prefix)
                provision.activate(prefix)  # Cached activation is idempotent.
            self.assertNotIn('DYLD', env_file.read_text())
            self.assertIn(f'LLVM_SYS_231_PREFIX={prefix}', env_file.read_text())

    @unittest.skipIf(os.name == 'nt', 'Unix paths')
    def test_macos_rejects_sdk_without_shared_llvm(self):
        with tempfile.TemporaryDirectory() as directory:
            prefix = Path(directory) / 'sdk'
            (prefix / 'lib').mkdir(parents=True)
            with patch.dict(os.environ, {}, clear=True), \
                    patch.object(provision.platform, 'system', return_value='Darwin'), \
                    patch.object(provision.subprocess, 'check_output', return_value='23.1.4\n'):
                with self.assertRaisesRegex(RuntimeError, 'libLLVM.dylib'):
                    provision.activate(prefix)

    def test_each_source_build_enables_only_its_host_backend(self):
        self.assertEqual(provision.SOURCE_BUILDS[('Darwin', 'arm64')], 'AArch64')
        self.assertEqual(provision.SOURCE_BUILDS[('Darwin', 'x86_64')], 'X86')
        self.assertEqual(provision.SOURCE_BUILDS[('Windows', 'amd64')], 'X86')
        self.assertNotIn(('Linux', 'x86_64'), provision.SOURCE_BUILDS)


class CargoFeatureTests(unittest.TestCase):
    def test_inkwell_does_not_enable_every_llvm_backend(self):
        root = Path(__file__).resolve().parents[2]
        manifest = tomllib.loads((root / 'Cargo.toml').read_text())
        self.assertIs(manifest['dependencies']['inkwell']['default-features'], False)
        for arch, feature in (('x86_64', 'target-x86'), ('aarch64', 'target-aarch64')):
            dep = manifest['target'][f'cfg(target_arch = "{arch}")']['dependencies']['inkwell']
            self.assertEqual(dep['features'], [feature])
            self.assertIs(dep['default-features'], False)


if __name__ == '__main__':
    unittest.main()
