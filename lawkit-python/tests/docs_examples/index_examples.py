import unittest
import tempfile
import os
from lawkit_python import lawkit

class TestIndexExamples(unittest.TestCase):
    
    def create_temp_csv(self, content):
        """Helper function to create temporary CSV files"""
        temp_file = tempfile.NamedTemporaryFile(mode='w', suffix='.csv', delete=False)
        temp_file.write(content)
        temp_file.close()
        return temp_file.name
    
    def test_benf_data_csv(self):
        """Test case 1: Python library Benford law analysis from index.md"""
        csv_content = "value\n1234\n5678\n9012\n3456\n7890\n2345\n6789\n1023\n4567\n8901"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.benf(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('numbers_analyzed', result)
            self.assertGreater(result['numbers_analyzed'], 0)
        finally:
            os.unlink(temp_file)
    
    def test_pareto_sales_csv(self):
        """Test case 2: Python library Pareto analysis from index.md"""
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
    
    def test_analyze_multi_laws(self):
        """Test case 3: Python library multi-law comparison from index.md"""
        csv_content = "data\n123\n456\n789\n1011\n1213\n1415\n1617\n1819\n2021\n2223"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.analyze(temp_file, laws=['benf', 'pareto', 'normal'])
            self.assertIsNotNone(result)
            self.assertIn('laws_executed', result)
            self.assertGreaterEqual(result['laws_executed'], 3)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_data_csv(self):
        """Test case 4: Python library basic analyze from index.md"""
        csv_content = "numbers\n100\n200\n300\n400\n500\n600\n700\n800\n900\n1000"
        temp_file = self.create_temp_csv(csv_content)
        
        try:
            result = lawkit.analyze(temp_file)
            self.assertIsNotNone(result)
            self.assertIn('numbers_analyzed', result)
            self.assertGreater(result['numbers_analyzed'], 0)
        finally:
            os.unlink(temp_file)

if __name__ == '__main__':
    unittest.main()