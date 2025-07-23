import unittest
import tempfile
import os
from lawkit import LawkitPython

class TestGettingStartedExamples(unittest.TestCase):
    
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
    
    # Test case 2: lawkit benf data.csv --verbose (Python equivalent)
    def test_benf_verbose(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 3: lawkit benf data.csv --format json (Python equivalent)
    def test_benf_format_json(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 4: lawkit benf data.csv --threshold high (Python equivalent)
    def test_benf_threshold_high(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, threshold='high')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 5: lawkit benf audit_data.csv --confidence 0.99 --verbose (Python equivalent)
    def test_benf_confidence_verbose(self):
        data = [1234, 2345, 3456, 1111, 2222, 3333]
        result = self.lawkit.benf(data, confidence=0.99, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 6: lawkit benf large_data.csv --sample-size 10000 --optimize (Python equivalent)
    def test_benf_sample_size_optimize(self):
        data = [100000, 200000, 300000, 110000, 210000, 310000]
        result = self.lawkit.benf(data, sample_size=10000)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 7: lawkit benf financial_data.csv --min-value 100 (Python equivalent)
    def test_benf_min_value(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, min_value=100)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 8: lawkit pareto sales.csv (Python equivalent)
    def test_pareto_basic(self):
        data = [80000, 12000, 5000, 2000, 1000]
        result = self.lawkit.pareto(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 9: lawkit pareto sales.csv --concentration 0.9 (Python equivalent)
    def test_pareto_concentration(self):
        data = [80000, 12000, 5000, 2000, 1000]
        result = self.lawkit.pareto(data, concentration=0.9)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 10: lawkit pareto sales.csv --gini-coefficient (Python equivalent)
    def test_pareto_gini_coefficient(self):
        data = [80000, 12000, 5000, 2000, 1000]
        result = self.lawkit.pareto(data, gini_coefficient=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 11: lawkit zipf document.txt (Python equivalent)
    def test_zipf_basic(self):
        text_data = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
        result = self.lawkit.zipf(text_data)
        self.assertIsNotNone(result)
        self.assertGreater(result['words_analyzed'], 0)
    
    # Test case 12: lawkit zipf japanese_text.txt (Python equivalent)
    def test_zipf_japanese(self):
        text_data = ["日本語", "テキスト", "分析"]
        result = self.lawkit.zipf(text_data)
        self.assertIsNotNone(result)
        self.assertGreater(result['words_analyzed'], 0)
    
    # Test case 13: lawkit zipf text.txt --min-count 5 (Python equivalent)
    def test_zipf_min_count(self):
        text_data = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
        result = self.lawkit.zipf(text_data, min_count=5)
        self.assertIsNotNone(result)
        self.assertGreater(result['words_analyzed'], 0)
    
    # Test case 14: lawkit normal measurements.csv (Python equivalent)
    def test_normal_basic(self):
        data = [1.2, 1.3, 1.1, 1.4, 1.0, 1.5]
        result = self.lawkit.normal(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 15: lawkit normal quality_data.csv --verbose (Python equivalent)
    def test_normal_verbose(self):
        data = [1.2, 1.3, 1.1, 1.4, 1.0, 1.5]
        result = self.lawkit.normal(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 16: lawkit normal process_data.csv --outliers (Python equivalent)
    def test_normal_outliers(self):
        data = [1.2, 1.3, 1.1, 1.4, 1.0, 1.5]
        result = self.lawkit.normal(data, outliers=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 17: lawkit poisson events.csv (Python equivalent)
    def test_poisson_basic(self):
        data = [1, 2, 3, 0, 1, 2]
        result = self.lawkit.poisson(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 18: lawkit poisson events.csv --verbose (Python equivalent)
    def test_poisson_verbose(self):
        data = [1, 2, 3, 0, 1, 2]
        result = self.lawkit.poisson(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 19: lawkit poisson critical_events.csv --confidence 0.99 --verbose (Python equivalent)
    def test_poisson_confidence_verbose(self):
        data = [1, 2, 3, 0, 1, 2]
        result = self.lawkit.poisson(data, confidence=0.99, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 20: lawkit analyze data.csv --laws benf,pareto,normal (Python equivalent)
    def test_analyze_multi_laws(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford', 'pareto', 'normal'])
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 21: lawkit validate data.csv --laws all (Python equivalent)
    def test_validate_all_laws(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.validate(data, laws='all')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 22: lawkit diagnose data.csv --focus conflict (Python equivalent)
    def test_diagnose_focus_conflict(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.diagnose(data, focus='conflict')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 23: lawkit benf accounting.csv (Python equivalent)
    def test_benf_accounting(self):
        data = [1234, 2345, 3456]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 24: lawkit pareto sales.csv --threshold 0.8 (Python equivalent)
    def test_pareto_threshold(self):
        data = [80000, 12000, 5000, 2000, 1000]
        result = self.lawkit.pareto(data, threshold=0.8)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 25: lawkit benf data.csv (text format default) (Python equivalent)
    def test_benf_text_format(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 26: lawkit benf data.csv --format csv (Python equivalent)
    def test_benf_format_csv(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='csv')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 27: lawkit benf data.csv --format yaml (Python equivalent)
    def test_benf_format_yaml(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='yaml')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 28: lawkit benf data.csv --format xml (Python equivalent)
    def test_benf_format_xml(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, output_format='xml')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 29: echo "１２３４５６ ７８９０" | lawkit benf (Python equivalent)
    def test_benf_japanese_numbers(self):
        data = [123456, 7890]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 30: echo "一千二百三十四" | lawkit benf (Python equivalent)
    def test_benf_chinese_numbers(self):
        data = [1234]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 31: echo "壹萬貳仟參佰肆拾伍" | lawkit benf (Python equivalent)
    def test_benf_traditional_chinese(self):
        data = [12345]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 32: echo "五万六千七百八十九" | lawkit benf (Python equivalent)
    def test_benf_japanese_kanji(self):
        data = [56789]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 33: lawkit benf data.csv --filter ">=1000" (Python equivalent)
    def test_benf_filter(self):
        data = [1000, 2000, 3000, 1100, 2200, 3300]
        result = self.lawkit.benf(data, filter_condition=">=1000")
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 34: lawkit pareto sales.csv --concentration 0.95 (Python equivalent)
    def test_pareto_concentration_95(self):
        data = [80000, 12000, 5000, 2000, 1000]
        result = self.lawkit.pareto(data, concentration=0.95)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)

if __name__ == '__main__':
    unittest.main()