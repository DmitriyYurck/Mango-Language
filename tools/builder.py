import os
import platform
import shutil
import subprocess
from pathlib import Path

def build_rust_project(project_path: Path):
    subprocess.run(["cargo", "build", "--release"], cwd=project_path, check=True)

def find_windows_install_path(binary_name: str) -> Path:
    path_dirs = os.environ["PATH"].split(";")
    for dir_path in path_dirs:
        p = Path(dir_path.strip())
        if p.exists() and os.access(p, os.W_OK):
            target = p / f"{binary_name}.exe"
            return target
    raise RuntimeError("No writable directory found in PATH")

def install_binary(project_path: Path, binary_name: str = "mangoi"):
    target_dir = project_path / "target" / "release"
    binary_path = target_dir / f"{binary_name}.exe" if platform.system() == "Windows" else target_dir / binary_name

    if not binary_path.exists():
        raise FileNotFoundError(f"Binary not found: {binary_path}")

    system = platform.system()
    if system == "Linux":
        install_path = Path("/usr/local/bin") / binary_name
    elif system == "Windows":
        install_path = find_windows_install_path(binary_name)
    else:
        raise RuntimeError(f"Unsupported OS: {system}")

    shutil.copy2(binary_path, install_path)

if __name__ == "__main__":
    rust_project_path = Path.cwd()
    binary_name = "mangoi"

    build_rust_project(rust_project_path)
    install_binary(rust_project_path, binary_name)
