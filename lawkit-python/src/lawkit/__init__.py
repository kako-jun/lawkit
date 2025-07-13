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
    analyze_laws,
    validate_laws,
    diagnose_laws,
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


# Version is now managed dynamically from pyproject.toml
# This prevents hardcoded version mismatches during releases
try:
    from importlib.metadata import version
    __version__ = version("lawkit-python")
except ImportError:
    # Fallback for Python < 3.8
    try:
        import pkg_resources
        __version__ = pkg_resources.get_distribution("lawkit-python").version
    except Exception:
        __version__ = "unknown"
__all__ = [
    # Main analysis functions
    "analyze_benford",
    "analyze_pareto", 
    "analyze_zipf",
    "analyze_normal",
    "analyze_poisson",
    "analyze_laws",
    "validate_laws",
    "diagnose_laws",
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
    
]