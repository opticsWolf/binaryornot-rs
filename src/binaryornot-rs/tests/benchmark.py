#!/usr/bin/env python
"""
Benchmark: binaryornot_rs (PyO3/Rust) vs binaryornot (pure Python)

Runs the same file-based and string-based tests against both implementations,
10 iterations each, and reports average times + speedup ratios.
"""

import os
import statistics
import timeit
from pathlib import Path

from binaryornot.check import is_binary as is_binary_py
from binaryornot.helpers import is_binary_string as is_binary_string_py

import binaryornot_rs

# ---------------------------------------------------------------------------
# Paths to the same fixture files used in the test suite
# ---------------------------------------------------------------------------
BASE_DIR = Path(__file__).parent.parent.parent / "tests"

FILE_TESTS = [
    # (description, relative_path, expected)
    ("empty.txt", "files/empty.txt", False),
    ("robots.txt", "files/robots.txt", False),
    ("unicode.txt", "files/unicode.txt", False),
    ("logo.png", "files/logo.png", True),
    ("lena.jpg", "files/lena.jpg", True),
    ("lena.gif", "files/lena.gif", True),
    ("palette-1c-8b.tiff", "files/palette-1c-8b.tiff", True),
    ("rgb-3c-8b.bmp", "files/rgb-3c-8b.bmp", True),
    ("hello_world.pyc", "files/hello_world.pyc", True),
    ("pdf.pdf", "isBinaryFile/pdf.pdf", True),
    ("russian_file.rst", "isBinaryFile/russian_file.rst", False),
    ("grep", "isBinaryFile/grep", True),
    ("test.sqlite", "isBinaryFile/test.sqlite", True),
    ("bom_utf-16.txt", "isBinaryFile/encodings/bom_utf-16.txt", False),
    ("bom_utf-32le.txt", "isBinaryFile/encodings/bom_utf-32le.txt", False),
    ("test-utf16be.txt", "isBinaryFile/encodings/test-utf16be.txt", False),
    ("utf_8.txt", "isBinaryFile/encodings/utf_8.txt", False),
    ("test-gb.txt", "isBinaryFile/encodings/test-gb.txt", False),
    ("big5.txt", "isBinaryFile/encodings/big5.txt", False),
    ("test-kr.txt", "isBinaryFile/encodings/test-kr.txt", False),
    ("test-latin.txt", "isBinaryFile/encodings/test-latin.txt", False),
    ("bootstrap-glyphicons.css", "files/bootstrap-glyphicons.css", False),
    ("cookiecutter.json", "files/cookiecutter.json", False),
    ("index.js", "isBinaryFile/index.js", False),
    ("no.lua", "isBinaryFile/no.lua", False),
    ("glyphicons-regular.svg", "files/glyphiconshalflings-regular.svg", False),
    ("glyphicons-regular.ttf", "files/glyphiconshalflings-regular.ttf", True),
    ("glyphicons-regular.woff", "files/glyphiconshalflings-regular.woff", True),
    ("glyphicons-regular.otf", "files/glyphiconshalflings-regular.otf", True),
    ("glyphicons-regular.eot", "files/glyphiconshalflings-regular.eot", True),
    ("issue-642.png", "files/issue-642.png", True),
    ("decoding-error", "files/decoding-error", True),
    ("lookup-error", "files/lookup-error", True),
    (".DS_Store", "files/.DS_Store", True),
    ("glyphicons-regular.css", "files/bootstrap-glyphicons.css", False),
]

# String-based test chunks
STRING_TESTS = [
    ("PNG magic + adversarial",
     b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x02\x00"
     b"\x00\x00\x02\x00\x08\x04\x00\x00\x00^q\x1cq\x00\x00"
     b"\x00\x00\x04gAMA\x00\x00\xb1\x8f\x0b\xfca\x05\x00\x00"
     b"\x00 cHRM\x00\x00z&\x00\x00\x80\x84\x00\x00\xfa\x00"
     b"\x00\x00\x80\xe8\x00\x00u0\x00\x00\xea`\x00\x00:\x98"
     b"\x00\x00\x17p\x9c\xbaQ<\x00\x00\x00\x02bKGD\x00\xff"
     b"\x87\x8f\xcc\xbf\x00\x00\x00\x07tIME\x07\xe4\x07\x0e"
     b"\x0b\x07\t)6\x99\x95\x00\x00",
     True),
    ("Plain text",
     b"Hello, world! This is a plain text file with enough content.\n" * 2,
     False),
    ("UTF-16LE BOM",
     "Hello, world! This is UTF-16LE text for benchmarking.".encode("utf-16-le"),
     False),
    ("UTF-32 BOM",
     "Hello, world! This is UTF-32 text for benchmarking.".encode("utf-32"),
     False),
    ("High-entropy binary",
     bytes(range(256)) * 2,
     True),
    ("All nulls",
     b"\x00" * 128,
     True),
    ("GBK text",
     "这是一个GBK编码的测试文本，用于基准测试。".encode("gbk"),
     False),
    ("Big5 text",
     "這是Big5編碼的測試文本，用於基準測試。".encode("big5"),
     False),
    ("Shift-JIS text",
     "これはShift-JISエンコードのテストテキストです。".encode("shift-jis"),
     False),
    ("Latin-1 text",
     "Café résumé naïve über straße".encode("latin-1"),
     False),
]

ITERATIONS = 10
TIMEIT_NUMBER = 100  # calls per iteration


def run_benchmarks():
    """Run file-based and string-based benchmarks against both implementations."""

    # ------------------------------------------------------------------
    # File-based: is_binary(path)
    # ------------------------------------------------------------------
    print("=" * 78)
    print("  FILE-BASED: is_binary(path)")
    print("=" * 78)
    print(f"  {'Test':<45} {'Python (ms)':>12} {'Rust (ms)':>12} {'Speedup':>10}")
    print("-" * 78)

    file_speedups = []
    for desc, rel_path, expected in FILE_TESTS:
        full_path = str(BASE_DIR / rel_path)
        if not os.path.exists(full_path):
            continue

        py_times = timeit.repeat(
            lambda: is_binary_py(full_path),
            number=TIMEIT_NUMBER,
            repeat=ITERATIONS,
        )
        rs_times = timeit.repeat(
            lambda: binaryornot_rs.is_binary(full_path),
            number=TIMEIT_NUMBER,
            repeat=ITERATIONS,
        )

        py_avg_ms = statistics.mean(py_times) / TIMEIT_NUMBER * 1000
        rs_avg_ms = statistics.mean(rs_times) / TIMEIT_NUMBER * 1000
        speedup = py_avg_ms / rs_avg_ms if rs_avg_ms > 0 else float("inf")
        file_speedups.append(speedup)

        print(f"  {desc:<45} {py_avg_ms:>11.3f} {rs_avg_ms:>11.3f} {speedup:>9.1f}x")

    print("-" * 78)

    # ------------------------------------------------------------------
    # String-based: is_binary_string(chunk)
    # ------------------------------------------------------------------
    print()
    print("=" * 78)
    print("  STRING-BASED: is_binary_string(chunk)")
    print("=" * 78)
    print(f"  {'Test':<45} {'Python (ms)':>12} {'Rust (ms)':>12} {'Speedup':>10}")
    print("-" * 78)

    string_speedups = []
    for desc, chunk, expected in STRING_TESTS:
        py_times = timeit.repeat(
            lambda c=chunk: is_binary_string_py(c),
            number=TIMEIT_NUMBER,
            repeat=ITERATIONS,
        )
        rs_times = timeit.repeat(
            lambda c=chunk: binaryornot_rs.is_binary_string(c),
            number=TIMEIT_NUMBER,
            repeat=ITERATIONS,
        )

        py_avg_ms = statistics.mean(py_times) / TIMEIT_NUMBER * 1000
        rs_avg_ms = statistics.mean(rs_times) / TIMEIT_NUMBER * 1000
        speedup = py_avg_ms / rs_avg_ms if rs_avg_ms > 0 else float("inf")
        string_speedups.append(speedup)

        print(f"  {desc:<45} {py_avg_ms:>11.3f} {rs_avg_ms:>11.3f} {speedup:>9.1f}x")

    print("-" * 78)

    # ------------------------------------------------------------------
    # Summary
    # ------------------------------------------------------------------
    print()
    print("=" * 78)
    print("  SUMMARY")
    print("=" * 78)

    all_speedups = file_speedups + string_speedups
    print(f"  File-based tests:   {len(file_speedups):>3} tests, "
          f"median speedup: {statistics.median(file_speedups):>7.1f}x")
    print(f"  String-based tests: {len(string_speedups):>3} tests, "
          f"median speedup: {statistics.median(string_speedups):>7.1f}x")
    print(f"  Overall:            {len(all_speedups):>3} tests, "
          f"median speedup: {statistics.median(all_speedups):>7.1f}x")
    print(f"                      mean speedup:   {statistics.mean(all_speedups):>7.1f}x")
    print("=" * 78)


if __name__ == "__main__":
    run_benchmarks()
