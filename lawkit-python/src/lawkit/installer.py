#!/usr/bin/env python3
"""Binary installer for lawkit."""

import os
import platform
import shutil
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
import zipfile
from pathlib import Path

LAWKIT_VERSION = "2.1.0"


def get_platform_info():
    """Get platform-specific download information."""
    system = platform.system().lower()
    machine = platform.machine().lower()
    
    if system == "windows":
        return "lawkit-windows-x86_64.zip", "lawkit.exe"
    elif system == "darwin":
        if machine in ["arm64", "aarch64"]:
            return "lawkit-macos-aarch64.tar.gz", "lawkit"
        else:
            return "lawkit-macos-x86_64.tar.gz", "lawkit"
    elif system == "linux":
        if machine in ["arm64", "aarch64"]:
            return "lawkit-linux-aarch64.tar.gz", "lawkit"
        else:
            return "lawkit-linux-x86_64.tar.gz", "lawkit"
    else:
        raise RuntimeError(f"Unsupported platform: {system}-{machine}")


def download_file(url: str, dest_path: Path) -> None:
    """Download a file from URL to destination."""
    print(f"Downloading lawkit binary from {url}...")
    
    try:
        with urllib.request.urlopen(url) as response:
            with open(dest_path, 'wb') as dest_file:
                shutil.copyfileobj(response, dest_file)
    except Exception as e:
        raise RuntimeError(f"Failed to download {url}: {e}")


def extract_archive(archive_path: Path, extract_dir: Path) -> None:
    """Extract archive file."""
    print("Extracting binary...")
    
    if archive_path.suffix == '.zip':
        with zipfile.ZipFile(archive_path, 'r') as zip_file:
            zip_file.extractall(extract_dir)
    elif archive_path.name.endswith('.tar.gz'):
        with tarfile.open(archive_path, 'r:gz') as tar_file:
            tar_file.extractall(extract_dir)
    else:
        raise RuntimeError(f"Unsupported archive format: {archive_path}")


def main():
    """Main function to download and install lawkit binary."""
    try:
        # Get the package directory (where this script will be installed)
        if hasattr(sys, '_MEIPASS'):
            # If running from PyInstaller bundle
            package_dir = Path(sys._MEIPASS).parent
        else:
            # Normal installation
            package_dir = Path(__file__).parent.parent.parent
        
        bin_dir = package_dir / "bin"
        
        # Get platform info
        archive_name, binary_name = get_platform_info()
        binary_path = bin_dir / binary_name
        
        # Skip download if binary already exists
        if binary_path.exists():
            print("lawkit binary already exists, skipping download.")
            return 0
        
        # Create bin directory
        bin_dir.mkdir(exist_ok=True)
        
        # Download URL
        download_url = f"https://github.com/kako-jun/lawkit/releases/download/v{LAWKIT_VERSION}/{archive_name}"
        
        # Download to temporary file
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)
            archive_path = temp_path / archive_name
            
            download_file(download_url, archive_path)
            extract_archive(archive_path, bin_dir)
        
        # Make binary executable on Unix systems
        if platform.system() != "Windows":
            binary_path.chmod(0o755)
        
        print(f"✅ lawkit binary installed successfully at {binary_path}")
        
        # Test the binary
        try:
            result = subprocess.run([str(binary_path), "--version"], 
                                  capture_output=True, text=True, timeout=10)
            if result.returncode == 0:
                print(f"✅ lawkit binary is working correctly")
                print(f"   Version: {result.stdout.strip()}")
            else:
                print(f"⚠️  lawkit binary installed but version check failed")
        except (subprocess.TimeoutExpired, subprocess.CalledProcessError) as e:
            print(f"⚠️  lawkit binary installed but verification failed: {e}")
        
        return 0
        
    except Exception as error:
        print(f"❌ Failed to download lawkit binary: {error}")
        print("You may need to install lawkit manually from: https://github.com/kako-jun/lawkit/releases")
        print("Or build from source: https://github.com/kako-jun/lawkit")
        # Don't fail the installation, just warn
        return 0


if __name__ == "__main__":
    sys.exit(main())