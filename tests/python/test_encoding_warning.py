"""Verify that binaryornot_rs does not trigger EncodingWarning.

Python 3.10+ emits EncodingWarning when open() is called without an
explicit encoding argument, if -X warn_default_encoding is active.
The Rust PyO3 module should be immune since it never calls Python open().
"""

import subprocess
import sys
import unittest


class TestNoEncodingWarning(unittest.TestCase):
    """Test that importing binaryornot_rs triggers no EncodingWarning."""

    def test_no_encoding_warning_on_import(self):
        """Importing binaryornot_rs with warn_default_encoding raises no warnings."""
        result = subprocess.run(
            [
                sys.executable,
                "-X",
                "warn_default_encoding",
                "-W",
                "error::EncodingWarning",
                "-c",
                "import binaryornot_rs",
            ],
            capture_output=True,
            text=True,
        )
        self.assertEqual(
            result.returncode, 0,
            f"EncodingWarning triggered on import:\n{result.stderr}",
        )

    def test_no_encoding_warning_on_is_binary(self):
        """Calling is_binary() with warn_default_encoding raises no warnings."""
        result = subprocess.run(
            [
                sys.executable,
                "-X",
                "warn_default_encoding",
                "-W",
                "error::EncodingWarning",
                "-c",
                "import binaryornot_rs; binaryornot_rs.is_binary_string(b'Hello, world!')",
            ],
            capture_output=True,
            text=True,
        )
        self.assertEqual(
            result.returncode, 0,
            f"EncodingWarning triggered on is_binary_string:\n{result.stderr}",
        )

    def test_no_encoding_warning_on_is_binary_string(self):
        """Calling is_binary_string() with warn_default_encoding raises no warnings."""
        result = subprocess.run(
            [
                sys.executable,
                "-X",
                "warn_default_encoding",
                "-W",
                "error::EncodingWarning",
                "-c",
                "import binaryornot_rs; binaryornot_rs.is_binary_string(b'\\x89PNG\\r\\n\\x1a\\n' + b'\\x00' * 100)",
            ],
            capture_output=True,
            text=True,
        )
        self.assertEqual(
            result.returncode, 0,
            f"EncodingWarning triggered on is_binary_string with binary data:\n{result.stderr}",
        )


if __name__ == "__main__":
    unittest.main()
