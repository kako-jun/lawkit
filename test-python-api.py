#!/usr/bin/env python3

import lawkit
import tempfile
import os

# Test data
test_data = """amount,category
123,A
456,B
789,A
321,C
654,B
987,A"""

def test_python_package():
    print("🧪 Testing Python package lawkit-python...")
    
    try:
        # Test 1: Version
        print("\n1. Testing version...")
        version = lawkit.get_version()
        print(f"Version: {version}")
        
        # Test 2: Availability check
        print("\n2. Testing availability...")
        available = lawkit.is_lawkit_available()
        print(f"Lawkit available: {available}")
        
        # Test 3: Self-test
        print("\n3. Testing self-test...")
        self_test = lawkit.selftest()
        print(f"Self-test passed: {self_test}")
        
        # Create temporary file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.csv', delete=False) as f:
            f.write(test_data)
            temp_file = f.name
        
        try:
            # Test 4: Benford analysis (text output)
            print("\n4. Testing Benford analysis (text)...")
            benf_text = lawkit.analyze_benford(temp_file)
            print(f"Benford text result length: {len(benf_text)}")
            
            # Test 5: Benford analysis (JSON output)
            print("\n5. Testing Benford analysis (JSON)...")
            options = lawkit.LawkitOptions(format='json')
            benf_json = lawkit.analyze_benford(temp_file, options)
            print(f"Benford JSON result type: {type(benf_json)}")
            if hasattr(benf_json, 'risk_level'):
                print(f"Risk level: {benf_json.risk_level}")
                print(f"P-value: {benf_json.p_value}")
            
            # Test 6: Pareto analysis
            print("\n6. Testing Pareto analysis...")
            pareto_options = lawkit.LawkitOptions(format='json', gini_coefficient=True)
            pareto_result = lawkit.analyze_pareto(temp_file, pareto_options)
            print(f"Pareto result type: {type(pareto_result)}")
            if hasattr(pareto_result, 'gini_coefficient'):
                print(f"Gini coefficient: {pareto_result.gini_coefficient}")
                print(f"80/20 concentration: {pareto_result.concentration_80_20}")
            
            # Test 7: String analysis
            print("\n7. Testing string analysis...")
            string_result = lawkit.analyze_string(test_data, 'benf', 
                                                 lawkit.LawkitOptions(format='json'))
            print(f"String analysis result type: {type(string_result)}")
            
            # Test 8: Data generation
            print("\n8. Testing data generation...")
            generated_data = lawkit.generate_data('benf', samples=100, seed=42)
            print(f"Generated data length: {len(generated_data)}")
            print(f"Contains numbers: {'1' in generated_data}")
            
            # Test 9: Multi-law analysis
            print("\n9. Testing multi-law analysis...")
            analyze_result = lawkit.analyze_laws(temp_file, lawkit.LawkitOptions(format='json', laws='benf,pareto'))
            print(f"Analysis result type: {type(analyze_result)}")
            
            # Test 10: Data validation
            print("\n10. Testing data validation...")
            validate_result = lawkit.validate_laws(temp_file, lawkit.LawkitOptions(format='json'))
            print(f"Validation result type: {type(validate_result)}")
            
            # Test 11: Conflict diagnosis
            print("\n11. Testing conflict diagnosis...")
            diagnose_result = lawkit.diagnose_laws(temp_file, lawkit.LawkitOptions(format='json'))
            print(f"Diagnosis result type: {type(diagnose_result)}")
            
        finally:
            # Clean up
            if os.path.exists(temp_file):
                os.unlink(temp_file)
        
        print("\n✅ All Python tests completed!")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == '__main__':
    test_python_package()