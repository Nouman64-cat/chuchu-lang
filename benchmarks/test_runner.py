import unittest
from compare import compare
from run import summary

class RegressionTests(unittest.TestCase):
    def dataset(self, samples):
        return dict(environment={},source_hashes={},config={},results=[dict(workload='loop',variant='chuchu-O2',execution=summary(samples))])
    def test_regression_and_improvement_direction(self):
        baseline=self.dataset([1.0,1.01,1.02,1.03,1.04])
        self.assertEqual(compare(baseline,self.dataset([2.0,2.01,2.02]))['results'][0]['status'],'possible regression')
        self.assertEqual(compare(baseline,self.dataset([0.5,0.51,0.52]))['results'][0]['status'],'faster')
    def test_overlap_and_noise_are_not_strict_failures(self):
        baseline=self.dataset([1.0,1.01,1.02])
        self.assertEqual(compare(baseline,baseline)['results'][0]['status'],'unchanged/overlapping intervals')
        self.assertEqual(compare(baseline,self.dataset([0.2,1.0,3.0]))['results'][0]['status'],'unreliable/noisy')
    def test_different_sources_are_incomparable(self):
        before=self.dataset([1.0,1.0,1.0]);after=self.dataset([2.0,2.0,2.0]);after['source_hashes']={'changed':'source'}
        self.assertEqual(compare(before,after)['results'][0]['status'],'incomparable or missing baseline')


class PortabilityTests(unittest.TestCase):
    def test_executable_paths_are_platform_specific(self):
        from unittest.mock import patch
        from pathlib import Path
        from run import executable_path
        with patch('run.sys.platform', 'win32'):
            self.assertEqual(executable_path(Path('program with spaces')), Path('program with spaces.exe'))
        with patch('run.sys.platform', 'linux'):
            self.assertEqual(executable_path(Path('program')), Path('program'))

    def test_linux_metadata_does_not_invoke_macos_utilities(self):
        from unittest.mock import patch
        from run import machine_metadata
        with patch('run.sys.platform', 'linux'), patch('run.version', return_value=None) as called:
            result = machine_metadata()
        self.assertEqual(result['llvm'], None)
        names = [str(args.args[0][0]) for args in called.call_args_list]
        self.assertFalse(set(names) & {'brew', 'sysctl', 'sw_vers'})

    def test_memory_is_unavailable_off_macos(self):
        from unittest.mock import patch
        from run import memory_run
        with patch('run.sys.platform', 'win32'), patch('run.command') as command:
            self.assertIsNone(memory_run(['program.exe']))
            command.assert_not_called()

if __name__=='__main__': unittest.main()
