"""
Backward compatibility module for lawkit-python

This module provides compatibility functions for users migrating from 
other statistical analysis tools or expecting different API patterns.
"""

import subprocess
import platform
from pathlib import Path
from typing import List, Union


class LawkitProcess:
    """Compatibility class that mimics subprocess.CompletedProcess"""
    def __init__(self, returncode: int, stdout: str, stderr: str):
        self.returncode = returncode
        self.stdout = stdout
        self.stderr = stderr


def run_lawkit(args: List[str], input_data: Union[str, None] = None) -> LawkitProcess:
    """
    Run lawkit command with arguments (legacy compatibility function)
    
    Args:
        args: Command line arguments (without 'lawkit' prefix)
        input_data: Optional input data to pass via stdin
        
    Returns:
        LawkitProcess object with returncode, stdout, stderr
        
    Examples:
        >>> result = run_lawkit(["benf", "data.csv"])
        >>> if result.returncode == 0:
        ...     print("Analysis successful")
        ...     print(result.stdout)
        ... else:
        ...     print("Analysis failed")
        ...     print(result.stderr)
        
        >>> # With input data
        >>> csv_data = "amount\\n123\\n456\\n789"
        >>> result = run_lawkit(["benf", "-"], input_data=csv_data)
    """
    # Get the path to the lawkit binary
    package_dir = Path(__file__).parent.parent.parent
    binary_name = "lawkit.exe" if platform.system() == "Windows" else "lawkit"
    local_binary_path = package_dir / "bin" / binary_name
    
    if local_binary_path.exists():
        lawkit_path = str(local_binary_path)
    else:
        lawkit_path = "lawkit"
    
    try:
        # Run the command
        result = subprocess.run(
            [lawkit_path] + args,
            input=input_data,
            capture_output=True,
            text=True,
            timeout=300  # 5 minute timeout
        )
        
        return LawkitProcess(
            returncode=result.returncode,
            stdout=result.stdout,
            stderr=result.stderr
        )
        
    except subprocess.TimeoutExpired:
        return LawkitProcess(
            returncode=-1,
            stdout="",
            stderr="Command timed out after 5 minutes"
        )
    except FileNotFoundError:
        return LawkitProcess(
            returncode=-1,
            stdout="",
            stderr="lawkit command not found. Please install lawkit CLI tool."
        )
    except Exception as e:
        return LawkitProcess(
            returncode=-1,
            stdout="",
            stderr=f"Error running lawkit: {e}"
        )


def run_benford_analysis(file_path: str, **kwargs) -> LawkitProcess:
    """
    Legacy function for Benford's Law analysis
    
    Args:
        file_path: Path to input file
        **kwargs: Additional options (format, output, etc.)
        
    Returns:
        LawkitProcess object
        
    Examples:
        >>> result = run_benford_analysis("data.csv", format="csv", output="json")
    """
    args = ["benf", file_path]
    
    if "format" in kwargs:
        args.extend(["--format", kwargs["format"]])
    
    if "output" in kwargs:
        args.extend(["--output", kwargs["output"]])
    
    if "min_count" in kwargs:
        args.extend(["--min-count", str(kwargs["min_count"])])
    
    if "threshold" in kwargs:
        args.extend(["--threshold", str(kwargs["threshold"])])
    
    if kwargs.get("verbose", False):
        args.append("--verbose")
    
    if kwargs.get("optimize", False):
        args.append("--optimize")
    
    return run_lawkit(args)


def run_pareto_analysis(file_path: str, **kwargs) -> LawkitProcess:
    """
    Legacy function for Pareto principle analysis
    
    Args:
        file_path: Path to input file
        **kwargs: Additional options
        
    Returns:
        LawkitProcess object
        
    Examples:
        >>> result = run_pareto_analysis("sales.csv", gini_coefficient=True)
    """
    args = ["pareto", file_path]
    
    if "format" in kwargs:
        args.extend(["--format", kwargs["format"]])
    
    if "output" in kwargs:
        args.extend(["--output", kwargs["output"]])
    
    if kwargs.get("gini_coefficient", False):
        args.append("--gini-coefficient")
    
    if "percentiles" in kwargs:
        args.extend(["--percentiles", kwargs["percentiles"]])
    
    if kwargs.get("business_analysis", False):
        args.append("--business-analysis")
    
    if kwargs.get("verbose", False):
        args.append("--verbose")
    
    return run_lawkit(args)


def check_lawkit_installation() -> bool:
    """
    Check if lawkit is properly installed
    
    Returns:
        True if lawkit is available, False otherwise
        
    Examples:
        >>> if not check_lawkit_installation():
        ...     print("Please install lawkit first")
        ...     exit(1)
    """
    result = run_lawkit(["--version"])
    return result.returncode == 0


def get_lawkit_help(subcommand: str = None) -> str:
    """
    Get help text for lawkit or a specific subcommand
    
    Args:
        subcommand: Optional subcommand name
        
    Returns:
        Help text as string
        
    Examples:
        >>> help_text = get_lawkit_help()
        >>> print(help_text)
        
        >>> benf_help = get_lawkit_help("benf")
        >>> print(benf_help)
    """
    if subcommand:
        result = run_lawkit([subcommand, "--help"])
    else:
        result = run_lawkit(["--help"])
    
    return result.stdout if result.returncode == 0 else result.stderr