import sys
import subprocess
import platform
from pathlib import Path
import shutil


def find_bundled_binary():
    """Find the bundled binary in the package"""
    pkg_dir = Path(__file__).parent
    binary_name = "ruvo.exe" if platform.system() == "Windows" else "ruvo"
    binary_path = pkg_dir / "bin" / binary_name

    if binary_path.exists():
        return binary_path

    # If not found in package, try PATH
    binary_in_path = shutil.which(binary_name)
    if binary_in_path:
        return Path(binary_in_path)

    raise FileNotFoundError(
            f"Could not find ruvo binary. Please ensure the package is installed correctly."
    )


def main():
    """Entry point for the Python wrapper"""
    try:
        binary_path = find_bundled_binary()
        # Forward all arguments to the Rust binary
        result = subprocess.run([str(binary_path)] + sys.argv[1:])
        sys.exit(result.returncode)
    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error running ruvo: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
