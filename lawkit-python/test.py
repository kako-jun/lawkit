#!/usr/bin/env python3
"""
Unified test runner for lawkit-python package.
Follows the 6-file test structure standard.
"""

import unittest
import sys
import os

def main():
    """Run all tests in the unified 6-file structure."""
    # Add the tests directory to the path
    test_dir = os.path.join(os.path.dirname(__file__), 'tests')
    if test_dir not in sys.path:
        sys.path.insert(0, test_dir)
    
    # Discover and run all tests
    loader = unittest.TestLoader()
    start_dir = test_dir
    suite = loader.discover(start_dir, pattern='test_*.py')
    
    # Run tests with detailed output
    runner = unittest.TextTestRunner(verbosity=2, buffer=True)
    result = runner.run(suite)
    
    # Print summary
    tests_run = result.testsRun
    failures = len(result.failures)
    errors = len(result.errors)
    skipped = len(result.skipped) if hasattr(result, 'skipped') else 0
    
    print(f"\\n" + "="*50)
    print(f"LAWKIT-PYTHON TEST SUMMARY")
    print(f"="*50)
    print(f"Tests run: {tests_run}")
    print(f"Failures: {failures}")
    print(f"Errors: {errors}")
    print(f"Skipped: {skipped}")
    print(f"Success rate: {((tests_run - failures - errors) / tests_run * 100):.1f}%" if tests_run > 0 else "0.0%")
    
    # Return appropriate exit code
    return 0 if (failures == 0 and errors == 0) else 1

if __name__ == "__main__":
    exit_code = main()
    sys.exit(exit_code)