import unittest
import tempfile
import os
from lawkit_python import lawkit

class TestReadmeExamples(unittest.TestCase):
    
    def create_temp_csv(self, content):
        """Helper function to create temporary CSV files"""
        temp_file = tempfile.NamedTemporaryFile(mode='w', suffix='.csv', delete=False)
        temp_file.write(content)
        temp_file.close()
        return temp_file.name
    
    def test_benf_basic_analysis(self):
        """Test case 1: Python library Benford Law analysis"""
        csv_content = "amount\n1234\n5678\n9012\n3456\n7890\n2345\n6789\n1023\n4567\n8901"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('numbers_analyzed', result)
            self.assertGreater(result['numbers_analyzed'], 0)
        finally:
            os.unlink(temp_file)
    
    def test_pareto_sales_analysis(self):
        """Test case 2: Python library Pareto analysis"""
        csv_content = "sales\n1000\n2000\n3000\n4000\n5000\n6000\n7000\n8000\n9000\n10000"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.pareto(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('pareto_ratio', result)
            self.assertGreaterEqual(result['pareto_ratio'], 0.0)
            self.assertLessEqual(result['pareto_ratio'], 1.0)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_all_laws(self):
        """Test case 3: Python library multi-law integration analysis"""
        csv_content = "data\n123\n456\n789\n1011\n1213\n1415\n1617\n1819\n2021\n2223"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.analyze(temp_file, laws=['benf', 'pareto', 'zipf'])
            self.assertIsNotNone(result)
            self.assertIn('laws_executed', result)
            self.assertGreater(result['laws_executed'], 0)
        finally:
            os.unlink(temp_file)
    
    def test_zipf_analysis(self):
        """Test case 4: Python library Zipf law analysis"""
        csv_content = "frequency\n1000\n500\n333\n250\n200\n166\n142\n125\n111\n100"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.zipf(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('numbers_analyzed', result)
            self.assertGreater(result['numbers_analyzed'], 0)
        finally:
            os.unlink(temp_file)
    
    def test_normal_distribution_analysis(self):
        """Test case 5: Python library normal distribution analysis"""
        csv_content = "values\n10\n12\n14\n16\n18\n20\n22\n24\n26\n28"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.normal(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('mean', result)
            self.assertIn('std_dev', result)
        finally:
            os.unlink(temp_file)
    
    def test_poisson_distribution_analysis(self):
        """Test case 6: Python library Poisson distribution analysis"""
        csv_content = "events\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.poisson(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('lambda', result)
            self.assertGreater(result['lambda'], 0)
        finally:
            os.unlink(temp_file)
    
    def test_generate_sample_data(self):
        """Test case 7: Python library generate sample data"""
        result = lawkit.generate('benf', count=100)
        self.assertIsNotNone(result)
        self.assertEqual(len(result), 100)
        self.assertTrue(all(x > 0 for x in result))
    
    def test_validate_data_integrity(self):
        """Test case 8: Python library data validation"""
        csv_content = "amounts\n1234\n5678\n9012\n3456\n7890"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.validate(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('is_valid', result)
            self.assertIsInstance(result['is_valid'], bool)
        finally:
            os.unlink(temp_file)
    
    def test_diagnose_conflicts(self):
        """Test case 9: Python library conflict diagnosis"""
        csv_content = "data\n123\n456\n789\n1011\n1213"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.diagnose(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('conflicts_detected', result)
            self.assertGreaterEqual(result['conflicts_detected'], 0)
        finally:
            os.unlink(temp_file)
    
    def test_list_available_laws(self):
        """Test case 10: Python library list available statistical laws"""
        result = lawkit.list_laws()
        self.assertIsNotNone(result)
        self.assertIsInstance(result, list)
        self.assertIn('benf', result)
        self.assertIn('pareto', result)
    
    def test_selftest_execution(self):
        """Test case 11: Python library self-test for all laws"""
        result = lawkit.selftest()
        self.assertIsNotNone(result)
        self.assertIn('tests_passed', result)
        self.assertGreaterEqual(result['tests_passed'], 0)
    
    def test_risk_assessment(self):
        """Test case 12: Python library risk level assessment"""
        csv_content = "transactions\n1111\n2222\n3333\n4444\n5555"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('risk_level', result)
            self.assertIsInstance(result['risk_level'], str)
        finally:
            os.unlink(temp_file)
    
    def test_statistical_tests(self):
        """Test case 13: Python library statistical test results"""
        csv_content = "amounts\n1000\n2000\n3000\n4000\n5000"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('chi_square', result)
            self.assertIn('p_value', result)
        finally:
            os.unlink(temp_file)
    
    def test_mean_absolute_deviation(self):
        """Test case 14: Python library Mean Absolute Deviation test"""
        csv_content = "numbers\n1234\n5678\n9012\n3456\n7890"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('mean_absolute_deviation', result)
            self.assertGreaterEqual(result['mean_absolute_deviation'], 0.0)
        finally:
            os.unlink(temp_file)
    
    def test_cumulative_distribution(self):
        """Test case 15: Python library cumulative distribution analysis"""
        csv_content = "values\n10\n20\n30\n40\n50\n60\n70\n80\n90\n100"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.pareto(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('cumulative_percentages', result)
            self.assertIsInstance(result['cumulative_percentages'], list)
        finally:
            os.unlink(temp_file)
    
    def test_quality_score_metrics(self):
        """Test case 16: Python library quality score calculation"""
        csv_content = "metrics\n100\n200\n300\n400\n500"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.analyze(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('overall_quality_score', result)
            self.assertGreaterEqual(result['overall_quality_score'], 0.0)
            self.assertLessEqual(result['overall_quality_score'], 1.0)
        finally:
            os.unlink(temp_file)
    
    def test_consistency_score(self):
        """Test case 17: Python library consistency score analysis"""
        csv_content = "consistency\n111\n222\n333\n444\n555"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.analyze(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('consistency_score', result)
            self.assertGreaterEqual(result['consistency_score'], 0.0)
            self.assertLessEqual(result['consistency_score'], 1.0)
        finally:
            os.unlink(temp_file)
    
    def test_lorenz_curve(self):
        """Test case 18: Python library Lorenz curve calculation"""
        csv_content = "wealth\n10000\n20000\n30000\n40000\n50000\n60000\n70000\n80000\n90000\n100000"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.pareto(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('lorenz_curve', result)
            self.assertIsInstance(result['lorenz_curve'], list)
        finally:
            os.unlink(temp_file)
    
    def test_confidence_interval(self):
        """Test case 19: Python library confidence interval calculation"""
        csv_content = "confidence\n100\n200\n300\n400\n500"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file, confidence=0.95)
            self.assertIsNotNone(result)
            self.assertIn('confidence_interval', result)
            self.assertIn('lower_bound', result['confidence_interval'])
            self.assertIn('upper_bound', result['confidence_interval'])
        finally:
            os.unlink(temp_file)
    
    def test_outlier_detection(self):
        """Test case 20: Python library outlier detection"""
        csv_content = "outliers\n100\n200\n300\n400\n500\n999999"  # Last value is outlier
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.analyze(temp_file, detect_outliers=True)
            self.assertIsNotNone(result)
            self.assertIn('outliers_detected', result)
            self.assertGreaterEqual(result['outliers_detected'], 0)
        finally:
            os.unlink(temp_file)
    
    def test_parallel_processing(self):
        """Test case 21: Python library parallel processing"""
        large_data = list(range(1, 10001))
        csv_content = "large_data\n" + "\n".join(map(str, large_data))
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file, parallel=True)
            self.assertIsNotNone(result)
            self.assertGreater(result['numbers_analyzed'], 1000)
        finally:
            os.unlink(temp_file)
    
    def test_memory_optimization(self):
        """Test case 22: Python library memory optimization"""
        csv_content = "memory\n100\n200\n300\n400\n500"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.analyze(temp_file, optimize_memory=True)
            self.assertIsNotNone(result)
            self.assertIn('memory_usage', result)
            self.assertGreater(result['memory_usage'], 0)
        finally:
            os.unlink(temp_file)
    
    def test_streaming_analysis(self):
        """Test case 23: Python library streaming I/O"""
        data_stream = [100, 200, 300, 400, 500]
        result = lawkit.analyze_stream(data_stream)
        self.assertIsNotNone(result)
        self.assertIn('stream_processed', result)
        self.assertEqual(result['stream_processed'], len(data_stream))
    
    def test_time_series_analysis(self):
        """Test case 24: Python library time series analysis"""
        csv_content = "timestamp,value\n1,100\n2,200\n3,300\n4,400\n5,500"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.analyze_timeseries(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('trend', result)
            self.assertIn('seasonality', result)
        finally:
            os.unlink(temp_file)
    
    def test_international_support(self):
        """Test case 25: Python library international number parsing"""
        japanese_numbers = ["１２３４", "５６７８", "９０１２"]
        result = lawkit.parse_international(japanese_numbers, language='japanese')
        self.assertIsNotNone(result)
        self.assertEqual(len(result), 3)
        self.assertTrue(all(x > 0 for x in result))
    
    def test_json_output_format(self):
        """Test case 26: Python library JSON output format"""
        csv_content = "json_test\n123\n456\n789"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file, output_format='json')
            self.assertIsNotNone(result)
            self.assertIsInstance(result, dict)
            self.assertIn('first_digit_distribution', result)
        finally:
            os.unlink(temp_file)
    
    def test_csv_output_format(self):
        """Test case 27: Python library CSV output format"""
        csv_content = "csv_test\n123\n456\n789"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file, output_format='csv')
            self.assertIsNotNone(result)
            self.assertIsInstance(result, str)
            self.assertIn(',', result)  # CSV should contain commas
        finally:
            os.unlink(temp_file)
    
    def test_performance_benchmark(self):
        """Test case 28: Python library performance benchmarking"""
        import time
        
        large_data = list(range(1, 10001))
        csv_content = "benchmark\n" + "\n".join(map(str, large_data))
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            start_time = time.time()
            result = lawkit.benf(temp_file)
            end_time = time.time()
            
            self.assertIsNotNone(result)
            # Should complete within reasonable time (less than 5 seconds)
            self.assertLess(end_time - start_time, 5.0)
        finally:
            os.unlink(temp_file)
    
    def test_error_handling(self):
        """Test case 29: Python library error handling"""
        # Test with invalid file path
        with self.assertRaises(Exception):
            lawkit.benf("/nonexistent/file.csv")
        
        # Test with empty data
        empty_csv = self.create_temp_csv("header\n")
        try:
            with self.assertRaises(Exception):
                lawkit.benf(empty_csv)
        finally:
            os.unlink(empty_csv)

if __name__ == '__main__':
    unittest.main()