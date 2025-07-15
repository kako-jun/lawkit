"""
Entry point for running lawkit as a module: python -m lawkit
"""

import sys
import subprocess
from .lawkit import _get_lawkit_binary_path, LawkitError


def main() -> None:
    """Main entry point for python -m lawkit"""
    try:
        lawkit_path = _get_lawkit_binary_path()
        # Pass all command line arguments to lawkit binary
        result = subprocess.run([lawkit_path] + sys.argv[1:])
        sys.exit(result.returncode)
    except LawkitError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(e.exit_code)
    except FileNotFoundError:
        print("Error: lawkit command not found. Please install lawkit CLI tool.", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()