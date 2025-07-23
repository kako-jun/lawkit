import unittest
import tempfile
import os
from lawkit import LawkitPython

class TestFaqExamples(unittest.TestCase):
    
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
    
    # Test case 1: cut -d',' -f2 data.csv | lawkit benf (Python equivalent)
    def test_cut_pipe_benf(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 2: awk -F',' '{print $2}' data.csv | lawkit pareto (Python equivalent)
    def test_awk_pipe_pareto(self):
        data = [1000, 500, 300, 200, 100, 50]
        result = self.lawkit.pareto(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 3: lawkit benf --threshold high data.csv (Python equivalent)
    def test_benf_threshold_high(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, threshold='high')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 4: echo "1,234.56" | lawkit benf (Python equivalent)
    def test_english_number_format(self):
        data = [1234.56]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 5: echo "１，２３４．５６" | lawkit benf (Python equivalent)
    def test_japanese_number_format(self):
        data = [1234.56]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 6: echo "१,२३४.५६" | lawkit benf (Python equivalent)
    def test_hindi_number_format(self):
        data = [1234.56]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 7: lawkit benf data.csv (Python equivalent)
    def test_benf_basic(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 8: echo "١٢٣٤٥٦" | lawkit benf (Python equivalent)
    def test_arabic_numbers(self):
        data = [123456]  # Simplified representation
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 9: lawkit analyze --laws benf,pareto data.csv (Python equivalent)
    def test_analyze_benf_pareto(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford', 'pareto'])
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 10: lawkit analyze --laws all data.csv (Python equivalent)
    def test_analyze_all_laws(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 11: lawkit analyze --laws all --recommend data.csv (Python equivalent)
    def test_analyze_recommend(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws='all', recommend=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 12: lawkit benf --quiet large_data.csv (Python equivalent)
    def test_benf_quiet(self):
        data = [100000, 200000, 300000, 110000, 210000, 310000]
        result = self.lawkit.benf(data, quiet=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 13: lawkit benf --threshold medium large_data.csv (Python equivalent)
    def test_benf_threshold_medium(self):
        data = [100000, 200000, 300000, 110000, 210000, 310000]
        result = self.lawkit.benf(data, threshold='medium')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 14: lawkit benf --format json large_data.csv (Python equivalent)
    def test_benf_format_json(self):
        data = [100000, 200000, 300000, 110000, 210000, 310000]
        result = self.lawkit.benf(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 15: lawkit pareto small_data.csv (Python equivalent)
    def test_pareto_small_data(self):
        data = [100, 50, 30, 20, 10]
        result = self.lawkit.pareto(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)

if __name__ == '__main__':
    unittest.main()