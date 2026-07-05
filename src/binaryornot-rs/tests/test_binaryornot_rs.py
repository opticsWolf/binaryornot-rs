#!/usr/bin/env python
"""
test_binaryornot_rs
-------------------

Tests for the `binaryornot_rs` PyO3 module.

Mirrors the original Python test suite from binaryornot/tests/test_check.py,
adapted to use the Rust PyO3 bindings instead of the pure Python implementation.
"""

import logging
import os
import unittest
from contextlib import contextmanager
from pathlib import Path
from tempfile import mkstemp

import binaryornot_rs

logging.basicConfig(format="%(levelname)s: %(message)s", level=logging.INFO)

# Base directory for test fixture files (from the original Python project)
BASE_DIR = Path(__file__).parent.parent.parent / "tests"


@contextmanager
def bytes_in_file(data):
    """Write bytes to a temp file and yield its path."""
    o, f = mkstemp()
    try:
        os.write(o, data)
        os.close(o)
        yield f
    finally:
        os.unlink(f)


class TestIsBinary(unittest.TestCase):
    """Test is_binary() with various files."""

    def test_empty(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "files/empty.txt")))

    def test_triggers_decoding_error(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/decoding-error")))

    def test_triggers_lookup_error(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/lookup-error")))

    def test_ds_store(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/.DS_Store")))

    def test_txt(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "files/robots.txt")))

    def test_txt_unicode(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "files/unicode.txt")))

    def test_binary_pdf2(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/pdf.pdf")))

    def test_text_russian2(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/russian_file.rst")))

    def test_binary_exe2(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/grep")))

    def test_negative_binary(self):
        # A text file named .pyc is detected as binary by extension.
        # With check_extensions=False, content detection classifies it as text.
        path = str(BASE_DIR / "isBinaryFile/this_is_not_a_bin.pyc")
        self.assertTrue(binaryornot_rs.is_binary(path))
        self.assertFalse(binaryornot_rs.is_binary(path, check_extensions=False))

    def test_binary_sqlite(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/test.sqlite")))

    def test_binary_png_issue_642(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/issue-642.png")))


class TestFontFiles(unittest.TestCase):
    """Test is_binary() with various font file types."""

    def test_eot(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/glyphiconshalflings-regular.eot")))

    def test_otf(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/glyphiconshalflings-regular.otf")))

    def test_ttf(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/glyphiconshalflings-regular.ttf")))

    def test_woff(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/glyphiconshalflings-regular.woff")))


class TestImageFiles(unittest.TestCase):
    """Test is_binary() with various image file types."""

    def test_png(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/logo.png")))

    def test_gif(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/lena.gif")))

    def test_jpg(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/lena.jpg")))

    def test_tiff(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/palette-1c-8b.tiff")))

    def test_bmp(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/rgb-3c-8b.bmp")))

    def test_binary_rgb_stream(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/pixelstream.rgb")))

    def test_binary_gif2(self):
        # Empty file named .gif: extension check says binary, content check says text.
        path = str(BASE_DIR / "isBinaryFile/null_file.gif")
        self.assertTrue(binaryornot_rs.is_binary(path))
        self.assertFalse(binaryornot_rs.is_binary(path, check_extensions=False))

    def test_binary_gif3(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/trunks.gif")))

    def test_svg(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "files/glyphiconshalflings-regular.svg")))


class TestEncodings(unittest.TestCase):
    """Test is_binary() with files containing various encodings."""

    def test_text_utf16(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/bom_utf-16.txt")))

    def test_text_utf16le(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/bom_utf-16le.txt")))

    def test_text_utf16be(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/test-utf16be.txt")))

    def test_text_utf32le(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/bom_utf-32le.txt")))

    def test_text_utf82(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/utf_8.txt")))

    def test_text_gb2(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/test-gb2.txt")))

    def test_text_kr(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/test-kr.txt")))

    def test_text_latin(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/test-latin.txt")))

    def test_text_big5(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/big5.txt")))

    def test_text_gb(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/test-gb.txt")))

    def test_text_utf32(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/bom_utf-32.txt")))

    def test_text_utf8(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/bom_utf-8.txt")))

    def test_text_big5b(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/big5_B.txt")))

    def test_text_shishi(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/test-shishi.txt")))

    def test_text_utfcn(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/encodings/utf8cn.txt")))


class TestCodeFiles(unittest.TestCase):
    """Test is_binary() with various code file types."""

    def test_css(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "files/bootstrap-glyphicons.css")))

    def test_json(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "files/cookiecutter.json")))

    def test_text_perl2(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/perl_script")))

    def test_text_js(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/index.js")))

    def test_text_lua(self):
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/no.lua")))


class TestProgrammingArtifacts(unittest.TestCase):
    """Test is_binary() with various leftover byproducts from running or
    building programs."""

    def test_binary_pyc(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/hello_world.pyc")))

    def test_binary_empty_pyc(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/empty.pyc")))

    def test_binary_troublesome_pyc(self):
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/troublesome.pyc")))


class TestErrorHandling(unittest.TestCase):
    """Test is_binary() error behavior."""

    def test_nonexistent_file_raises(self):
        with self.assertRaises(OSError):
            binaryornot_rs.is_binary(str(BASE_DIR / "files/this_file_does_not_exist.txt"))


class TestMagicBytesGuard(unittest.TestCase):
    """Test that known binary file signatures bypass the decision tree."""

    def test_png_signature_with_adversarial_content(self):
        # First 128 bytes of issue-642.png — the tree misclassifies this as text
        chunk = (
            b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x02\x00"
            b"\x00\x00\x02\x00\x08\x04\x00\x00\x00^q\x1cq\x00\x00"
            b"\x00\x04gAMA\x00\x00\xb1\x8f\x0b\xfca\x05\x00\x00"
            b"\x00 cHRM\x00\x00z&\x00\x00\x80\x84\x00\x00\xfa\x00"
            b"\x00\x00\x80\xe8\x00\x00u0\x00\x00\xea`\x00\x00:\x98"
            b"\x00\x00\x17p\x9c\xbaQ<\x00\x00\x00\x02bKGD\x00\xff"
            b"\x87\x8f\xcc\xbf\x00\x00\x00\x07tIME\x07\xe4\x07\x0e"
            b"\x0b\x07\t)6\x99\x95\x00\x00"
        )
        self.assertTrue(binaryornot_rs.is_binary_string(chunk))

    def test_plain_text_not_caught(self):
        chunk = b"Hello, world! This is a plain text file with enough content.\n" * 2
        self.assertFalse(binaryornot_rs.is_binary_string(chunk[:128]))


class TestExtensionCheck(unittest.TestCase):
    """Test that known binary extensions are detected without reading the file."""

    def test_pyc_detected_by_extension(self):
        """A .pyc file is detected as binary by its extension."""
        with bytes_in_file(b"This is plain text content, not real bytecode.") as f:
            pyc_path = f + ".pyc"
            os.rename(f, pyc_path)
            try:
                self.assertTrue(binaryornot_rs.is_binary(pyc_path))
            finally:
                os.rename(pyc_path, f)

    def test_png_detected_by_extension(self):
        """A .png file is detected as binary by its extension."""
        with bytes_in_file(b"Not actually a PNG, just text.") as f:
            png_path = f + ".png"
            os.rename(f, png_path)
            try:
                self.assertTrue(binaryornot_rs.is_binary(png_path))
            finally:
                os.rename(png_path, f)

    def test_extension_check_disabled(self):
        """With check_extensions=False, a text file with a binary extension is classified by content."""
        with bytes_in_file(b"This is plain text content, not real bytecode.\n" * 5) as f:
            pyc_path = f + ".pyc"
            os.rename(f, pyc_path)
            try:
                self.assertFalse(binaryornot_rs.is_binary(pyc_path, check_extensions=False))
            finally:
                os.rename(pyc_path, f)

    def test_text_extensions_not_affected(self):
        """Text file extensions like .txt and .py are not in the binary list."""
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "files/robots.txt")))
        self.assertFalse(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/index.js")))

    def test_no_extension_falls_through(self):
        """Files without extensions fall through to content detection."""
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "isBinaryFile/grep")))

    def test_pathlib_path_works(self):
        """Extension check works with pathlib.Path objects (converted to str)."""
        self.assertTrue(binaryornot_rs.is_binary(str(BASE_DIR / "files/logo.png")))

    def test_extension_case_insensitive(self):
        """Extension check is case-insensitive (.PNG == .png)."""
        with bytes_in_file(b"Not actually a PNG.") as f:
            upper_path = f + ".PNG"
            os.rename(f, upper_path)
            try:
                self.assertTrue(binaryornot_rs.is_binary(upper_path))
            finally:
                os.rename(upper_path, f)


class TestVersion(unittest.TestCase):
    """Test that __version__ is exposed."""

    def test_version_is_string(self):
        self.assertIsInstance(binaryornot_rs.__version__, str)

    def test_version_matches_cargo_toml(self):
        # Version should be semver
        parts = binaryornot_rs.__version__.split(".")
        self.assertEqual(len(parts), 3)
        for part in parts:
            int(part)  # must be valid integer


class TestModuleAPI(unittest.TestCase):
    """Test that the expected public API is present."""

    def test_has_is_binary(self):
        self.assertTrue(hasattr(binaryornot_rs, "is_binary"))
        self.assertTrue(callable(binaryornot_rs.is_binary))

    def test_has_is_binary_string(self):
        self.assertTrue(hasattr(binaryornot_rs, "is_binary_string"))
        self.assertTrue(callable(binaryornot_rs.is_binary_string))

    def test_has_version(self):
        self.assertTrue(hasattr(binaryornot_rs, "__version__"))


if __name__ == "__main__":
    unittest.main()
