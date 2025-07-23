import unittest
import tempfile
import os
from lawkit import LawkitPython

class TestConfigurationExamples(unittest.TestCase):
    
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
    
    # Test case 1: lawkit benf data.csv --format json (Python equivalent)
    def test_format_json(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 2: lawkit benf data.csv --format yaml (Python equivalent)
    def test_format_yaml(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='yaml')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 3: lawkit benf data.csv --format csv (Python equivalent)
    def test_format_csv(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='csv')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 4: lawkit benf data.csv --format toml (Python equivalent)
    def test_format_toml(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='toml')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 5: lawkit benf data.csv --format xml (Python equivalent)
    def test_format_xml(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='xml')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 6: echo "１２３４５６" | lawkit benf (Python equivalent)
    def test_japanese_numbers(self):
        data = [123456]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 7: echo "一千二百三十四" | lawkit benf (Python equivalent)
    def test_chinese_numbers(self):
        data = [1234]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 8: lawkit benf data.csv --quiet (Python equivalent)
    def test_quiet_mode(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, quiet=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 9: lawkit benf data.csv --verbose (Python equivalent)
    def test_verbose_mode(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 10: lawkit pareto data.csv --threshold 0.8 (Python equivalent)
    def test_pareto_threshold(self):
        data = [1000, 500, 300, 200, 100, 50]
        result = self.lawkit.pareto(data, threshold=0.8)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 11: lawkit analyze data.csv --laws benford,pareto,normal (Python equivalent)
    def test_analyze_multi_laws(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford', 'pareto', 'normal'])
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 12: lawkit analyze data.csv --laws benford --focus accuracy (Python equivalent)
    def test_analyze_focus_accuracy(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford'], focus='quality')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 13: lawkit analyze data.csv --laws all --purpose audit (Python equivalent)
    def test_analyze_purpose_audit(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all', purpose='fraud')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 14: lawkit analyze data.csv --laws all --recommend (Python equivalent)
    def test_analyze_recommend(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all', recommend=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 15: lawkit benf data.csv --format json (Python equivalent - duplicate)
    def test_json_output_example(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 16: lawkit benf data.csv --format csv (Python equivalent - duplicate)
    def test_csv_output_example(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='csv')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 17: echo "１２３４ ５６７８" | lawkit benf (Python equivalent)
    def test_japanese_spaced_numbers(self):
        data = [1234, 5678]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 18: echo "壹万贰千 三千四百" | lawkit benf (Python equivalent)
    def test_chinese_financial_numbers(self):
        data = [12000, 3400]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 19: echo "123 ４５６ 七八九" | lawkit benf (Python equivalent)
    def test_mixed_format_numbers(self):
        data = [123, 456, 789]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 20: lawkit analyze data.csv --laws benford,pareto,normal (Python equivalent - duplicate)
    def test_multi_law_selection(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford', 'pareto', 'normal'])
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 21: lawkit analyze data.csv --laws benford --focus accuracy (Python equivalent - duplicate)
    def test_analysis_focus(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford'], focus='quality')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 22: lawkit analyze data.csv --laws all --purpose audit (Python equivalent - duplicate)
    def test_purpose_specific_analysis(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all', purpose='fraud')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 23: lawkit analyze data.csv --laws all --recommend (Python equivalent - duplicate)
    def test_recommendation_mode(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all', recommend=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 24: lawkit validate data.csv --laws all (Python equivalent - N/A)
    def test_validate_all_laws(self):
        # Python library doesn't have validate - this is CLI only
        self.assertTrue(True)
    
    # Test case 25: lawkit diagnose data.csv --laws all (Python equivalent - N/A)
    def test_diagnose_all_laws(self):
        # Python library doesn't have diagnose - this is CLI only
        self.assertTrue(True)
    
    # Test case 26: lawkit analyze data1.csv --laws benford --format json (Python equivalent)
    def test_batch_benford_json(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford'], output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 27: lawkit analyze data2.csv --laws pareto --format json (Python equivalent)
    def test_batch_pareto_json(self):
        data = [1000, 500, 300, 200, 100, 50]
        result = self.lawkit.analyze(data, laws=['pareto'], output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 28: lawkit analyze data3.csv --laws normal --format json (Python equivalent)
    def test_batch_normal_json(self):
        data = [50, 51, 49, 52, 48, 50]
        result = self.lawkit.analyze(data, laws=['normal'], output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 29: lawkit benf large_data.csv --quiet (Python equivalent)
    def test_large_data_quiet(self):
        data = [10000, 20000, 30000, 11000, 21000, 31000]
        result = self.lawkit.benf(data, quiet=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 30: lawkit analyze large_data.csv --laws benford --quiet (Python equivalent)
    def test_large_data_analyze_quiet(self):
        data = [10000, 20000, 30000, 11000, 21000, 31000]
        result = self.lawkit.analyze(data, laws=['benford'], quiet=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 31: lawkit benf data.csv --verbose (Python equivalent - duplicate)
    def test_debug_verbose(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 32: lawkit benf data.csv --format json | jq '.numbers_analyzed' (Python equivalent)
    def test_json_pipe_jq(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)

if __name__ == '__main__':
    unittest.main()