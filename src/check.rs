/// Core binary/text detection logic.
///
/// Provides `is_binary()` (file path) and `is_binary_string()` (raw bytes).

use std::fs;
use std::io::Read;
use std::path::Path;

use crate::extensions::has_binary_extension;
use crate::features::compute_features;
use crate::signatures::has_known_binary_signature;
use crate::tree::is_binary as classify_by_tree;

/// Number of bytes to read from the start of a file for analysis.
pub const CHUNK_SIZE: usize = 512;

/// Read the first `CHUNK_SIZE` bytes from a file.
pub fn get_starting_chunk<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let file = fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut buf = Vec::with_capacity(CHUNK_SIZE);
    reader.take(CHUNK_SIZE as u64).read_to_end(&mut buf)?;
    Ok(buf)
}

/// Check if a byte chunk appears to be binary or text.
///
/// Uses a trained decision tree on byte statistics including entropy,
/// character class ratios, encoding validity checks, and BOM detection.
///
/// Returns `true` if the chunk appears to be binary, `false` if text.
pub fn is_binary_string(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }

    // Fast path: known binary signatures bypass the decision tree
    if has_known_binary_signature(bytes) {
        return true;
    }

    let features = compute_features(bytes);
    classify_by_tree(&features)
}

/// Check if a file is binary or text.
///
/// By default, checks the file extension against a list of known binary
/// types before reading the file. Set `check_extensions` to `false` to
/// classify purely by file contents.
///
/// # Arguments
/// * `path` - Path to the file to check.
/// * `check_extensions` - If `true` (default), check the extension first.
///
/// # Returns
/// `true` if the file is binary, `false` if text.
///
/// # Errors
/// Returns `std::io::Error` if the file cannot be read.
pub fn is_binary<P: AsRef<Path>>(path: P, check_extensions: bool) -> std::io::Result<bool> {
    let path = path.as_ref();

    // Extension check (fast path — no file I/O)
    if check_extensions {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if has_binary_extension(ext) {
                return Ok(true);
            }
        }
    }

    // Content-based detection
    let chunk = get_starting_chunk(path)?;
    Ok(is_binary_string(&chunk))
}

/// Check if a file is binary or text (convenience wrapper, panics on I/O error).
pub fn is_binary_unwrap<P: AsRef<Path>>(path: P, check_extensions: bool) -> bool {
    is_binary(path, check_extensions).expect("Failed to read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bytes() {
        assert!(!is_binary_string(&[]));
    }

    #[test]
    fn test_plain_text() {
        let text = b"Hello, world! This is a plain text file with enough content.\n";
        assert!(!is_binary_string(text));
    }

    #[test]
    fn test_png_signature() {
        let chunk = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR";
        assert!(is_binary_string(chunk));
    }

    #[test]
    fn test_gif_signature() {
        let chunk = b"GIF89a\x01\x00\x01\x00\x80\x00\x00";
        assert!(is_binary_string(chunk));
    }

    #[test]
    fn test_utf16_text() {
        // UTF-16 LE encoded text (no BOM)
        let chunk: Vec<u8> = "Hello, world! This is a test of UTF-16 LE detection. ".encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect();
        assert!(!is_binary_string(&chunk));
    }

    #[test]
    fn test_high_entropy_binary() {
        let chunk: Vec<u8> = (0..512).map(|i| (i * 7 + 3) as u8).collect();
        assert!(is_binary_string(&chunk));
    }

    #[test]
    fn test_control_characters_binary() {
        let chunk: Vec<u8> = (1..=31).cycle().take(512).collect();
        assert!(is_binary_string(&chunk));
    }
}
