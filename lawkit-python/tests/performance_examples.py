import unittest
import tempfile
import os
from lawkit_python import lawkit

class TestPerformanceExamples(unittest.TestCase):
    
    def create_temp_csv(self, content):
        temp_file = tempfile.NamedTemporaryFile(mode='w', suffix='.csv', delete=False)
        temp_file.write(content)
        temp_file.close()
        return temp_file.name
    
    def create_temp_txt(self, content):
        temp_file = tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False)
        temp_file.write(content)
        temp_file.close()
        return temp_file.name
    
    def test_basic_benford(self):
        # Test case 1: lawkit benf data.csv
        csv_content = "value\n123\n234\n345\n111\n222\n333\n121\n232\n343"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_pareto_with_threshold(self):
        # Test case 2: lawkit pareto data.csv --threshold 0.8
        csv_content = "value\n1000\n500\n300\n200\n100\n50\n30\n20\n10"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.pareto(temp_file, threshold=0.8)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_zipf_text(self):
        # Test case 3: lawkit zipf text.txt
        txt_content = "the quick brown fox jumps over the lazy dog the the"
        temp_file = self.create_temp_txt(txt_content)
        try:
            result = lawkit.zipf(temp_file)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_normal_distribution(self):
        # Test case 4: lawkit normal data.csv
        csv_content = "value\n50\n51\n49\n52\n48\n50\n51\n49\n50"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.normal(temp_file)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_poisson_distribution(self):
        # Test case 5: lawkit poisson data.csv
        csv_content = "count\n3\n2\n4\n1\n3\n2\n5\n3\n2"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.poisson(temp_file)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_multiple_laws(self):
        # Test case 6: lawkit analyze data.csv --laws benford,pareto,normal
        csv_content = "value\n123\n234\n345\n111\n222\n333\n121\n232\n343"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws=['benford', 'pareto', 'normal'])
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_quiet_json(self):
        # Test case 7: lawkit benf data.csv --quiet --format json
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, quiet=True, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_verbose(self):
        # Test case 8: lawkit benf data.csv --verbose
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, verbose=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_format_csv(self):
        # Test case 9: lawkit benf data.csv --format csv
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='csv')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_format_yaml(self):
        # Test case 10: lawkit benf data.csv --format yaml
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='yaml')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_format_toml(self):
        # Test case 11: lawkit benf data.csv --format toml
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='toml')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_format_xml(self):
        # Test case 12: lawkit benf data.csv --format xml
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='xml')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_benford_pareto(self):
        # Test case 13: lawkit analyze data.csv --laws benford,pareto
        csv_content = "value\n1000\n500\n300\n200\n100\n50\n30\n20\n10"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws=['benford', 'pareto'])
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_focus_accuracy(self):
        # Test case 14: lawkit analyze data.csv --laws benford --focus accuracy
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws=['benford'], focus='quality')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_all_laws_purpose_audit(self):
        # Test case 15: lawkit analyze data.csv --laws all --purpose audit
        csv_content = "value\n123\n234\n345\n111\n222\n333\n121\n232\n343"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws='all', purpose='fraud')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_all_laws_recommend(self):
        # Test case 16: lawkit analyze data.csv --laws all --recommend
        csv_content = "value\n123\n234\n345\n111\n222\n333\n121\n232\n343"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws='all', recommend=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_optimized(self):
        # Test case 17: lawkit benf optimized.csv
        csv_content = "value\n123\n234\n345\n111\n222\n333\n121\n232\n343"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_quiet(self):
        # Test case 18: lawkit benf data.csv --quiet
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, quiet=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_verbose_2(self):
        # Test case 19: lawkit benf data.csv --verbose (duplicate)
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, verbose=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_pareto_quiet(self):
        # Test case 20: lawkit pareto data.csv --quiet
        csv_content = "value\n1000\n500\n300\n200\n100\n50\n30\n20\n10"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.pareto(temp_file, quiet=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_benford_pareto_quiet(self):
        # Test case 21: lawkit analyze data.csv --laws benford,pareto --quiet
        csv_content = "value\n1000\n500\n300\n200\n100\n50\n30\n20\n10"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws=['benford', 'pareto'], quiet=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_all_laws_quiet(self):
        # Test case 22: lawkit analyze data.csv --laws all --quiet
        csv_content = "value\n123\n234\n345\n111\n222\n333\n121\n232\n343"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws='all', quiet=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_json_quiet(self):
        # Test case 23: lawkit analyze data.csv --format json --quiet
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, output_format='json', quiet=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_format_yaml_2(self):
        # Test case 24: lawkit benf data.csv --format yaml (duplicate)
        csv_content = "value\n123\n234\n345\n111\n222\n333"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='yaml')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_all_laws_verbose(self):
        # Test case 25: lawkit analyze data.csv --laws all --verbose
        csv_content = "value\n123\n234\n345\n111\n222\n333\n121\n232\n343"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws='all', verbose=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_small_data_quiet(self):
        # Test case 26: lawkit benf small_data.csv --quiet
        csv_content = "value\n123\n234\n345"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, quiet=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_medium_data(self):
        # Test case 27: lawkit analyze medium_data.csv --laws benford,pareto
        csv_content = "value\n1000\n2000\n3000\n1100\n2100\n3100\n1200\n2200\n3200"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws=['benford', 'pareto'])
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_analyze_large_data_quiet(self):
        # Test case 28: lawkit analyze large_data.csv --laws benford --quiet
        csv_content = "value\n10000\n20000\n30000\n11000\n21000\n31000\n12000\n22000\n32000"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws=['benford'], quiet=True)
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_benford_huge_data_quiet_json(self):
        # Test case 29: lawkit benf huge_data.csv --quiet --format json
        csv_content = "value\n100000\n200000\n300000\n110000\n210000\n310000\n120000\n220000\n320000"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, quiet=True, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_version(self):
        # Test case 30: lawkit --version
        result = lawkit.version()
        self.assertIsNotNone(result)
    
    def test_help(self):
        # Test case 31: lawkit --help
        result = lawkit.help()
        self.assertIsNotNone(result)
    
    def test_benf_help(self):
        # Test case 32: lawkit benf --help
        result = lawkit.benf_help()
        self.assertIsNotNone(result)
    
    def test_pareto_help(self):
        # Test case 33: lawkit pareto --help
        result = lawkit.pareto_help()
        self.assertIsNotNone(result)
    
    def test_zipf_help(self):
        # Test case 34: lawkit zipf --help
        result = lawkit.zipf_help()
        self.assertIsNotNone(result)

if __name__ == '__main__':
    unittest.main()