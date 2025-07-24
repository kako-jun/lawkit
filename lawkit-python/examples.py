#!/usr/bin/env python3
"""
lawkit-python Examples - UNIFIED API DESIGN

Demonstrates native Python API usage for statistical law analysis
Users prepare data themselves and call the unified law() function
"""

import json
import random
import math
import tempfile
import os
import numpy as np
from pathlib import Path
from typing import Any, Dict, List, Union
from lawkit_python import law

def print_header(title: str) -> None:
    """Print a formatted header"""
    print(f"\n{title}")
    print("=" * len(title))

def print_example(title: str, description: str) -> None:
    """Print example title and description"""
    print(f"\n{title}")
    print(f"   {description}")

def print_results(results: List[Dict[str, Any]]) -> None:
    """Print law analysis results in a formatted way"""
    if not results:
        print("   No analysis results.")
        return
    
    print("   Analysis Results:")
    for result in results:
        result_type = result.get('type', 'unknown')
        path = result.get('path', '')
        
        if result_type == 'BenfordAnalysis':
            risk_level = result.get('risk_level', 'unknown')
            chi_square = result.get('chi_square', 0)
            p_value = result.get('p_value', 0)
            total_numbers = result.get('total_numbers', 0)
            
            risk_emoji = '✅' if risk_level == 'low' else '⚠️' if risk_level == 'medium' else '🚨'
            print(f"   {risk_emoji} Benford's Law: {path}")
            print(f"      Risk Level: {risk_level}")
            print(f"      Chi-square: {chi_square:.4f}")
            print(f"      P-value: {p_value:.6f}")
            print(f"      Sample Size: {total_numbers}")
            
        elif result_type == 'ParetoAnalysis':
            risk_level = result.get('risk_level', 'unknown')
            top_20_contribution = result.get('top_20_percent_contribution', 0)
            concentration_index = result.get('concentration_index', 0)
            
            risk_emoji = '✅' if risk_level == 'low' else '⚠️' if risk_level == 'medium' else '🚨'
            print(f"   {risk_emoji} Pareto Analysis: {path}")
            print(f"      Risk Level: {risk_level}")
            print(f"      Top 20% Contribution: {top_20_contribution:.1f}%")
            print(f"      Concentration Index: {concentration_index:.3f}")
            
        elif result_type == 'ValidationResult':
            passed = result.get('validation_passed', False)
            quality_score = result.get('data_quality_score', 0)
            issues = result.get('issues_found', [])
            
            status_emoji = '✅' if passed else '❌'
            print(f"   {status_emoji} Validation: {path}")
            print(f"      Status: {'PASSED' if passed else 'FAILED'}")
            print(f"      Quality Score: {quality_score:.2f}")
            if issues:
                print(f"      Issues: {', '.join(issues)}")
                
        elif result_type == 'GeneratedData':
            data_type = result.get('data_type', 'unknown')
            count = result.get('count', 0)
            sample_data = result.get('sample_data', [])
            
            print(f"   📊 Generated {data_type}: {count} samples")
            if sample_data:
                print(f"      Sample: {sample_data[:5]}...")
                
        else:
            print(f"   • {result}")

def generate_benford_data(count: int, seed: int = None) -> List[float]:
    """Generate data that follows Benford's Law"""
    if seed:
        random.seed(seed)
    
    data = []
    for _ in range(count):
        # Generate using exponential distribution which naturally follows Benford's Law
        magnitude = random.randint(1, 6)  # 1-6 digits
        base = math.exp(random.uniform(0, math.log(10)))  # Benford distribution
        value = base * (10 ** (magnitude - 1))
        data.append(value)
    
    return data

def generate_suspicious_financial_data(count: int) -> List[float]:
    """Generate suspicious financial data that violates Benford's Law"""
    data = []
    # Artificially bias toward higher first digits (fraud pattern)
    biased_digits = [5, 6, 7, 8, 9] * (count // 5)
    normal_digits = [1, 2, 3, 4] * (count // 20)
    all_digits = biased_digits + normal_digits
    random.shuffle(all_digits)
    
    for i in range(count):
        first_digit = all_digits[i % len(all_digits)]
        magnitude = random.randint(2, 5)  # 2-5 digits for financial data
        remaining = random.randint(10**(magnitude-2), 10**(magnitude-1) - 1)
        value = first_digit * (10**(magnitude-1)) + remaining
        data.append(float(value))
    
    return data

def generate_pareto_data(count: int) -> List[float]:
    """Generate data that follows Pareto distribution (80/20 rule)"""
    data = []
    # 20% of items get 80% of the value
    high_value_count = count // 5
    low_value_count = count - high_value_count
    
    # High value items (20%)
    for _ in range(high_value_count):
        data.append(random.uniform(800, 1000))
    
    # Low value items (80%)
    for _ in range(low_value_count):
        data.append(random.uniform(1, 200))
    
    random.shuffle(data)
    return data

def example_fraud_detection():
    """Financial fraud detection using Benford's Law"""
    print_example(
        "Financial Fraud Detection",
        "Analyze financial transactions for potential fraud using Benford's Law"
    )
    
    # Generate legitimate financial transactions
    legitimate_transactions = generate_benford_data(2000, seed=42)
    
    # Generate suspicious transactions
    suspicious_transactions = generate_suspicious_financial_data(2000)
    
    print("   Analyzing legitimate transactions...")
    legitimate_results = law(
        "benford",
        legitimate_transactions,
        confidence_level=0.99,
        analysis_threshold=0.05,
        risk_threshold="medium"
    )
    print_results(legitimate_results)
    
    print("\n   Analyzing suspicious transactions...")
    suspicious_results = law(
        "benford", 
        suspicious_transactions,
        confidence_level=0.99,
        analysis_threshold=0.05,
        risk_threshold="medium"
    )
    print_results(suspicious_results)

def example_business_data_quality():
    """Business data quality assessment using multiple laws"""
    print_example(
        "Business Data Quality Assessment",
        "Use multiple statistical laws to assess data quality"
    )
    
    # Sales data that should follow Pareto distribution
    sales_data = generate_pareto_data(500)
    
    # Customer transaction amounts
    transaction_data = generate_benford_data(1000, seed=123)
    
    # Product ratings (should be normal-ish)
    ratings_data = [random.gauss(4.2, 0.8) for _ in range(800)]
    ratings_data = [max(1, min(5, r)) for r in ratings_data]  # Clamp to 1-5
    
    # Analyze sales data for Pareto compliance
    print("   Sales Data Analysis (Pareto Distribution):")
    sales_results = law(
        "pareto",
        sales_data,
        confidence_level=0.95,
        analysis_threshold=0.8
    )
    print_results(sales_results)
    
    # Analyze transaction data for Benford compliance
    print("\n   Transaction Data Analysis (Benford's Law):")
    transaction_results = law(
        "benford",
        transaction_data,
        confidence_level=0.95
    )
    print_results(transaction_results)
    
    # Analyze ratings for normal distribution
    print("\n   Ratings Data Analysis (Normal Distribution):")
    ratings_results = law(
        "normal",
        ratings_data,
        confidence_level=0.95
    )
    print_results(ratings_results)

def example_audit_compliance():
    """Audit compliance testing for regulatory requirements"""
    print_example(
        "Audit Compliance Testing",
        "Validate financial reports for regulatory compliance"
    )
    
    # Expense reports that should follow Benford's Law
    expense_reports = [
        234.56, 1234.78, 567.89, 2345.67, 789.12,
        3456.78, 890.23, 4567.89, 123.45, 5678.90,
        345.67, 6789.01, 456.78, 7890.12, 678.90,
        1123.45, 2234.56, 3345.67, 4456.78, 5567.89
    ]
    
    # Account balances
    account_balances = [
        10234.56, 23456.78, 34567.89, 45678.90, 56789.01,
        12345.67, 23456.78, 34567.89, 45678.90, 56789.01,
        15432.10, 26543.21, 37654.32, 48765.43, 59876.54
    ]
    
    print("   Expense Reports Compliance Check:")
    expense_results = law(
        "validate",
        expense_reports,
        confidence_level=0.95,
        significance_level=0.05,
        min_sample_size=15
    )
    print_results(expense_results)
    
    print("\n   Account Balances Analysis:")
    balance_results = law(
        "benford",
        account_balances,
        confidence_level=0.95,
        risk_threshold="low"
    )
    print_results(balance_results)

def example_real_time_monitoring():
    """Real-time data stream monitoring for anomalies"""
    print_example(
        "Real-time Data Stream Monitoring",
        "Monitor incoming data streams for statistical anomalies"
    )
    
    # Simulate streaming data batches
    print("   Processing streaming data batches...")
    
    for batch_num in range(1, 4):
        print(f"\n   Batch {batch_num}:")
        
        if batch_num == 2:
            # Introduce anomaly in batch 2
            batch_data = generate_suspicious_financial_data(300)
            print("   (Anomalous batch injected)")
        else:
            # Normal data
            batch_data = generate_benford_data(300, seed=batch_num * 100)
        
        # Analyze batch in real-time
        batch_results = law(
            "benford",
            batch_data,
            confidence_level=0.98,
            enable_parallel_processing=True,
            memory_limit_mb=50
        )
        
        # Quick assessment
        for result in batch_results:
            if result.get('type') == 'BenfordAnalysis':
                risk = result.get('risk_level', 'unknown')
                p_val = result.get('p_value', 0)
                
                if risk == 'high':
                    print(f"   🚨 ALERT: High risk detected (p={p_val:.4f})")
                elif risk == 'medium':
                    print(f"   ⚠️  WARNING: Medium risk (p={p_val:.4f})")
                else:
                    print(f"   ✅ Normal pattern (p={p_val:.4f})")

def example_international_data():
    """International data analysis with multiple number formats"""
    print_example(
        "International Data Analysis",
        "Handle various international number formats and currencies"
    )
    
    # Mixed international financial data
    international_data = [
        1234.56,     # US format
        2345.67,     # Standard
        3456.78,     # Standard
        "4,567.89",  # US comma format
        "5.678,90",  # European format
        "¥123,456",  # Japanese Yen
        "€7,890.12", # Euro
        "£8,901.23", # British Pound
        9012.34,     # Standard
        "10,123.45", # US comma format
    ]
    
    print("   Note: This example shows data format handling concept.")
    print("   In practice, users would normalize data formats before analysis.")
    
    # For demonstration, convert to float values
    normalized_data = []
    for item in international_data:
        if isinstance(item, str):
            # Simple cleanup (in practice, use proper localization libraries)
            clean_str = ''.join(c for c in item if c.isdigit() or c in '.,')
            if ',' in clean_str and '.' in clean_str:
                # Assume US format: 1,234.56
                clean_str = clean_str.replace(',', '')
            elif clean_str.count('.') > 1:
                # European format: 1.234,56 -> 1234,56 -> 1234.56
                parts = clean_str.split('.')
                clean_str = ''.join(parts[:-1]) + '.' + parts[-1]
            elif ',' in clean_str:
                # European decimal: 1234,56 -> 1234.56
                clean_str = clean_str.replace(',', '.')
            
            try:
                normalized_data.append(float(clean_str))
            except ValueError:
                continue
        else:
            normalized_data.append(float(item))
    
    # Analyze normalized international data
    results = law(
        "benford",
        normalized_data,
        confidence_level=0.95,
        enable_international_numerals=True
    )
    print_results(results)

def example_data_generation():
    """Generate test data following various statistical laws"""
    print_example(
        "Test Data Generation",
        "Generate synthetic datasets for testing and validation"
    )
    
    # Generate Benford's Law compliant data
    print("   Generating Benford's Law test data...")
    benford_config = {
        "law": "benford",
        "count": 1000,
        "seed": 42,
        "parameters": {
            "min_digits": 1,
            "max_digits": 6
        }
    }
    
    benford_results = law("generate", benford_config)
    print_results(benford_results)
    
    # Generate Pareto distribution data
    print("\n   Generating Pareto distribution test data...")
    pareto_config = {
        "law": "pareto", 
        "count": 500,
        "parameters": {
            "alpha": 1.16,  # Shape parameter
            "scale": 100
        }
    }
    
    pareto_results = law("generate", pareto_config)
    print_results(pareto_results)
    
    # Generate Zipf distribution data
    print("\n   Generating Zipf distribution test data...")
    zipf_config = {
        "law": "zipf",
        "count": 100,
        "parameters": {
            "s": 1.0,  # Zipf parameter
            "n": 100   # Number of elements
        }
    }
    
    zipf_results = law("generate", zipf_config)
    print_results(zipf_results)

def example_comprehensive_analysis():
    """Comprehensive multi-law analysis"""
    print_example(
        "Comprehensive Multi-Law Analysis",
        "Analyze dataset using multiple statistical laws automatically"
    )
    
    # Complex dataset with mixed characteristics
    complex_dataset = {
        "financial_transactions": generate_benford_data(800, seed=42),
        "sales_by_product": generate_pareto_data(200),
        "customer_ratings": [random.gauss(4.0, 1.0) for _ in range(500)],
        "event_counts": [random.poisson(3.5) for _ in range(300)]
    }
    
    # Comprehensive analysis using multiple laws
    comprehensive_results = law(
        "analyze",
        complex_dataset,
        laws_to_check=["benford", "pareto", "normal", "poisson"],
        confidence_level=0.95,
        include_metadata=True,
        detailed_report=True
    )
    print_results(comprehensive_results)

def example_performance_benchmarking():
    """Performance benchmarking with large datasets"""
    print_example(
        "Performance Benchmarking",
        "Test performance with large datasets and optimization"
    )
    
    dataset_sizes = [1000, 5000, 10000]
    
    for size in dataset_sizes:
        print(f"\n   Testing dataset size: {size:,}")
        
        # Generate large dataset
        large_dataset = generate_benford_data(size, seed=42)
        
        # Analyze with performance optimization
        import time
        start_time = time.time()
        
        results = law(
            "benford",
            large_dataset,
            enable_parallel_processing=True,
            memory_limit_mb=200,
            confidence_level=0.95
        )
        
        end_time = time.time()
        processing_time = end_time - start_time
        
        print(f"   Processing time: {processing_time:.3f} seconds")
        print(f"   Throughput: {size/processing_time:.0f} items/second")
        
        # Show results
        for result in results:
            if result.get('type') == 'BenfordAnalysis':
                risk = result.get('risk_level', 'unknown')
                print(f"   Analysis result: {risk} risk")

def example_error_handling():
    """Demonstrate error handling for invalid data"""
    print_example(
        "Error Handling and Edge Cases",
        "Handle various edge cases and invalid data gracefully"
    )
    
    # Test cases for edge conditions
    test_cases = [
        ([], "Empty dataset"),
        ([1], "Single value"),
        ([1, 1, 1, 1, 1], "All identical values"),
        (["invalid", "data"], "Non-numeric data"),
        ([0, 0, 0], "All zeros"),
        ([-1, -2, -3], "Negative numbers")
    ]
    
    for test_data, description in test_cases:
        print(f"\n   Testing: {description}")
        try:
            # Use diagnostic mode for edge cases
            results = law(
                "diagnose",
                test_data,
                confidence_level=0.95
            )
            
            for result in results:
                if result.get('type') == 'DiagnosticResult':
                    diagnostic_type = result.get('diagnostic_type', 'unknown')
                    confidence = result.get('confidence_level', 0)
                    print(f"   Diagnostic: {diagnostic_type} (confidence: {confidence:.2f})")
                    
        except Exception as e:
            print(f"   Expected error handled: {str(e)[:50]}...")

def main():
    """Run all examples"""
    print("=" * 70)
    print("lawkit-python Native API Examples - UNIFIED API DESIGN")
    print("=" * 70)
    print("\nAll examples use only the unified law() function.")
    print("Users prepare data themselves using standard Python libraries.")
    
    examples = [
        example_fraud_detection,
        example_business_data_quality,
        example_audit_compliance,
        example_real_time_monitoring,
        example_international_data,
        example_data_generation,
        example_comprehensive_analysis,
        example_performance_benchmarking,
        example_error_handling,
    ]
    
    for example_func in examples:
        try:
            print_header(f"Example: {example_func.__name__.replace('example_', '').replace('_', ' ').title()}")
            example_func()
        except Exception as e:
            print(f"\n❌ ERROR in {example_func.__name__}: {e}")
            import traceback
            traceback.print_exc()
    
    print_header("Summary")
    print("✅ All examples use the unified law() API only")
    print("📊 Users handle data preparation with standard libraries")  
    print("🔍 Comprehensive statistical law analysis")
    print("🚀 Ready for production fraud detection and audit systems")
    
    print("\nStatistical Law Analysis Benefits:")
    print("  • Benford's Law for fraud detection")
    print("  • Pareto analysis for business insights")
    print("  • Normal distribution testing")
    print("  • Poisson distribution analysis")
    print("  • Zipf's Law for natural phenomena")
    print("  • Multi-law comprehensive analysis")
    print("  • Real-time anomaly detection")
    
    print("\nBusiness Use Cases:")
    print("  • Financial audit and compliance")
    print("  • Fraud detection in transactions")
    print("  • Data quality assessment")
    print("  • Business intelligence analysis")
    print("  • Risk management systems")
    print("  • Regulatory compliance monitoring")
    print("  • Market research validation")
    
    print("\nFor more information:")
    print("  • Documentation: https://github.com/kako-jun/lawkit")
    print("  • PyPI Package: https://pypi.org/project/lawkit-python/")
    print("  • Issues: https://github.com/kako-jun/lawkit/issues")

if __name__ == "__main__":
    main()