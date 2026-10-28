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

if __name__=='__main__': unittest.main()
