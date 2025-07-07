"""
lawkit: Python wrapper for the lawkit CLI tool

This package provides a Python interface to the lawkit CLI tool for statistical
law analysis including Benford's Law, Pareto principle, Zipf's Law, Normal distribution,
and Poisson distribution analysis. Perfect for fraud detection, data quality assessment,
and statistical analysis.
"""

from .lawkit import (
    analyze_benford,
    analyze_pareto,
    analyze_zipf,
    analyze_normal,
    analyze_poisson,
    compare_laws,
    generate_data,
    analyze_string,
    is_lawkit_available,
    get_version,
    selftest,
    LawkitOptions,
    LawkitResult,
    LawkitError,
    Format,
    OutputFormat,
    LawType,
)

# For backward compatibility and convenience
from .compat import run_lawkit

__version__ = "2.1.0"
__all__ = [
    # Main analysis functions
    "analyze_benford",
    "analyze_pareto", 
    "analyze_zipf",
    "analyze_normal",
    "analyze_poisson",
    "compare_laws",
    
    # Utility functions
    "generate_data",
    "analyze_string",
    "is_lawkit_available",
    "get_version",
    "selftest",
    
    # Types and classes
    "LawkitOptions",
    "LawkitResult",
    "LawkitError",
    "Format",
    "OutputFormat", 
    "LawType",
    
    # Backward compatibility
    "run_lawkit",
]