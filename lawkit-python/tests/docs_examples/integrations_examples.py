import unittest
import tempfile
import os
from lawkit_python import lawkit

class TestIntegrationsExamples(unittest.TestCase):
    
    def create_temp_csv(self, content):
        temp_file = tempfile.NamedTemporaryFile(mode='w', suffix='.csv', delete=False)
        temp_file.write(content)
        temp_file.close()
        return temp_file.name
    
    def test_ci_cd_analyze_with_laws_and_format_json(self):
        # Test case 1: lawkit analyze "$file" --laws benford,normal --format json
        csv_content = "value\n1\n2\n3\n11\n12\n13\n21\n22\n31"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws=['benford', 'normal'], output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_gitlab_analyze_all_laws_json(self):
        # Test case 2: lawkit analyze data/financial.csv --laws all --format json
        csv_content = "amount\n1000\n2000\n3000\n1100\n1200\n1300\n2100\n2200\n3100"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws='all', output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_python_api_benford_json(self):
        # Test case 3: lawkit benf data_file --format json (Python API example)
        csv_content = "digit\n1\n2\n3\n11\n12\n13\n21\n22"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_nodejs_benford_json(self):
        # Test case 4: lawkit benf dataFile --format json (Node.js example)
        csv_content = "sales\n1234\n2345\n3456\n1111\n1222\n1333\n2111"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_postgresql_benford_csv_json(self):
        # Test case 5: lawkit benf /tmp/data.csv --format json (PostgreSQL example)
        csv_content = "transaction_amount\n1500\n2600\n3700\n1800\n1900\n2000"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_tableau_analyze_all_laws_json(self):
        # Test case 6: lawkit analyze data_source --laws all --format json (Tableau example)
        csv_content = "business_value\n1000\n2000\n3000\n1100\n1200\n2100\n3100\n4000"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws='all', output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_powerbi_benford_json(self):
        # Test case 7: lawkit benf temp_file --format json (Power BI example)
        csv_content = "revenue\n15000\n26000\n37000\n18000\n19000\n20000"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_aws_lambda_benford_json(self):
        # Test case 8: lawkit benf /tmp/data.csv --format json (AWS Lambda example)
        csv_content = "financial_data\n12000\n23000\n34000\n11000\n12500\n21000"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_gcp_analyze_all_laws_json(self):
        # Test case 9: lawkit analyze /tmp/data.csv --laws all --format json (GCP example)
        csv_content = "cloud_data\n10000\n20000\n30000\n11000\n12000\n21000\n31000"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.analyze(temp_file, laws='all', output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_prometheus_benford_json(self):
        # Test case 10: lawkit benf data_file --format json (Prometheus example)
        csv_content = "metric_value\n1500\n2600\n3700\n1800\n1900\n2000"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)
    
    def test_rust_custom_benford_json(self):
        # Test case 11: lawkit benf file_path --format json (Rust custom example)
        csv_content = "custom_data\n1234\n2345\n3456\n1567\n1678\n2789"
        temp_file = self.create_temp_csv(csv_content)
        try:
            result = lawkit.benf(temp_file, output_format='json')
            self.assertIsNotNone(result)
        finally:
            os.unlink(temp_file)

if __name__ == '__main__':
    unittest.main()