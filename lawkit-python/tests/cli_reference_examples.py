import unittest
import tempfile
import os
from lawkit import LawkitPython

class TestCliReferenceExamples(unittest.TestCase):
    
    def create_temp_csv(self, content):
        """Helper function to create temporary CSV files for testing"""
        fd, temp_path = tempfile.mkstemp(suffix='.csv')
        with os.fdopen(fd, 'w') as f:
            f.write(content)
        return temp_path
    
    def create_temp_txt(self, content):
        """Helper function to create temporary text files for testing"""
        fd, temp_path = tempfile.mkstemp(suffix='.txt')
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
    
    # Test case 1: lawkit --help (Python equivalent - N/A)
    def test_help(self):
        # Python library doesn't have help - this is CLI only
        self.assertTrue(True)
    
    # Test case 2: lawkit --version (Python equivalent)
    def test_version(self):
        version = self.lawkit.version()
        self.assertEqual(version, "2.0.1")
    
    # Test case 3: lawkit list (Python equivalent - N/A)
    def test_list(self):
        # Python library doesn't have list - this is CLI only
        self.assertTrue(True)
    
    # Test case 4: lawkit benf data.csv (Python equivalent)
    def test_benf_basic(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 5: lawkit benf transactions.json --verbose --format json (Python equivalent)
    def test_benf_verbose_json(self):
        data = [1234, 2345, 3456, 1111, 2222, 3333]
        result = self.lawkit.benf(data, verbose=True, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 6: lawkit benf data.csv --quiet (Python equivalent)
    def test_benf_quiet(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.benf(data, quiet=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 7: lawkit benf accounts.csv --filter ">=1000" --threshold high (Python equivalent)
    def test_benf_filter_threshold(self):
        data = [1000, 2000, 3000, 1100, 2100, 3100]
        result = self.lawkit.benf(data, filter_range=">=1000", threshold="high")
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 8: lawkit benf audit_data.csv --confidence 0.99 --verbose (Python equivalent)
    def test_benf_confidence_verbose(self):
        data = [1234, 2345, 3456, 1111, 2222, 3333]
        result = self.lawkit.benf(data, confidence=0.99, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 9: lawkit benf big_data.csv --sample-size 50000 (Python equivalent)
    def test_benf_sample_size(self):
        data = [10000, 20000, 30000, 11000, 21000, 31000]
        result = self.lawkit.benf(data, sample_size=50000)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 10: lawkit benf financial_data.csv --min-value 100 (Python equivalent)
    def test_benf_min_value(self):
        data = [100, 200, 300, 150, 250, 350]
        result = self.lawkit.benf(data, min_value=100)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 11: echo "123 456 789" | lawkit benf --verbose (Python equivalent)
    def test_benf_stdin_verbose(self):
        data = [123, 456, 789]
        result = self.lawkit.benf(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 12: lawkit pareto sales.csv (Python equivalent)
    def test_pareto_basic(self):
        data = [1000, 500, 300, 200, 100, 50]
        result = self.lawkit.pareto(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 13: lawkit pareto data.csv --concentration 0.9 (Python equivalent)
    def test_pareto_concentration(self):
        data = [1000, 500, 300, 200, 100, 50]
        result = self.lawkit.pareto(data, concentration=0.9)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 14: lawkit pareto customers.csv --business-analysis --gini-coefficient (Python equivalent)
    def test_pareto_business_gini(self):
        data = [1000, 800, 600, 400, 200, 100]
        result = self.lawkit.pareto(data, business_analysis=True, gini_coefficient=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 15: lawkit pareto revenue.csv --percentiles 70,80,90,95 (Python equivalent)
    def test_pareto_percentiles(self):
        data = [10000, 8000, 6000, 4000, 2000, 1000]
        result = self.lawkit.pareto(data, percentiles=[70, 80, 90, 95])
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 16: lawkit zipf frequency_data.csv (Python equivalent)
    def test_zipf_basic(self):
        data = [100, 50, 33, 25, 20, 17]
        result = self.lawkit.zipf(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 17: lawkit zipf text_document.txt --text (Python equivalent)
    def test_zipf_text(self):
        text_data = "the quick brown fox jumps over the lazy dog the"
        result = self.lawkit.zipf_text(text_data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 18: lawkit zipf large_text.txt --text --words 500 (Python equivalent)
    def test_zipf_text_words(self):
        text_data = "the quick brown fox jumps over the lazy dog the fox"
        result = self.lawkit.zipf_text(text_data, max_words=500)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 19: lawkit zipf rankings.csv --verbose (Python equivalent)
    def test_zipf_verbose(self):
        data = [100, 50, 33, 25, 20, 17]
        result = self.lawkit.zipf(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 20: lawkit zipf data.csv --format json (Python equivalent)
    def test_zipf_json(self):
        data = [100, 50, 33, 25, 20, 17]
        result = self.lawkit.zipf(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 21: lawkit normal data.csv (Python equivalent)
    def test_normal_basic(self):
        data = [50, 51, 49, 52, 48, 50]
        result = self.lawkit.normal(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 22: lawkit normal data.csv --test shapiro (Python equivalent)
    def test_normal_test_shapiro(self):
        data = [50, 51, 49, 52, 48, 50]
        result = self.lawkit.normal(data, test_method='shapiro')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 23: lawkit normal data.csv --outliers --outlier-method lof (Python equivalent)
    def test_normal_outliers_lof(self):
        data = [50, 51, 49, 52, 48, 100]
        result = self.lawkit.normal(data, outliers=True, outlier_method='lof')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 24: lawkit normal production_data.csv --quality-control --spec-limits 9.5,10.5 (Python equivalent)
    def test_normal_quality_control(self):
        data = [9.8, 10.1, 9.9, 10.2, 9.7, 10.0]
        result = self.lawkit.normal(data, quality_control=True, spec_limits=(9.5, 10.5))
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 25: lawkit normal timeseries_data.csv --enable-timeseries --timeseries-window 20 (Python equivalent)
    def test_normal_timeseries(self):
        data = [50, 51, 49, 52, 48, 50, 51, 49]
        result = self.lawkit.normal(data, enable_timeseries=True, timeseries_window=20)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 26: lawkit normal measurements.csv --verbose (Python equivalent)
    def test_normal_verbose(self):
        data = [50.1, 49.9, 50.2, 49.8, 50.0, 50.1]
        result = self.lawkit.normal(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 27: lawkit normal quality_data.csv --format json (Python equivalent)
    def test_normal_json(self):
        data = [50, 51, 49, 52, 48, 50]
        result = self.lawkit.normal(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 28: lawkit poisson events.csv (Python equivalent)
    def test_poisson_basic(self):
        data = [3, 2, 4, 1, 3, 2]
        result = self.lawkit.poisson(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 29: lawkit poisson data.csv --test chi_square (Python equivalent)
    def test_poisson_test_chi_square(self):
        data = [3, 2, 4, 1, 3, 2]
        result = self.lawkit.poisson(data, test_method='chi_square')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 30: lawkit poisson server_logs.csv --predict --max-events 50 (Python equivalent)
    def test_poisson_predict(self):
        data = [5, 3, 7, 2, 4, 6]
        result = self.lawkit.poisson(data, predict=True, max_events=50)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 31: lawkit poisson rare_events.csv --rare-events (Python equivalent)
    def test_poisson_rare_events(self):
        data = [0, 1, 0, 0, 2, 0]
        result = self.lawkit.poisson(data, rare_events=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 32: lawkit poisson incidents.csv --verbose (Python equivalent)
    def test_poisson_verbose(self):
        data = [2, 3, 1, 4, 2, 3]
        result = self.lawkit.poisson(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 33: lawkit poisson data.csv --format json (Python equivalent)
    def test_poisson_json(self):
        data = [3, 2, 4, 1, 3, 2]
        result = self.lawkit.poisson(data, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 34: lawkit poisson server_errors.csv --confidence 0.99 --verbose (Python equivalent)
    def test_poisson_confidence_verbose(self):
        data = [1, 0, 2, 1, 0, 3]
        result = self.lawkit.poisson(data, confidence=0.99, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 35: lawkit generate benf --samples 5000 (Python equivalent - N/A)
    def test_generate_benf(self):
        # Python library doesn't have generate - this is CLI only
        self.assertTrue(True)
    
    # Test case 36: lawkit generate benf --samples 2000 --fraud-rate 0.1 (Python equivalent - N/A)
    def test_generate_benf_fraud(self):
        # Python library doesn't have generate - this is CLI only
        self.assertTrue(True)
    
    # Test case 37: lawkit generate benf --samples 1000 --seed 42 --range 1,50000 (Python equivalent - N/A)
    def test_generate_benf_seed_range(self):
        # Python library doesn't have generate - this is CLI only
        self.assertTrue(True)
    
    # Test case 38: lawkit generate normal --samples 1000 --output-file test_data.csv (Python equivalent - N/A)
    def test_generate_normal_output_file(self):
        # Python library doesn't have generate - this is CLI only
        self.assertTrue(True)
    
    # Test case 39: lawkit analyze data.csv (Python equivalent)
    def test_analyze_basic(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 40: lawkit analyze transactions.csv --purpose fraud --recommend (Python equivalent)
    def test_analyze_fraud_recommend(self):
        data = [1234, 2345, 3456, 1111, 2222]
        result = self.lawkit.analyze(data, purpose='fraud', recommend=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 41: lawkit analyze data.csv --laws benf,normal --focus quality (Python equivalent)
    def test_analyze_laws_focus(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, laws=['benford', 'normal'], focus='quality')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 42: lawkit analyze dataset.csv --verbose --format json (Python equivalent)
    def test_analyze_verbose_json(self):
        data = [123, 234, 345, 111, 222, 333]
        result = self.lawkit.analyze(data, verbose=True, output_format='json')
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 43: lawkit validate data.csv (Python equivalent - N/A)
    def test_validate_basic(self):
        # Python library doesn't have validate - this is CLI only
        self.assertTrue(True)
    
    # Test case 44: lawkit diagnose data.csv (Python equivalent - N/A)
    def test_diagnose_basic(self):
        # Python library doesn't have diagnose - this is CLI only
        self.assertTrue(True)
    
    # Test case 45: lawkit benf transactions.csv --verbose (Python equivalent)
    def test_benf_transactions_verbose(self):
        data = [1234, 2345, 3456, 1111, 2222]
        result = self.lawkit.benf(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 46: lawkit analyze suspicious_data.csv --purpose fraud --recommend (Python equivalent)
    def test_analyze_suspicious_fraud(self):
        data = [1111, 2222, 3333, 4444, 5555]
        result = self.lawkit.analyze(data, purpose='fraud', recommend=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 47: lawkit analyze dataset.csv --purpose quality --verbose (Python equivalent)
    def test_analyze_quality_verbose(self):
        data = [50, 51, 49, 52, 48, 50]
        result = self.lawkit.analyze(data, purpose='quality', verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['laws_executed'], 0)
    
    # Test case 48: lawkit normal dataset.csv --verbose (Python equivalent)
    def test_normal_dataset_verbose(self):
        data = [50, 51, 49, 52, 48, 50]
        result = self.lawkit.normal(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 49: lawkit pareto sales.csv --threshold 0.8 (Python equivalent)
    def test_pareto_sales_threshold(self):
        data = [1000, 800, 600, 400, 200, 100]
        result = self.lawkit.pareto(data, threshold=0.8)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)
    
    # Test case 50: lawkit zipf customer_frequency.csv --verbose (Python equivalent)
    def test_zipf_customer_verbose(self):
        data = [100, 50, 33, 25, 20]
        result = self.lawkit.zipf(data, verbose=True)
        self.assertIsNotNone(result)
        self.assertGreater(result['numbers_analyzed'], 0)

if __name__ == '__main__':
    unittest.main()