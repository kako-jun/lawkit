"""
Main lawkit wrapper implementation
"""

import json
import subprocess
import tempfile
import os
import platform
from pathlib import Path
from typing import Union, List, Dict, Any, Optional, Literal
from dataclasses import dataclass


# Type definitions
Format = Literal["text", "json", "csv", "yaml", "toml", "xml"]
OutputFormat = Literal["text", "json", "csv", "yaml", "toml", "xml"]
LawType = Literal["benf", "pareto", "zipf", "normal", "poisson"]


@dataclass
class LawkitOptions:
    """Options for lawkit operations"""
    format: Optional[Format] = None
    output: Optional[OutputFormat] = None
    min_count: Optional[int] = None
    threshold: Optional[float] = None
    confidence: Optional[float] = None
    verbose: bool = False
    optimize: bool = False
    international: bool = False
    # Law-specific options
    gini_coefficient: bool = False
    percentiles: Optional[str] = None
    business_analysis: bool = False
    # Statistical options
    test_type: Optional[str] = None
    alpha: Optional[float] = None
    # Advanced options
    outlier_detection: bool = False
    time_series: bool = False
    parallel: bool = False
    memory_efficient: bool = False


class LawkitResult:
    """Result of a lawkit analysis operation"""
    def __init__(self, data: Dict[str, Any], law_type: str):
        self.data = data
        self.law_type = law_type
    
    @property
    def risk_level(self) -> Optional[str]:
        """Get risk level if present"""
        return self.data.get("risk_level")
    
    @property
    def p_value(self) -> Optional[float]:
        """Get p-value if present"""
        return self.data.get("p_value")
    
    @property
    def chi_square(self) -> Optional[float]:
        """Get chi-square statistic if present"""
        return self.data.get("chi_square")
    
    @property
    def mad(self) -> Optional[float]:
        """Get Mean Absolute Deviation if present"""
        return self.data.get("mad")
    
    @property
    def gini_coefficient(self) -> Optional[float]:
        """Get Gini coefficient if present"""
        return self.data.get("gini_coefficient")
    
    @property
    def concentration_80_20(self) -> Optional[float]:
        """Get 80/20 concentration if present"""
        return self.data.get("concentration_80_20")
    
    @property
    def exponent(self) -> Optional[float]:
        """Get Zipf exponent if present"""
        return self.data.get("exponent")
    
    @property
    def lambda_estimate(self) -> Optional[float]:
        """Get lambda estimate for Poisson distribution if present"""
        return self.data.get("lambda")
    
    @property
    def mean(self) -> Optional[float]:
        """Get mean if present"""
        return self.data.get("mean")
    
    @property
    def std_dev(self) -> Optional[float]:
        """Get standard deviation if present"""
        return self.data.get("std_dev")
    
    @property
    def outliers(self) -> Optional[List[Any]]:
        """Get outliers if present"""
        return self.data.get("outliers")
    
    @property
    def anomalies(self) -> Optional[List[Any]]:
        """Get anomalies if present"""  
        return self.data.get("anomalies")
    
    def __repr__(self) -> str:
        return f"LawkitResult(law_type='{self.law_type}', data={self.data})"


class LawkitError(Exception):
    """Error thrown when lawkit command fails"""
    def __init__(self, message: str, exit_code: int, stderr: str):
        super().__init__(message)
        self.exit_code = exit_code
        self.stderr = stderr


def _get_lawkit_binary_path() -> str:
    """Get the path to the lawkit binary"""
    # Check if local binary exists (installed via postinstall)
    package_dir = Path(__file__).parent.parent.parent
    binary_name = "lawkit.exe" if platform.system() == "Windows" else "lawkit"
    local_binary_path = package_dir / "bin" / binary_name
    
    if local_binary_path.exists():
        return str(local_binary_path)
    
    # Fall back to system PATH
    return "lawkit"


def _execute_lawkit(args: List[str]) -> tuple[str, str]:
    """Execute lawkit command and return stdout, stderr"""
    lawkit_path = _get_lawkit_binary_path()
    
    try:
        result = subprocess.run(
            [lawkit_path] + args,
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout, result.stderr
    except subprocess.CalledProcessError as e:
        raise LawkitError(
            f"lawkit exited with code {e.returncode}",
            e.returncode,
            e.stderr or ""
        )
    except FileNotFoundError:
        raise LawkitError(
            "lawkit command not found. Please install lawkit CLI tool.",
            -1,
            ""
        )


def analyze_benford(
    input_data: str,
    options: Optional[LawkitOptions] = None
) -> Union[str, LawkitResult]:
    """
    Analyze data using Benford's Law
    
    Args:
        input_data: Path to input file or '-' for stdin
        options: Analysis options
        
    Returns:
        String output for text format, or LawkitResult for JSON format
        
    Examples:
        >>> result = analyze_benford('financial_data.csv')
        >>> print(result)
        
        >>> json_result = analyze_benford('accounting.json', 
        ...                              LawkitOptions(format='json', output='json'))
        >>> print(f"Risk level: {json_result.risk_level}")
        >>> print(f"P-value: {json_result.p_value}")
    """
    if options is None:
        options = LawkitOptions()
    
    args = ["benf", input_data]
    
    # Add common options
    _add_common_options(args, options)
    
    stdout, stderr = _execute_lawkit(args)
    
    # If output format is JSON, parse the result
    if options.output == "json":
        try:
            json_data = json.loads(stdout)
            return LawkitResult(json_data, "benford")
        except json.JSONDecodeError as e:
            raise LawkitError(f"Failed to parse JSON output: {e}", -1, "")
    
    # Return raw output for other formats
    return stdout


def analyze_pareto(
    input_data: str,
    options: Optional[LawkitOptions] = None
) -> Union[str, LawkitResult]:
    """
    Analyze data using Pareto principle (80/20 rule)
    
    Args:
        input_data: Path to input file or '-' for stdin
        options: Analysis options
        
    Returns:
        String output for text format, or LawkitResult for JSON format
        
    Examples:
        >>> result = analyze_pareto('sales_data.csv')
        >>> print(result)
        
        >>> json_result = analyze_pareto('revenue.json', 
        ...                             LawkitOptions(output='json', gini_coefficient=True))
        >>> print(f"Gini coefficient: {json_result.gini_coefficient}")
        >>> print(f"80/20 concentration: {json_result.concentration_80_20}")
    """
    if options is None:
        options = LawkitOptions()
    
    args = ["pareto", input_data]
    
    # Add common options
    _add_common_options(args, options)
    
    # Add Pareto-specific options
    if options.gini_coefficient:
        args.append("--gini-coefficient")
    
    if options.percentiles:
        args.extend(["--percentiles", options.percentiles])
    
    if options.business_analysis:
        args.append("--business-analysis")
    
    stdout, stderr = _execute_lawkit(args)
    
    # If output format is JSON, parse the result
    if options.output == "json":
        try:
            json_data = json.loads(stdout)
            return LawkitResult(json_data, "pareto")
        except json.JSONDecodeError as e:
            raise LawkitError(f"Failed to parse JSON output: {e}", -1, "")
    
    # Return raw output for other formats
    return stdout


def analyze_zipf(
    input_data: str,
    options: Optional[LawkitOptions] = None
) -> Union[str, LawkitResult]:
    """
    Analyze data using Zipf's Law
    
    Args:
        input_data: Path to input file or '-' for stdin
        options: Analysis options
        
    Returns:
        String output for text format, or LawkitResult for JSON format
        
    Examples:
        >>> result = analyze_zipf('text_data.txt')
        >>> print(result)
        
        >>> json_result = analyze_zipf('word_frequencies.json', 
        ...                          LawkitOptions(output='json'))
        >>> print(f"Zipf exponent: {json_result.exponent}")
    """
    if options is None:
        options = LawkitOptions()
    
    args = ["zipf", input_data]
    
    # Add common options
    _add_common_options(args, options)
    
    stdout, stderr = _execute_lawkit(args)
    
    # If output format is JSON, parse the result
    if options.output == "json":
        try:
            json_data = json.loads(stdout)
            return LawkitResult(json_data, "zipf")
        except json.JSONDecodeError as e:
            raise LawkitError(f"Failed to parse JSON output: {e}", -1, "")
    
    # Return raw output for other formats
    return stdout


def analyze_normal(
    input_data: str,
    options: Optional[LawkitOptions] = None
) -> Union[str, LawkitResult]:
    """
    Analyze data for normal distribution
    
    Args:
        input_data: Path to input file or '-' for stdin
        options: Analysis options
        
    Returns:
        String output for text format, or LawkitResult for JSON format
        
    Examples:
        >>> result = analyze_normal('measurements.csv')
        >>> print(result)
        
        >>> json_result = analyze_normal('quality_data.json', 
        ...                             LawkitOptions(output='json', outlier_detection=True))
        >>> print(f"Mean: {json_result.mean}")
        >>> print(f"Standard deviation: {json_result.std_dev}")
        >>> print(f"Outliers: {json_result.outliers}")
    """
    if options is None:
        options = LawkitOptions()
    
    args = ["normal", input_data]
    
    # Add common options
    _add_common_options(args, options)
    
    # Add Normal-specific options
    if options.outlier_detection:
        args.append("--outlier-detection")
    
    if options.test_type:
        args.extend(["--test-type", options.test_type])
    
    stdout, stderr = _execute_lawkit(args)
    
    # If output format is JSON, parse the result
    if options.output == "json":
        try:
            json_data = json.loads(stdout)
            return LawkitResult(json_data, "normal")
        except json.JSONDecodeError as e:
            raise LawkitError(f"Failed to parse JSON output: {e}", -1, "")
    
    # Return raw output for other formats
    return stdout


def analyze_poisson(
    input_data: str,
    options: Optional[LawkitOptions] = None
) -> Union[str, LawkitResult]:
    """
    Analyze data using Poisson distribution
    
    Args:
        input_data: Path to input file or '-' for stdin
        options: Analysis options
        
    Returns:
        String output for text format, or LawkitResult for JSON format
        
    Examples:
        >>> result = analyze_poisson('event_counts.csv')
        >>> print(result)
        
        >>> json_result = analyze_poisson('incidents.json', 
        ...                              LawkitOptions(output='json'))
        >>> print(f"Lambda estimate: {json_result.lambda_estimate}")
    """
    if options is None:
        options = LawkitOptions()
    
    args = ["poisson", input_data]
    
    # Add common options
    _add_common_options(args, options)
    
    stdout, stderr = _execute_lawkit(args)
    
    # If output format is JSON, parse the result
    if options.output == "json":
        try:
            json_data = json.loads(stdout)
            return LawkitResult(json_data, "poisson")
        except json.JSONDecodeError as e:
            raise LawkitError(f"Failed to parse JSON output: {e}", -1, "")
    
    # Return raw output for other formats
    return stdout


def compare_laws(
    input_data: str,
    options: Optional[LawkitOptions] = None
) -> Union[str, LawkitResult]:
    """
    Compare multiple statistical laws on the same data
    
    Args:
        input_data: Path to input file or '-' for stdin
        options: Analysis options
        
    Returns:
        String output for text format, or LawkitResult for JSON format
        
    Examples:
        >>> result = compare_laws('dataset.csv')
        >>> print(result)
        
        >>> json_result = compare_laws('complex_data.json', 
        ...                          LawkitOptions(output='json'))
        >>> print(f"Risk level: {json_result.risk_level}")
    """
    if options is None:
        options = LawkitOptions()
    
    args = ["compare", input_data]
    
    # Add common options
    _add_common_options(args, options)
    
    stdout, stderr = _execute_lawkit(args)
    
    # If output format is JSON, parse the result
    if options.output == "json":
        try:
            json_data = json.loads(stdout)
            return LawkitResult(json_data, "compare")
        except json.JSONDecodeError as e:
            raise LawkitError(f"Failed to parse JSON output: {e}", -1, "")
    
    # Return raw output for other formats
    return stdout


def generate_data(
    law_type: LawType,
    samples: int = 1000,
    seed: Optional[int] = None,
    **kwargs
) -> str:
    """
    Generate sample data following a specific statistical law
    
    Args:
        law_type: Type of statistical law to use
        samples: Number of samples to generate
        seed: Random seed for reproducibility
        **kwargs: Law-specific parameters
        
    Returns:
        Generated data as string
        
    Examples:
        >>> data = generate_data('benf', samples=1000, seed=42)
        >>> print(data)
        
        >>> normal_data = generate_data('normal', samples=500, mean=100, stddev=15)
        >>> pareto_data = generate_data('pareto', samples=1000, concentration=0.8)
    """
    args = ["generate", law_type, "--samples", str(samples)]
    
    if seed is not None:
        args.extend(["--seed", str(seed)])
    
    # Add law-specific parameters
    for key, value in kwargs.items():
        key_formatted = key.replace("_", "-")
        args.extend([f"--{key_formatted}", str(value)])
    
    stdout, stderr = _execute_lawkit(args)
    return stdout


def analyze_string(
    content: str,
    law_type: LawType,
    options: Optional[LawkitOptions] = None
) -> Union[str, LawkitResult]:
    """
    Analyze string data directly (writes to temporary file)
    
    Args:
        content: Data content as string
        law_type: Type of statistical law to use
        options: Analysis options
        
    Returns:
        String output for text format, or LawkitResult for JSON format
        
    Examples:
        >>> csv_data = "amount\\n123\\n456\\n789"
        >>> result = analyze_string(csv_data, 'benf', 
        ...                        LawkitOptions(format='csv', output='json'))
        >>> print(result.risk_level)
    """
    if options is None:
        options = LawkitOptions()
    
    # Create temporary file
    with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.csv') as tmp_file:
        tmp_file.write(content)
        tmp_file_path = tmp_file.name
    
    try:
        # Analyze the temporary file
        if law_type == "benf":
            return analyze_benford(tmp_file_path, options)
        elif law_type == "pareto":
            return analyze_pareto(tmp_file_path, options)
        elif law_type == "zipf":
            return analyze_zipf(tmp_file_path, options)
        elif law_type == "normal":
            return analyze_normal(tmp_file_path, options)
        elif law_type == "poisson":
            return analyze_poisson(tmp_file_path, options)
        else:
            raise LawkitError(f"Unknown law type: {law_type}", -1, "")
    finally:
        # Clean up temporary file
        os.unlink(tmp_file_path)


def _add_common_options(args: List[str], options: LawkitOptions) -> None:
    """Add common options to command arguments"""
    if options.format:
        args.extend(["--format", options.format])
    
    if options.output:
        args.extend(["--output", options.output])
    
    if options.min_count is not None:
        args.extend(["--min-count", str(options.min_count)])
    
    if options.threshold is not None:
        args.extend(["--threshold", str(options.threshold)])
    
    if options.confidence is not None:
        args.extend(["--confidence", str(options.confidence)])
    
    if options.alpha is not None:
        args.extend(["--alpha", str(options.alpha)])
    
    if options.verbose:
        args.append("--verbose")
    
    if options.optimize:
        args.append("--optimize")
    
    if options.international:
        args.append("--international")
    
    if options.time_series:
        args.append("--time-series")
    
    if options.parallel:
        args.append("--parallel")
    
    if options.memory_efficient:
        args.append("--memory-efficient")


def is_lawkit_available() -> bool:
    """
    Check if lawkit command is available in the system
    
    Returns:
        True if lawkit is available, False otherwise
        
    Examples:
        >>> if not is_lawkit_available():
        ...     print("Please install lawkit CLI tool")
        ...     exit(1)
    """
    try:
        _execute_lawkit(["--version"])
        return True
    except LawkitError:
        return False


def get_version() -> str:
    """
    Get the version of the lawkit CLI tool
    
    Returns:
        Version string
        
    Examples:
        >>> version = get_version()
        >>> print(f"Using lawkit version: {version}")
    """
    try:
        stdout, stderr = _execute_lawkit(["--version"])
        return stdout.strip()
    except LawkitError:
        return "Unknown"


def selftest() -> bool:
    """
    Run lawkit self-test to verify installation
    
    Returns:
        True if self-test passes, False otherwise
        
    Examples:
        >>> if not selftest():
        ...     print("lawkit self-test failed")
        ...     exit(1)
    """
    try:
        _execute_lawkit(["selftest"])
        return True
    except LawkitError:
        return False