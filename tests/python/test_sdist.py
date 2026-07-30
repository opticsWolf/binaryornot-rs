"""Verify that the maturin wheel contains the expected module.

Adapted from the original test_sdist.py which checked that .pyc fixtures
survive the Python sdist build. For the Rust project we verify the
maturin wheel instead — it must contain the compiled .pyd/.so module.
"""

import zipfile
from pathlib import Path

import pytest

import binaryornot_rs

# Find the wheel in target/wheels/
_WHEEL_DIR = Path(__file__).parent.parent / "target" / "wheels"


def _find_wheel():
    """Find the latest binaryornot_rs wheel in target/wheels/."""
    if not _WHEEL_DIR.exists():
        pytest.skip("No wheels directory found — run 'maturin build --release --features pyo3-ext' first")
    wheels = sorted(_WHEEL_DIR.glob("binaryornot_rs-*.whl"))
    if not wheels:
        pytest.skip("No binaryornot_rs wheel found — run 'maturin build --release --features pyo3-ext' first")
    return wheels[-1]


class TestWheelContents:
    """Test that the maturin wheel is well-formed and contains the module."""

    def test_wheel_exists(self):
        """A wheel must have been built."""
        wheel = _find_wheel()
        assert wheel.exists(), f"Wheel not found at {wheel}"

    def test_wheel_is_valid_zip(self):
        """The wheel must be a valid zip file."""
        wheel = _find_wheel()
        assert zipfile.is_zipfile(wheel), f"{wheel} is not a valid zip file"

    def test_wheel_contains_module(self):
        """The wheel must contain the compiled binaryornot_rs module."""
        wheel = _find_wheel()
        with zipfile.ZipFile(wheel) as zf:
            names = zf.namelist()
        # The module may be .pyd (Windows) or .so (Linux/macOS)
        pyd_files = [n for n in names if n.startswith("binaryornot_rs") and (n.endswith(".pyd") or n.endswith(".so"))]
        assert pyd_files, (
            f"No binaryornot_rs module (.pyd/.so) found in wheel.\n"
            f"Files: {names}"
        )

    def test_wheel_contains_dist_info(self):
        """The wheel must contain standard dist-info metadata."""
        wheel = _find_wheel()
        with zipfile.ZipFile(wheel) as zf:
            names = zf.namelist()
        dist_info_dirs = [n for n in names if "dist-info" in n]
        assert dist_info_dirs, f"No dist-info metadata found in wheel.\nFiles: {names}"

    def test_wheel_metadata_has_name(self):
        """The WHEEL metadata must declare Name: binaryornot_rs."""
        wheel = _find_wheel()
        with zipfile.ZipFile(wheel) as zf:
            names = zf.namelist()
        wheel_meta = [n for n in names if n.endswith("dist-info/WHEEL")]
        assert wheel_meta, f"No WHEEL metadata found in {wheel}"
        with zipfile.ZipFile(wheel) as zf:
            content = zf.read(wheel_meta[0]).decode("utf-8")
        # WHEEL metadata doesn't carry the name — that's in METADATA
        meta_files = [n for n in names if n.endswith("dist-info/METADATA")]
        assert meta_files, f"No METADATA found in {wheel}"
        with zipfile.ZipFile(wheel) as zf:
            meta_content = zf.read(meta_files[0]).decode("utf-8")
        assert "binaryornot_rs" in meta_content, (
            f"METADATA does not mention binaryornot_rs:\n{meta_content}"
        )

    def test_module_loads_from_wheel(self):
        """The installed module must be importable and functional."""
        # If we got here, the module is already installed and importable
        assert binaryornot_rs.is_binary_string(b"Hello, world!") is False
        assert binaryornot_rs.is_binary_string(b"\x89PNG\r\n\x1a\n" + b"\x00" * 120) is True
