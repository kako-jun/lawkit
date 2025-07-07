#!/usr/bin/env python3
"""
Package verification script for lawkit-python

This script verifies that the lawkit-python package is properly structured
and ready for distribution.
"""

import sys
import os
from pathlib import Path

def check_file_exists(path: Path, description: str) -> bool:
    """Check if a file exists and report the result."""
    if path.exists():
        print(f"✅ {description}: {path}")
        return True
    else:
        print(f"❌ {description}: {path} (missing)")
        return False

def check_directory_structure():
    """Check that all required directories and files exist."""
    base_dir = Path(__file__).parent
    
    print("Checking package structure...")
    
    all_good = True
    
    # Check main files
    all_good &= check_file_exists(base_dir / "pyproject.toml", "Project configuration")
    all_good &= check_file_exists(base_dir / "README.md", "README file")
    all_good &= check_file_exists(base_dir / "src" / "lawkit" / "__init__.py", "Package init")
    all_good &= check_file_exists(base_dir / "src" / "lawkit" / "lawkit.py", "Main module")
    all_good &= check_file_exists(base_dir / "src" / "lawkit" / "installer.py", "Installer module")
    all_good &= check_file_exists(base_dir / "src" / "lawkit" / "compat.py", "Compatibility module")
    
    # Check directories
    all_good &= check_file_exists(base_dir / "src", "Source directory")
    all_good &= check_file_exists(base_dir / "src" / "lawkit", "Package directory")
    all_good &= check_file_exists(base_dir / "bin", "Binary directory")
    
    return all_good

def check_pyproject_toml():
    """Check that pyproject.toml has the correct structure."""
    pyproject_path = Path(__file__).parent / "pyproject.toml"
    
    if not pyproject_path.exists():
        print("❌ pyproject.toml not found")
        return False
    
    print("\nChecking pyproject.toml content...")
    
    content = pyproject_path.read_text()
    
    required_sections = [
        "[build-system]",
        "[project]",
        "[project.urls]",
        "[project.optional-dependencies]",
        "[project.scripts]",
        "[tool.hatch.build.targets.wheel]",
        "[tool.ruff]",
        "[tool.mypy]"
    ]
    
    missing_sections = []
    for section in required_sections:
        if section not in content:
            missing_sections.append(section)
    
    if missing_sections:
        print(f"❌ Missing sections in pyproject.toml: {missing_sections}")
        return False
    
    # Check specific values
    if 'name = "lawkit-python"' not in content:
        print("❌ Package name should be 'lawkit-python'")
        return False
    
    if 'version = "2.1.0"' not in content:
        print("❌ Version should be '2.1.0'")
        return False
    
    if 'lawkit-download-binary = "lawkit.installer:main"' not in content:
        print("❌ Binary installer script not properly configured")
        return False
    
    print("✅ pyproject.toml structure is correct")
    return True

def check_imports():
    """Check that all imports work correctly."""
    print("\nChecking imports...")
    
    # Add src to path
    src_path = Path(__file__).parent / "src"
    sys.path.insert(0, str(src_path))
    
    try:
        import lawkit
        print("✅ Main package imports successfully")
        
        # Check version
        if hasattr(lawkit, '__version__'):
            print(f"✅ Package version: {lawkit.__version__}")
        else:
            print("❌ Package version not found")
            return False
        
        # Check main functions
        required_functions = [
            'analyze_benford', 'analyze_pareto', 'analyze_zipf', 
            'analyze_normal', 'analyze_poisson', 'compare_laws',
            'generate_data', 'analyze_string', 'is_lawkit_available',
            'get_version', 'selftest'
        ]
        
        for func_name in required_functions:
            if hasattr(lawkit, func_name):
                print(f"✅ Function {func_name} available")
            else:
                print(f"❌ Function {func_name} not found")
                return False
        
        # Check classes
        required_classes = ['LawkitOptions', 'LawkitResult', 'LawkitError']
        for class_name in required_classes:
            if hasattr(lawkit, class_name):
                print(f"✅ Class {class_name} available")
            else:
                print(f"❌ Class {class_name} not found")
                return False
        
        # Test object creation
        options = lawkit.LawkitOptions(format="csv", output="json")
        print(f"✅ LawkitOptions creation works")
        
        return True
        
    except Exception as e:
        print(f"❌ Import error: {e}")
        return False

def main():
    """Main verification function."""
    print("Verifying lawkit-python package structure...\n")
    
    structure_ok = check_directory_structure()
    pyproject_ok = check_pyproject_toml()
    imports_ok = check_imports()
    
    print(f"\n{'='*50}")
    if structure_ok and pyproject_ok and imports_ok:
        print("✅ All checks passed! Package is ready for distribution.")
        return 0
    else:
        print("❌ Some checks failed. Please fix the issues above.")
        return 1

if __name__ == "__main__":
    sys.exit(main())