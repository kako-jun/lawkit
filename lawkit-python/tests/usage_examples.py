import unittest
import tempfile
import os
from lawkit import LawkitPython

class TestUsageExamples(unittest.TestCase):
    
    def create_temp_csv(self, content):
        """Helper function to create temporary CSV files for testing"""
        fd, temp_path = tempfile.mkstemp(suffix='.csv')
        with os.fdopen(fd, 'w') as f:
            f.write(content)
        return temp_path
    
    def setUp(self):
        self.lawkit = LawkitPython()
        self.temp_files = []
    
    def tearDown(self):
        # Clean up temporary files
        for temp_file in self.temp_files:
            try:
                os.unlink(temp_file)
            except:
                pass
    
    # Test case 1: lawkit benf data.csv (Python equivalent)
    def test_benf_basic(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 2: lawkit benf --verbose --threshold critical data.csv (Python equivalent)
    def test_benf_verbose_threshold_critical(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, verbose=True, threshold='critical')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 3: lawkit benf --format json data.csv (Python equivalent)
    def test_benf_format_json(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 4: lawkit pareto sales_data.csv (Python equivalent)
    def test_pareto_basic(self):
        data = [80000, 12000, 5000, 2000, 1000]
        result = self.lawkit.pareto(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 5: lawkit pareto --verbose --format json revenue.csv (Python equivalent)
    def test_pareto_verbose_format_json(self):
        data = [100000, 50000, 20000, 10000, 5000]
        result = self.lawkit.pareto(data, verbose=True, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 6: lawkit pareto --threshold high customer_values.csv (Python equivalent)
    def test_pareto_threshold_high(self):
        data = [10000, 5000, 3000, 2000, 1000]
        result = self.lawkit.pareto(data, threshold='high')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 7: lawkit zipf data.csv (Python equivalent)
    def test_zipf_basic(self):
        data = [1000, 500, 333, 250, 200]
        result = self.lawkit.zipf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 8: lawkit zipf --verbose city_populations.csv (Python equivalent)
    def test_zipf_verbose(self):
        data = [10000000, 5000000, 3000000, 2000000, 1000000]
        result = self.lawkit.zipf(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 9: lawkit zipf --format json data.csv (Python equivalent)
    def test_zipf_format_json(self):
        data = [1000, 500, 333, 250, 200]
        result = self.lawkit.zipf(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 10: lawkit normal measurements.csv (Python equivalent)
    def test_normal_basic(self):
        data = [1.2, 1.3, 1.1, 1.4, 1.0, 1.5]
        result = self.lawkit.normal(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 11: lawkit normal --verbose data.csv (Python equivalent)
    def test_normal_verbose(self):
        data = [1.2, 1.3, 1.1, 1.4, 1.0, 1.5]
        result = self.lawkit.normal(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 12: lawkit normal --format json data.csv (Python equivalent)
    def test_normal_format_json(self):
        data = [1.2, 1.3, 1.1, 1.4, 1.0, 1.5]
        result = self.lawkit.normal(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 13: lawkit normal --threshold high production_data.csv (Python equivalent)
    def test_normal_threshold_high(self):
        data = [99.1, 99.3, 99.0, 99.4, 98.9, 99.5]
        result = self.lawkit.normal(data, threshold='high')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 14: lawkit poisson event_counts.csv (Python equivalent)
    def test_poisson_basic(self):
        data = [1, 2, 3, 0, 1, 2]
        result = self.lawkit.poisson(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 15: lawkit poisson --verbose data.csv (Python equivalent)
    def test_poisson_verbose(self):
        data = [1, 2, 3, 0, 1, 2]
        result = self.lawkit.poisson(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 16: lawkit poisson --format json incidents.csv (Python equivalent)
    def test_poisson_format_json(self):
        data = [1, 2, 3, 0, 1, 2]
        result = self.lawkit.poisson(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 17: lawkit poisson --threshold high defect_data.csv (Python equivalent)
    def test_poisson_threshold_high(self):
        data = [1, 2, 3, 0, 1, 2]
        result = self.lawkit.poisson(data, threshold='high')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 18-27: Input format tests (simplified for Python)
    def test_various_input_formats(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 28-33: Output format tests
    def test_various_output_formats(self):
        data = [123, 234, 345, 111, 222, 333]
        
        # Test different output formats
        result_text = self.lawkit.benf(data)
        self.assertIsNotNone(result_text)
        
        result_json = self.lawkit.benf(data, output_format='json')
        self.assertIsNotNone(result_json)
        
        result_csv = self.lawkit.pareto([80000, 12000, 5000], output_format='csv')
        self.assertIsNotNone(result_csv)
        
        result_yaml = self.lawkit.normal([1.2, 1.3, 1.1], output_format='yaml')
        self.assertIsNotNone(result_yaml)
    
    # Test case 34-38: Threshold tests
    def test_various_thresholds(self):
        data = [123, 234, 345, 111, 222, 333]
        
        result_low = self.lawkit.benf(data, threshold='low')
        self.assertIsNotNone(result_low)
        
        result_medium = self.lawkit.benf(data, threshold='medium')
        self.assertIsNotNone(result_medium)
        
        result_high = self.lawkit.benf(data, threshold='high')
        self.assertIsNotNone(result_high)
        
        result_critical = self.lawkit.benf(data, threshold='critical')
        self.assertIsNotNone(result_critical)
        
        result_auto = self.lawkit.benf(data, threshold='auto')
        self.assertIsNotNone(result_auto)
    
    # Test case 39-43: Multi-language number support (Python equivalent)
    def test_international_numbers(self):
        # Simplified representation for international numbers
        data = [123456, 1234, 123456, 123456]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 44: lawkit analyze --laws benf,pareto data.csv (Python equivalent)
    def test_analyze_two_laws(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford', 'pareto'])
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 45: lawkit analyze --laws all data.csv (Python equivalent)
    def test_analyze_all_laws(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 46: lawkit analyze --laws benf,pareto,normal --verbose --recommend data.csv (Python equivalent)
    def test_analyze_verbose_recommend(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford', 'pareto', 'normal'], verbose=True, recommend=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 47: lawkit analyze --laws all --focus fraud-detection data.csv (Python equivalent)
    def test_analyze_focus_fraud_detection(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all', focus='fraud-detection')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 48: lawkit analyze --laws all --purpose quality-assessment data.csv (Python equivalent)
    def test_analyze_purpose_quality_assessment(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all', purpose='quality-assessment')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 49: lawkit analyze --laws all --format json data.csv (Python equivalent)
    def test_analyze_format_json(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all', output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 50-52: lawkit generate tests (Python equivalent)
    def test_generate_data(self):
        data = self.lawkit.generate(samples=1000)
        self.assertIsNotNone(data)
        self.assertEqual(len(data), 1000)
        
        # Test analyzing generated data
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 53: lawkit validate --laws all data.csv (Python equivalent)
    def test_validate_all_laws(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.validate(data, laws='all')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 54: lawkit validate --laws benf,pareto --focus fraud-detection data.csv (Python equivalent)
    def test_validate_focus_fraud_detection(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.validate(data, laws=['benford', 'pareto'], focus='fraud-detection')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 55: lawkit validate --laws all --recommend data.csv (Python equivalent)
    def test_validate_recommend(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.validate(data, laws='all', recommend=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 56: lawkit diagnose --laws all data.csv (Python equivalent)
    def test_diagnose_all_laws(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.diagnose(data, laws='all')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 57: lawkit diagnose --laws all --purpose quality-assessment data.csv (Python equivalent)
    def test_diagnose_purpose_quality_assessment(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.diagnose(data, laws='all', purpose='quality-assessment')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 58: lawkit diagnose --laws all --verbose data.csv (Python equivalent)
    def test_diagnose_verbose(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.diagnose(data, laws='all', verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 59-60: lawkit selftest (Python equivalent)
    def test_selftest(self):
        result = self.lawkit.selftest()
        self.assertIsNotNone(result)
        self.assertTrue(result['success'])
        
        result_verbose = self.lawkit.selftest(verbose=True)
        self.assertIsNotNone(result_verbose)
        self.assertTrue(result_verbose['success'])
    
    # Test case 61-66: Use case specific tests (Python equivalent)
    def test_use_cases(self):
        # Financial fraud detection
        financial_data = [1234, 2345, 3456, 1111, 2222, 3333]
        fraud_result = self.lawkit.benf(financial_data, threshold='high')
        self.assertIsNotNone(fraud_result)
        
        pareto_result = self.lawkit.pareto([100000, 50000, 30000, 20000, 10000], verbose=True, output_format='json')
        self.assertIsNotNone(pareto_result)
        
        analysis_result = self.lawkit.analyze(financial_data, laws=['benford', 'pareto'], focus='fraud-detection')
        self.assertIsNotNone(analysis_result)
        
        # Quality control
        quality_data = [99.1, 99.3, 99.0, 99.4, 98.9, 99.5]
        quality_result = self.lawkit.normal(quality_data, threshold='high')
        self.assertIsNotNone(quality_result)
        
        validate_result = self.lawkit.validate(quality_data, laws=['normal', 'poisson'], purpose='quality-control')
        self.assertIsNotNone(validate_result)
        
        # Defect analysis
        defect_data = [1, 2, 0, 1, 3, 0]
        defect_result = self.lawkit.poisson(defect_data, verbose=True)
        self.assertIsNotNone(defect_result)

if __name__ == '__main__':
    unittest.main()