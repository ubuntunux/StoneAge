#!/usr/bin/env python3
"""
StoneAge Deployment Script
Builds release binaries for Linux and Windows, packages necessary runtime DLLs,
executables, and filtered resources into a distribution zip (StoneAge.zip).
"""

import argparse
import os
import shutil
import subprocess
import sys
import time
import zipfile
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parent

# Target binaries
LINUX_BIN = ROOT_DIR / "target" / "release" / "stone_age"
WINDOWS_BIN = ROOT_DIR / "target" / "x86_64-pc-windows-gnu" / "release" / "stone_age.exe"
RESOURCES_DIR = ROOT_DIR / "resources"

# Allowed subdirectories directly under resources/externals/
ALLOWED_EXTERNALS = {"fonts", "sounds", "textures"}


def run_command(cmd: list[str], env: dict[str, str] | None = None, cwd: Path = ROOT_DIR) -> None:
    """Executes a command with real-time output and checks for errors."""
    print(f"\n[EXEC] {' '.join(cmd)}")
    result = subprocess.run(cmd, env=env, cwd=cwd)
    if result.returncode != 0:
        print(f"[ERROR] Command failed with exit code {result.returncode}: {' '.join(cmd)}", file=sys.stderr)
        sys.exit(result.returncode)


def compile_shaders() -> None:
    """
    Compiles engine and game shaders into SPIR-V shader caches and updates resources.txt
    by running the application with the '-- deploy' flag.
    In deploy mode, the application initializes resources, compiles shaders, and cleanly
    terminates before executing game updates.
    """
    print("=" * 60)
    print("Compiling Shaders & Preparing Resources (cargo run --release -- deploy)...")
    print("=" * 60)
    run_command(["cargo", "run", "--release", "--", "deploy"])
    print("\n[OK] Shader compilation and resource preparation completed successfully.")


def build_linux() -> None:
    """Builds the release binary for Linux."""
    print("=" * 60)
    print("Building Linux Release Binary (target/release/stone_age)...")
    print("=" * 60)
    run_command(["cargo", "build", "--release"])
    if not LINUX_BIN.is_file():
        print(f"[ERROR] Linux binary not found at {LINUX_BIN}", file=sys.stderr)
        sys.exit(1)
    print(f"[OK] Linux binary built successfully: {LINUX_BIN} ({LINUX_BIN.stat().st_size:,} bytes)")


def build_windows() -> None:
    """Builds the release binary for Windows using MinGW GNU toolchain."""
    print("=" * 60)
    print("Building Windows Release Binary (stone_age.exe)...")
    print("=" * 60)
    env = os.environ.copy()
    wine_lib = "/usr/lib/x86_64-linux-gnu/wine/x86_64-windows"
    rustflags = f"-L {ROOT_DIR} -L {wine_lib} -C link-arg=-lucrt"
    env["RUSTFLAGS"] = rustflags

    run_command(["cargo", "build", "--release", "--target", "x86_64-pc-windows-gnu"], env=env)
    if not WINDOWS_BIN.is_file():
        print(f"[ERROR] Windows binary not found at {WINDOWS_BIN}", file=sys.stderr)
        sys.exit(1)
    print(f"[OK] Windows binary built successfully: {WINDOWS_BIN} ({WINDOWS_BIN.stat().st_size:,} bytes)")


def should_include_resource(path: Path) -> bool:
    """
    Checks if a resource file/folder should be included in the distribution.
    Only 'fonts', 'sounds', 'textures' folders under resources/externals/ are allowed.
    Root files directly under resources/externals/ (e.g. metadata, txt) are preserved.
    """
    try:
        rel_path = path.relative_to(ROOT_DIR).as_posix()
    except ValueError:
        return False

    parts = Path(rel_path).parts
    if len(parts) >= 2 and parts[0] == "resources" and parts[1] == "externals":
        if len(parts) >= 3:
            sub_entry = parts[2]
            sub_full = ROOT_DIR / "resources" / "externals" / sub_entry
            if sub_full.is_dir():
                return sub_entry in ALLOWED_EXTERNALS
            else:
                return True
    return True


def collect_distribution_items() -> tuple[list[tuple[Path, str, int]], list[str]]:
    """
    Collects all files and empty directories to be included in the distribution zip.
    Returns:
        files: list of (source_path, archive_arcname, mode)
        directories: list of archive directory names
    """
    files: list[tuple[Path, str, int]] = []
    dirs: list[str] = ["resources/", "temp/"]

    # 1. Executables
    # stone_age (Linux binary - ensure executable permissions 0o755)
    files.append((LINUX_BIN, "stone_age", 0o755))
    # stone_age.exe (Windows binary)
    files.append((WINDOWS_BIN, "stone_age.exe", 0o755))

    # 2. Root DLL files
    root_dlls = sorted(list(ROOT_DIR.glob("*.dll")))
    if not root_dlls:
        print("[WARN] No .dll files found in root directory!")
    for dll in root_dlls:
        files.append((dll, dll.name, 0o644))

    # 3. Empty .log file
    # We will handle .log via an empty in-memory buffer or temp file

    # 4. Resources
    if not RESOURCES_DIR.is_dir():
        print(f"[ERROR] Resources directory not found at {RESOURCES_DIR}", file=sys.stderr)
        sys.exit(1)

    for root, dirnames, filenames in os.walk(RESOURCES_DIR):
        root_path = Path(root)

        # Directory entries
        for d in dirnames:
            dir_path = root_path / d
            if should_include_resource(dir_path):
                arc_dir = dir_path.relative_to(ROOT_DIR).as_posix() + "/"
                dirs.append(arc_dir)

        # File entries
        for f in filenames:
            file_path = root_path / f
            if should_include_resource(file_path):
                arc_file = file_path.relative_to(ROOT_DIR).as_posix()
                files.append((file_path, arc_file, 0o644))

    return files, dirs


def create_distribution_zip(output_zip: Path, files: list[tuple[Path, str, int]], dirs: list[str]) -> None:
    """Packages collected items into the target zip file."""
    print("=" * 60)
    print(f"Creating Distribution Zip: {output_zip.name}...")
    print("=" * 60)

    # Sort directories and files for deterministic packaging
    dirs = sorted(list(set(dirs)))
    files.sort(key=lambda x: x[1])

    start_time = time.time()
    total_uncompressed_bytes = 0

    with zipfile.ZipFile(output_zip, mode="w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
        # Add directories
        for d in dirs:
            zinfo = zipfile.ZipInfo(d)
            zinfo.external_attr = (0o755 | 0o040000) << 16  # drwxr-xr-x
            zf.writestr(zinfo, b"")

        # Add empty .log file
        log_info = zipfile.ZipInfo(".log")
        log_info.external_attr = 0o644 << 16
        zf.writestr(log_info, b"")

        # Add files
        total_count = len(files)
        for idx, (src_path, arcname, mode) in enumerate(files, 1):
            file_size = src_path.stat().st_size
            total_uncompressed_bytes += file_size

            zinfo = zipfile.ZipInfo(arcname)
            zinfo.external_attr = (mode | 0o100000) << 16  # S_IFREG | mode
            zinfo.compress_type = zipfile.ZIP_DEFLATED

            with open(src_path, "rb") as f:
                zf.writestr(zinfo, f.read())

            if idx % 500 == 0 or idx == total_count:
                print(f"  Packaging files: {idx}/{total_count} ({idx * 100 // total_count}%)")

    elapsed = time.time() - start_time
    zip_size = output_zip.stat().st_size
    print(f"\n[OK] Distribution zip successfully created:")
    print(f"  - Output: {output_zip}")
    print(f"  - Zip File Size: {zip_size:,} bytes ({zip_size / (1024 * 1024):.2f} MB)")
    print(f"  - Uncompressed Size: {total_uncompressed_bytes:,} bytes ({total_uncompressed_bytes / (1024 * 1024):.2f} MB)")
    print(f"  - Total Files: {len(files) + 1} (including .log)")
    print(f"  - Total Directories: {len(dirs)}")
    print(f"  - Compression Time: {elapsed:.2f} seconds")


def main() -> None:
    parser = argparse.ArgumentParser(description="Build and package StoneAge for distribution.")
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Skip release building and only package existing binaries and resources.",
    )
    parser.add_argument(
        "--skip-shader-compile",
        action="store_true",
        help="Skip shader compilation step.",
    )
    parser.add_argument(
        "--output",
        "-o",
        type=str,
        default="StoneAge.zip",
        help="Name or path of the output zip file (default: StoneAge.zip).",
    )
    args = parser.parse_args()

    total_start = time.time()
    output_zip = (ROOT_DIR / args.output).resolve()

    if not args.skip_shader_compile:
        compile_shaders()
    else:
        print("[INFO] Skipping shader compilation as requested (--skip-shader-compile).")

    if not args.skip_build:
        build_linux()
        build_windows()
    else:
        print("[INFO] Skipping build step as requested (--skip-build).")
        if not LINUX_BIN.is_file():
            print(f"[ERROR] Linux binary not found at {LINUX_BIN}. Run without --skip-build first.", file=sys.stderr)
            sys.exit(1)
        if not WINDOWS_BIN.is_file():
            print(f"[ERROR] Windows binary not found at {WINDOWS_BIN}. Run without --skip-build first.", file=sys.stderr)
            sys.exit(1)

    print("\nCollecting distribution files...")
    files, dirs = collect_distribution_items()

    print(f"Found {len(files)} files and {len(dirs)} directories to package.")
    create_distribution_zip(output_zip, files, dirs)

    total_elapsed = time.time() - total_start
    print("=" * 60)
    print(f"Deployment completed in {total_elapsed:.2f} seconds.")
    print("=" * 60)


if __name__ == "__main__":
    main()
