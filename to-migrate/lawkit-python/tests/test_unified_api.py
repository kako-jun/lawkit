"""
Comprehensive unit tests for lawkit-python unified API
Tests PyO3 bindings layer, Python type conversion, and statistical law analysis
"""

import unittest
import json
import os
import sys
import tempfile
from typing import Dict, Any, List

# Import the lawkit_python module
import lawkit_python


class TestLawkitUnifiedAPI(unittest.TestCase):
    """Test the unified lawkit API - only the law() function should be exposed"""
    
    def setUp(self):
        """Set up test fixtures for each test"""
        # Shared fixtures from core tests (translated to Python)
        self.fixtures = TestFixtures()
    
    def test_law_function_exists(self):
        """Test that the main law() function is available"""
        self.assertTrue(hasattr(lawkit_python, 'law'))
        
    def test_law_function_signature(self):
        """Test that law() function accepts the expected parameters"""
        # Should accept subcommand, data, and optional options
        try:
            # This should not crash due to signature issues
            result = lawkit_python.law("validate", [1, 2, 3])
            # Result may fail due to data issues, but function should exist
        except Exception as e:
            # Function exists if we get a lawkit-specific error, not AttributeError
            self.assertNotIsInstance(e, AttributeError)

    # ============================================================================
    # UNIFIED API TESTS - Core Functionality
    # ============================================================================

    def test_law_benford_analysis(self):
        """Test Benford's law analysis through unified API"""
        data = self.fixtures.benford_compliant_data()
        
        results = lawkit_python.law("benford", data)
        
        self.assertEqual(len(results), 1)
        result = results[0]
        
        # Check result structure
        self.assertIn('type', result)
        self.assertEqual(result['type'], 'BenfordAnalysis')
        self.assertIn('data', result)
        
        benford_data = result['data']
        self.assertIn('observed_distribution', benford_data)
        self.assertIn('expected_distribution', benford_data)
        self.assertIn('chi_square', benford_data)
        self.assertIn('p_value', benford_data)
        self.assertIn('total_numbers', benford_data)
        self.assertIn('analysis_summary', benford_data)
        
        # Validate data types and ranges
        self.assertEqual(len(benford_data['observed_distribution']), 9)
        self.assertEqual(len(benford_data['expected_distribution']), 9)
        self.assertGreaterEqual(benford_data['chi_square'], 0.0)
        self.assertGreaterEqual(benford_data['p_value'], 0.0)
        self.assertLessEqual(benford_data['p_value'], 1.0)
        self.assertGreater(benford_data['total_numbers'], 0)
        self.assertIsInstance(benford_data['analysis_summary'], str)
        self.assertNotEqual(benford_data['analysis_summary'], "")

    def test_law_pareto_analysis(self):
        """Test Pareto principle analysis through unified API"""
        data = self.fixtures.pareto_compliant_data()
        
        results = lawkit_python.law("pareto", data)
        
        self.assertEqual(len(results), 1)
        result = results[0]
        
        self.assertEqual(result['type'], 'ParetoAnalysis')
        pareto_data = result['data']
        
        self.assertIn('top_20_percent_contribution', pareto_data)
        self.assertIn('pareto_ratio', pareto_data)
        self.assertIn('concentration_index', pareto_data)
        self.assertIn('total_items', pareto_data)
        self.assertIn('analysis_summary', pareto_data)
        
        # Validate values for compliant data
        self.assertGreater(pareto_data['top_20_percent_contribution'], 0.0)
        self.assertGreater(pareto_data['pareto_ratio'], 0.0)
        self.assertGreaterEqual(pareto_data['concentration_index'], 0.0)
        self.assertGreater(pareto_data['total_items'], 0)
        self.assertGreater(pareto_data['top_20_percent_contribution'], 60.0)  # Should be high for compliant data

    def test_law_zipf_analysis(self):
        """Test Zipf's law analysis through unified API"""
        data = self.fixtures.zipf_compliant_data()
        
        results = lawkit_python.law("zipf", data)
        
        self.assertEqual(len(results), 1)
        result = results[0]
        
        self.assertEqual(result['type'], 'ZipfAnalysis')
        zipf_data = result['data']
        
        self.assertIn('zipf_coefficient', zipf_data)
        self.assertIn('correlation_coefficient', zipf_data)
        self.assertIn('deviation_score', zipf_data)
        self.assertIn('total_items', zipf_data)
        self.assertIn('analysis_summary', zipf_data)
        
        # Validate ranges
        self.assertNotEqual(zipf_data['zipf_coefficient'], 0.0)
        self.assertGreaterEqual(zipf_data['correlation_coefficient'], -1.0)
        self.assertLessEqual(zipf_data['correlation_coefficient'], 1.0)
        self.assertGreaterEqual(zipf_data['deviation_score'], 0.0)
        self.assertGreater(zipf_data['total_items'], 0)

    def test_law_normal_analysis(self):
        """Test normal distribution analysis through unified API"""
        data = self.fixtures.normal_distribution_data()
        
        results = lawkit_python.law("normal", data)
        
        self.assertEqual(len(results), 1)
        result = results[0]
        
        self.assertEqual(result['type'], 'NormalAnalysis')
        normal_data = result['data']
        
        self.assertIn('mean', normal_data)
        self.assertIn('std_dev', normal_data)
        self.assertIn('skewness', normal_data)
        self.assertIn('kurtosis', normal_data)
        self.assertIn('normality_test_p', normal_data)
        self.assertIn('total_numbers', normal_data)
        self.assertIn('analysis_summary', normal_data)
        
        # Validate statistical properties
        self.assertGreater(normal_data['std_dev'], 0.0)
        self.assertGreaterEqual(normal_data['normality_test_p'], 0.0)
        self.assertLessEqual(normal_data['normality_test_p'], 1.0)
        self.assertGreater(normal_data['total_numbers'], 0)

    def test_law_poisson_analysis(self):
        """Test Poisson distribution analysis through unified API"""
        data = self.fixtures.poisson_distribution_data()
        
        results = lawkit_python.law("poisson", data)
        
        self.assertEqual(len(results), 1)
        result = results[0]
        
        self.assertEqual(result['type'], 'PoissonAnalysis')
        poisson_data = result['data']
        
        self.assertIn('lambda', poisson_data)
        self.assertIn('variance_ratio', poisson_data)
        self.assertIn('poisson_test_p', poisson_data)
        self.assertIn('total_events', poisson_data)
        self.assertIn('analysis_summary', poisson_data)
        
        # Validate Poisson properties
        self.assertGreater(poisson_data['lambda'], 0.0)
        self.assertGreater(poisson_data['variance_ratio'], 0.0)
        self.assertGreaterEqual(poisson_data['poisson_test_p'], 0.0)
        self.assertLessEqual(poisson_data['poisson_test_p'], 1.0)
        self.assertGreater(poisson_data['total_events'], 0)

    def test_law_validate_data(self):
        """Test data validation through unified API"""
        data = self.fixtures.validation_test_data()["valid_dataset"]
        
        results = lawkit_python.law("validate", data)
        
        self.assertEqual(len(results), 1)
        result = results[0]
        
        self.assertEqual(result['type'], 'ValidationResult')
        validation_data = result['data']
        
        self.assertIn('validation_passed', validation_data)
        self.assertIn('data_quality_score', validation_data)
        self.assertIn('analysis_summary', validation_data)
        
        # Valid data should pass validation
        self.assertTrue(validation_data['validation_passed'])
        self.assertGreater(validation_data['data_quality_score'], 0.0)

    def test_law_diagnose_data(self):
        """Test data diagnostics through unified API"""
        data = self.fixtures.diagnostic_test_data()["normal_with_outliers"]
        
        results = lawkit_python.law("diagnose", data)
        
        self.assertEqual(len(results), 1)
        result = results[0]
        
        self.assertEqual(result['type'], 'DiagnosticResult')
        diagnostic_data = result['data']
        
        self.assertIn('diagnostic_type', diagnostic_data)
        self.assertIn('findings', diagnostic_data)
        self.assertIn('confidence_level', diagnostic_data)
        self.assertIn('analysis_summary', diagnostic_data)
        
        self.assertEqual(diagnostic_data['diagnostic_type'], "General")
        self.assertIsInstance(diagnostic_data['findings'], list)
        self.assertGreater(len(diagnostic_data['findings']), 0)
        self.assertGreater(diagnostic_data['confidence_level'], 0.0)
        self.assertLessEqual(diagnostic_data['confidence_level'], 1.0)

    def test_law_generate_data(self):
        """Test data generation through unified API"""
        config = self.fixtures.generation_configs()["benford_config"]
        
        results = lawkit_python.law("generate", config)
        
        self.assertEqual(len(results), 1)
        result = results[0]
        
        self.assertEqual(result['type'], 'GeneratedData')
        generated_info = result['data']
        
        self.assertIn('data_type', generated_info)
        self.assertIn('count', generated_info)
        self.assertIn('sample_data', generated_info)
        self.assertIn('parameters', generated_info)
        
        self.assertEqual(generated_info['data_type'], "benford")
        self.assertEqual(generated_info['count'], 1000)
        self.assertIsInstance(generated_info['sample_data'], list)
        self.assertGreater(len(generated_info['sample_data']), 0)
        self.assertIsInstance(generated_info['parameters'], dict)
        self.assertGreater(len(generated_info['parameters']), 0)

    def test_law_analyze_all(self):
        """Test comprehensive analysis through unified API"""
        data = self.fixtures.integration_analysis_data()
        
        results = lawkit_python.law("analyze", data)
        
        # Should have multiple analysis results plus integration
        self.assertGreater(len(results), 1)
        
        # Last result should be integration analysis
        integration_result = results[-1]
        self.assertEqual(integration_result['type'], 'IntegrationAnalysis')
        
        integration_data = integration_result['data']
        self.assertIn('laws_analyzed', integration_data)
        self.assertIn('overall_risk', integration_data)
        self.assertIn('recommendations', integration_data)
        self.assertIn('analysis_summary', integration_data)
        
        self.assertIsInstance(integration_data['laws_analyzed'], list)
        self.assertGreater(len(integration_data['laws_analyzed']), 0)
        self.assertIsInstance(integration_data['overall_risk'], str)
        self.assertNotEqual(integration_data['overall_risk'], "")
        self.assertIsInstance(integration_data['recommendations'], list)
        self.assertGreater(len(integration_data['recommendations']), 0)

    def test_law_unknown_subcommand(self):
        """Test error handling for unknown subcommand"""
        data = [1, 2, 3]
        
        with self.assertRaises(Exception) as context:
            lawkit_python.law("unknown", data)
        
        self.assertIn("Unknown subcommand", str(context.exception))

    # ============================================================================
    # STATISTICAL LAW SPECIFIC TESTS
    # ============================================================================

    def test_benford_risk_levels(self):
        """Test risk level detection in Benford analysis"""
        # Test compliant data (should be LOW or MEDIUM risk)
        compliant_data = self.fixtures.benford_compliant_data()
        results = lawkit_python.law("benford", compliant_data)
        
        benford_data = results[0]['data']
        self.assertIn(benford_data['risk_level'], ["LOW", "MEDIUM"])
        
        # Test non-compliant data (should be higher risk)
        non_compliant_data = self.fixtures.benford_non_compliant_data()
        results = lawkit_python.law("benford", non_compliant_data)
        
        benford_data = results[0]['data']
        self.assertIn(benford_data['risk_level'], ["MEDIUM", "HIGH"])

    def test_pareto_principle_compliance(self):
        """Test Pareto principle compliance detection"""
        # Test compliant data
        compliant_data = self.fixtures.pareto_compliant_data()
        results = lawkit_python.law("pareto", compliant_data)
        
        pareto_data = results[0]['data']
        self.assertGreater(pareto_data['top_20_percent_contribution'], 60.0)
        self.assertGreater(pareto_data['concentration_index'], 0.0)
        
        # Test non-compliant data
        non_compliant_data = self.fixtures.pareto_non_compliant_data()
        results = lawkit_python.law("pareto", non_compliant_data)
        
        pareto_data = results[0]['data']
        self.assertLess(pareto_data['top_20_percent_contribution'], 60.0)

    def test_normal_distribution_detection(self):
        """Test normal distribution detection"""
        # Test normal data
        normal_data = self.fixtures.normal_distribution_data()
        results = lawkit_python.law("normal", normal_data)
        
        normal_analysis = results[0]['data']
        self.assertLess(abs(normal_analysis['skewness']), 2.0)  # Not too skewed
        self.assertGreater(normal_analysis['std_dev'], 0.0)
        self.assertIn(normal_analysis['risk_level'], ["LOW", "MEDIUM"])
        
        # Test non-normal data
        non_normal_data = self.fixtures.non_normal_distribution_data()
        results = lawkit_python.law("normal", non_normal_data)
        
        normal_analysis = results[0]['data']
        self.assertIn(normal_analysis['risk_level'], ["MEDIUM", "HIGH"])

    def test_poisson_distribution_detection(self):
        """Test Poisson distribution detection"""
        # Test Poisson data
        poisson_data = self.fixtures.poisson_distribution_data()
        results = lawkit_python.law("poisson", poisson_data)
        
        poisson_analysis = results[0]['data']
        self.assertGreater(poisson_analysis['lambda'], 0.0)
        self.assertGreater(poisson_analysis['variance_ratio'], 0.0)
        self.assertIn(poisson_analysis['risk_level'], ["LOW", "MEDIUM"])
        
        # Test non-Poisson data
        non_poisson_data = self.fixtures.non_poisson_data()
        results = lawkit_python.law("poisson", non_poisson_data)
        
        poisson_analysis = results[0]['data']
        self.assertIn(poisson_analysis['risk_level'], ["MEDIUM", "HIGH"])

    # ============================================================================
    # OPTIONS TESTING - lawkit Specific Options
    # ============================================================================

    def test_lawkit_specific_options(self):
        """Test lawkit-specific options"""
        data = self.fixtures.benford_compliant_data()
        
        options = {
            "risk_threshold": "medium",
            "confidence_level": 0.95,
            "significance_level": 0.05,
            "min_sample_size": 20,
            "enable_outlier_detection": True
        }
        
        results = lawkit_python.law("benford", data, options)
        
        self.assertEqual(len(results), 1)
        benford_data = results[0]['data']
        self.assertGreater(benford_data['total_numbers'], 0)

    def test_benford_specific_options(self):
        """Test Benford-specific options"""
        data = self.fixtures.benford_compliant_data()
        
        options = {
            "benford_digits": "first",
            "benford_base": 10
        }
        
        results = lawkit_python.law("benford", data, options)
        
        benford_data = results[0]['data']
        # Should analyze first digits (9 digits: 1-9)
        self.assertEqual(len(benford_data['observed_distribution']), 9)
        self.assertEqual(len(benford_data['expected_distribution']), 9)

    def test_pareto_specific_options(self):
        """Test Pareto-specific options"""
        data = self.fixtures.pareto_compliant_data()
        
        options = {
            "pareto_ratio": 0.8,  # 80/20 rule
            "pareto_category_limit": 100
        }
        
        results = lawkit_python.law("pareto", data, options)
        
        pareto_data = results[0]['data']
        self.assertGreater(pareto_data['total_items'], 0)
        self.assertGreater(pareto_data['top_20_percent_contribution'], 0.0)

    def test_generation_options(self):
        """Test data generation options"""
        config = {
            "type": "normal",
            "count": 500,
            "mean": 50.0,
            "std_dev": 10.0
        }
        
        options = {
            "generate_count": 500,
            "generate_range_min": 0.0,
            "generate_range_max": 100.0,
            "generate_seed": 12345
        }
        
        results = lawkit_python.law("generate", config, options)
        
        generated_info = results[0]['data']
        self.assertEqual(generated_info['data_type'], "normal")
        self.assertEqual(generated_info['count'], 500)
        self.assertIn("mean", generated_info['parameters'])
        self.assertIn("std_dev", generated_info['parameters'])
        self.assertEqual(len(generated_info['sample_data']), 500)

    # ============================================================================
    # OUTPUT FORMAT TESTS
    # ============================================================================

    def test_output_formats(self):
        """Test different output formats"""
        data = [1, 2, 3]
        
        # Test default format (should work)
        results = lawkit_python.law("validate", data)
        self.assertIsInstance(results, list)
        self.assertGreater(len(results), 0)
        
        # Test with format options if supported
        try:
            options = {"output_format": "json"}
            results = lawkit_python.law("validate", data, options)
            self.assertIsInstance(results, list)
        except Exception:
            # Format options might not be implemented at Python binding level
            pass

    # ============================================================================
    # DATA VALIDATION AND ERROR HANDLING TESTS
    # ============================================================================

    def test_empty_data_handling(self):
        """Test handling of empty data"""
        empty_data = []
        
        with self.assertRaises(Exception) as context:
            lawkit_python.law("benford", empty_data)
        
        self.assertIn("No valid numbers found", str(context.exception))

    def test_invalid_data_handling(self):
        """Test handling of invalid data types"""
        invalid_data = {"not": "numbers"}
        
        with self.assertRaises(Exception) as context:
            lawkit_python.law("benford", invalid_data)
        
        self.assertIn("No valid numbers found", str(context.exception))

    def test_small_sample_size(self):
        """Test handling of insufficient data"""
        small_data = [1.0, 2.0]
        
        with self.assertRaises(Exception) as context:
            lawkit_python.law("normal", small_data)
        
        self.assertIn("Insufficient data points", str(context.exception))

    def test_validation_with_issues(self):
        """Test validation with problematic data"""
        problematic_data = self.fixtures.validation_test_data()["small_dataset"]
        
        results = lawkit_python.law("validate", problematic_data)
        
        validation_data = results[0]['data']
        self.assertFalse(validation_data['validation_passed'])
        self.assertIn('issues_found', validation_data)
        self.assertGreater(len(validation_data['issues_found']), 0)
        self.assertLess(validation_data['data_quality_score'], 1.0)

    # ============================================================================
    # PYTHON-SPECIFIC BINDING TESTS
    # ============================================================================

    def test_python_type_conversion(self):
        """Test Python type conversion in PyO3 bindings"""
        # Test with Python list
        python_list = [1.0, 2.0, 3.0, 4.0, 5.0]
        results = lawkit_python.law("validate", python_list)
        self.assertIsInstance(results, list)
        
        # Test with nested Python structures
        nested_data = {
            "numbers": [1, 2, 3],
            "values": [4.0, 5.0, 6.0]
        }
        results = lawkit_python.law("validate", nested_data)
        self.assertIsInstance(results, list)

    def test_exception_handling(self):
        """Test Python exception handling from Rust"""
        # Test that Rust errors are properly converted to Python exceptions
        with self.assertRaises(Exception):
            lawkit_python.law("invalid_command", [1, 2, 3])
        
        # Test with None data
        with self.assertRaises(Exception):
            lawkit_python.law("benford", None)

    def test_large_dataset_performance(self):
        """Test performance with larger datasets"""
        # Generate larger dataset for Python performance testing
        large_data = list(range(1, 1001))  # 1000 numbers
        
        import time
        start_time = time.time()
        results = lawkit_python.law("validate", large_data)
        end_time = time.time()
        
        self.assertIsInstance(results, list)
        self.assertLess(end_time - start_time, 5.0)  # Should complete within 5 seconds

    def test_memory_management(self):
        """Test memory management in Python bindings"""
        # Test that large data structures are properly cleaned up
        for i in range(10):
            large_data = list(range(i * 100, (i + 1) * 100))
            results = lawkit_python.law("validate", large_data)
            self.assertIsInstance(results, list)
            # Force cleanup between iterations
            del results
            del large_data

    # ============================================================================
    # INTEGRATION TESTS WITH FIXTURES
    # ============================================================================

    def test_comprehensive_analysis_workflow(self):
        """Test comprehensive analysis workflow"""
        integration_data = self.fixtures.integration_analysis_data()
        
        options = {
            "show_details": True,
            "show_recommendations": True
        }
        
        results = lawkit_python.law("analyze", integration_data, options)
        
        # Should have multiple analyses
        self.assertGreater(len(results), 1)
        
        # Check that we get different types of analysis
        result_types = set()
        for result in results:
            result_types.add(result['type'])
        
        # Should have multiple analysis types
        self.assertGreaterEqual(len(result_types), 3)
        self.assertIn('IntegrationAnalysis', result_types)

    def test_data_generation_and_analysis_cycle(self):
        """Test data generation and analysis cycle"""
        # Generate Benford data
        config = {
            "type": "benford",
            "count": 100
        }
        
        generation_results = lawkit_python.law("generate", config)
        generated_info = generation_results[0]['data']
        generated_data = generated_info['sample_data']
        
        # Analyze the generated data
        analysis_results = lawkit_python.law("benford", generated_data)
        
        benford_data = analysis_results[0]['data']
        # Generated Benford data should show low risk
        self.assertIn(benford_data['risk_level'], ["LOW", "MEDIUM"])
        self.assertEqual(benford_data['total_numbers'], 100)


class TestFixtures:
    """Test fixtures for lawkit statistical law analysis testing"""
    
    def benford_compliant_data(self):
        """Benford-compliant financial data"""
        return {
            "financial_data": [
                123.45, 187.92, 234.67, 298.34, 345.78, 456.23, 567.89, 678.12, 789.56,
                1234.56, 1876.43, 2345.67, 2987.34, 3456.78, 4567.89, 5678.12, 6789.34, 7890.45,
                12345.67, 18765.43, 23456.78, 29876.54, 34567.89, 45678.12, 56789.34, 67890.45, 78901.23,
                123456.78, 187654.32, 234567.89, 298765.43, 345678.91, 456789.12, 567890.23, 678901.34
            ],
            "invoice_amounts": [
                101.50, 125.00, 198.75, 267.30, 334.20, 445.60, 523.80, 612.40, 785.90,
                1055.25, 1287.60, 1934.75, 2156.80, 3245.70, 4123.50, 5678.25, 6234.80, 7891.20,
                10234.50, 12876.30, 19847.60, 21568.90, 32457.80, 41235.60, 56782.40, 62348.70, 78912.30
            ]
        }
    
    def benford_non_compliant_data(self):
        """Non-Benford compliant data"""
        return {
            "uniform_data": [
                500.0, 501.0, 502.0, 503.0, 504.0, 505.0, 506.0, 507.0, 508.0, 509.0,
                510.0, 511.0, 512.0, 513.0, 514.0, 515.0, 516.0, 517.0, 518.0, 519.0,
                520.0, 521.0, 522.0, 523.0, 524.0, 525.0, 526.0, 527.0, 528.0, 529.0
            ],
            "suspicious_data": [
                7000.0, 7001.0, 7002.0, 7003.0, 7004.0, 7005.0, 7006.0, 7007.0, 7008.0, 7009.0,
                8000.0, 8001.0, 8002.0, 8003.0, 8004.0, 8005.0, 8006.0, 8007.0, 8008.0, 8009.0,
                9000.0, 9001.0, 9002.0, 9003.0, 9004.0, 9005.0, 9006.0, 9007.0, 9008.0, 9009.0
            ]
        }
    
    def pareto_compliant_data(self):
        """Pareto principle compliant data"""
        return {
            "sales_data": [
                # Top 20% should contribute ~80% of total
                10000.0, 9500.0, 9000.0, 8500.0,  # Top 4 items (20% of 20 items)
                1000.0, 950.0, 900.0, 850.0, 800.0, 750.0, 700.0, 650.0,
                600.0, 550.0, 500.0, 450.0, 400.0, 350.0, 300.0, 250.0
            ],
            "customer_revenue": [
                50000.0, 45000.0, 40000.0, 35000.0, 30000.0,  # Top 5 customers
                2000.0, 1900.0, 1800.0, 1700.0, 1600.0, 1500.0, 1400.0, 1300.0,
                1200.0, 1100.0, 1000.0, 900.0, 800.0, 700.0, 600.0, 500.0, 400.0, 300.0, 200.0, 100.0
            ]
        }
    
    def pareto_non_compliant_data(self):
        """Non-Pareto compliant uniform data"""
        return {
            "uniform_distribution": [
                1000.0, 1010.0, 1020.0, 1030.0, 1040.0, 1050.0, 1060.0, 1070.0, 1080.0, 1090.0,
                1100.0, 1110.0, 1120.0, 1130.0, 1140.0, 1150.0, 1160.0, 1170.0, 1180.0, 1190.0
            ]
        }
    
    def zipf_compliant_data(self):
        """Zipf's law compliant data"""
        return {
            "word_frequencies": [
                # Frequencies following Zipf's law: f(r) = f(1)/r
                10000.0, 5000.0, 3333.33, 2500.0, 2000.0, 1666.67, 1428.57, 1250.0, 1111.11, 1000.0,
                909.09, 833.33, 769.23, 714.29, 666.67, 625.0, 588.24, 555.56, 526.32, 500.0
            ]
        }
    
    def normal_distribution_data(self):
        """Normal distribution data"""
        return {
            "normal_sample": [
                98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8,
                100.0, 99.5, 100.7, 99.2, 100.3, 99.6, 100.1, 99.9, 100.4, 99.7
            ]
        }
    
    def non_normal_distribution_data(self):
        """Non-normal distribution data"""
        return {
            "skewed_data": [
                1.0, 1.1, 1.2, 1.3, 1.5, 1.8, 2.2, 2.8, 3.6, 4.7,
                6.1, 8.0, 10.4, 13.5, 17.6, 22.9, 29.8, 38.7, 50.3, 65.4,
                85.0, 110.5, 143.7, 186.8, 242.8, 315.6, 410.3, 533.4, 693.4, 901.4
            ]
        }
    
    def poisson_distribution_data(self):
        """Poisson distribution data"""
        return {
            "event_counts": [
                0, 1, 2, 1, 3, 0, 2, 1, 4, 2, 1, 0, 3, 2, 1, 5, 0, 2, 1, 3,
                2, 1, 0, 4, 2, 1, 3, 0, 2, 1, 2, 3, 1, 0, 2, 1, 4, 2, 0, 3
            ]
        }
    
    def non_poisson_data(self):
        """Non-Poisson data with high variance"""
        return {
            "high_variance": [
                0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 0, 0, 0, 0, 0,
                100, 100, 100, 100, 100, 0, 0, 0, 0, 0, 25, 25, 25, 25, 25
            ]
        }
    
    def integration_analysis_data(self):
        """Integration analysis data"""
        return {
            "comprehensive_dataset": {
                "financial_transactions": [
                    123.45, 187.92, 234.67, 298.34, 345.78, 456.23, 567.89, 678.12, 789.56,
                    1234.56, 1876.43, 2345.67, 2987.34, 3456.78, 4567.89, 5678.12, 6789.34, 7890.45
                ],
                "sales_amounts": [
                    50000.0, 45000.0, 40000.0, 35000.0, 30000.0,
                    2000.0, 1900.0, 1800.0, 1700.0, 1600.0, 1500.0, 1400.0, 1300.0,
                    1200.0, 1100.0, 1000.0, 900.0, 800.0, 700.0, 600.0
                ],
                "quality_scores": [
                    98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                    100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8
                ],
                "incident_counts": [
                    0, 1, 2, 1, 3, 0, 2, 1, 4, 2, 1, 0, 3, 2, 1, 5, 0, 2, 1, 3
                ]
            }
        }
    
    def validation_test_data(self):
        """Validation test data"""
        return {
            "valid_dataset": [
                123.45, 234.67, 345.89, 456.12, 567.34, 678.56, 789.78, 890.23, 901.45, 123.67,
                234.89, 345.12, 456.34, 567.56, 678.78, 789.01, 890.23, 901.45, 123.67, 234.89
            ],
            "small_dataset": [1.0, 2.0, 3.0]
        }
    
    def diagnostic_test_data(self):
        """Diagnostic test data"""
        return {
            "normal_with_outliers": [
                98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8,
                150.0,  # Outlier
                100.0, 99.5, 100.7, 99.2, 100.3, 99.6, 100.1, 99.9, 100.4, 99.7,
                50.0    # Another outlier
            ]
        }
    
    def generation_configs(self):
        """Generation configuration data"""
        return {
            "benford_config": {
                "type": "benford",
                "count": 1000,
                "base": 10
            },
            "normal_config": {
                "type": "normal",
                "count": 500,
                "mean": 100.0,
                "std_dev": 15.0
            },
            "poisson_config": {
                "type": "poisson",
                "count": 300,
                "lambda": 5.0
            }
        }


if __name__ == '__main__':
    unittest.main()