/// Integration tests for binaryornot.
///
/// These tests mirror the Python test suite from binaryornot/tests/test_check.py.

use binaryornot_rs::check::{is_binary, is_binary_string};
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

/// Create a temp file with given content and return its path.
fn temp_file_with(data: &[u8]) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(data).expect("Failed to write temp file");
    file.flush().expect("Failed to flush temp file");
    file
}

// ─── Core detection ────────────────────────────────────────────────────────

#[test]
fn test_empty_file() {
    let file = temp_file_with(&[]);
    assert!(!is_binary(file.path(), true).unwrap());
}

#[test]
fn test_plain_text() {
    let file = temp_file_with(b"The quick brown fox jumps over the lazy dog.\n");
    assert!(!is_binary(file.path(), true).unwrap());
}

#[test]
fn test_unicode_text() {
    let file = temp_file_with("Héllo wörld, café, naïve, résumé.".as_bytes());
    assert!(!is_binary(file.path(), true).unwrap());
}

#[test]
fn test_png_file() {
    // Minimal PNG signature + IHDR chunk
    let data = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x02\x00\x00\x00\x02\x00\x08\x04";
    let file = temp_file_with(data);
    assert!(is_binary(file.path(), true).unwrap());
}

#[test]
fn test_gif_file() {
    let data = b"GIF89a\x01\x00\x01\x00\x80\x00\x00\xff\xff\xff\x00\x00\x00";
    let file = temp_file_with(data);
    assert!(is_binary(file.path(), true).unwrap());
}

#[test]
fn test_jpeg_file() {
    let data = b"\xff\xd8\xff\xe0\x00\x10JFIF\x00\x01\x01\x00\x00\x01\x00\x01\x00\x00";
    let file = temp_file_with(data);
    assert!(is_binary(file.path(), true).unwrap());
}

#[test]
fn test_pdf_file() {
    let data = b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n1 0 obj\n<< /Type /Catalog >>\nendobj";
    let file = temp_file_with(data);
    assert!(is_binary(file.path(), true).unwrap());
}

#[test]
fn test_sqlite_file() {
    let data = b"SQLite format 3\x00\x10\x00\x01\x01";
    let file = temp_file_with(data);
    assert!(is_binary(file.path(), true).unwrap());
}

#[test]
fn test_zip_file() {
    let data = b"PK\x03\x04\x14\x00\x00\x00\x08\x00";
    let file = temp_file_with(data);
    assert!(is_binary(file.path(), true).unwrap());
}

#[test]
fn test_high_entropy_binary() {
    let data: Vec<u8> = (0..512).map(|i| (i * 7 + 3) as u8).collect();
    let file = temp_file_with(&data);
    assert!(is_binary(file.path(), true).unwrap());
}

#[test]
fn test_control_char_binary() {
    let data: Vec<u8> = (1..=31).cycle().take(512).collect();
    let file = temp_file_with(&data);
    assert!(is_binary(file.path(), true).unwrap());
}

// ─── Encoding detection ────────────────────────────────────────────────────

#[test]
fn test_utf16le_text() {
    let chunk: Vec<u8> = "Hello, world! This is a test of UTF-16 LE detection. "
        .encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .collect();
    let file = temp_file_with(&chunk);
    assert!(!is_binary(file.path(), true).unwrap());
}

#[test]
fn test_utf16be_text() {
    let chunk: Vec<u8> = "Hello, world! This is a test of UTF-16 BE detection. "
        .encode_utf16()
        .flat_map(|c| c.to_be_bytes())
        .collect();
    let file = temp_file_with(&chunk);
    assert!(!is_binary(file.path(), true).unwrap());
}

#[test]
fn test_utf16_with_bom() {
    let mut data = b"\xff\xfe".to_vec(); // UTF-16 LE BOM
    "Hello, world! This is a test of UTF-16 with BOM. "
        .encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .for_each(|b| data.push(b));
    let file = temp_file_with(&data);
    assert!(!is_binary(file.path(), true).unwrap());
}

#[test]
fn test_utf8_with_bom() {
    let data = b"\xef\xbb\xbfHello, world! This is a UTF-8 file with BOM.";
    let file = temp_file_with(data);
    assert!(!is_binary(file.path(), true).unwrap());
}

#[test]
fn test_utf32le_with_bom() {
    let mut data = b"\xff\xfe\x00\x00".to_vec(); // UTF-32 LE BOM
    "Hello world test of UTF-32 LE with BOM."
        .encode_utf16()
        .for_each(|c| {
            let cp = c as u32;
            data.extend_from_slice(&cp.to_le_bytes());
        });
    let file = temp_file_with(&data);
    assert!(!is_binary(file.path(), true).unwrap());
}

#[test]
fn test_utf32be_with_bom() {
    let mut data = b"\x00\x00\xfe\xff".to_vec(); // UTF-32 BE BOM
    "Hello world test of UTF-32 BE with BOM."
        .encode_utf16()
        .for_each(|c| {
            let cp = c as u32;
            data.extend_from_slice(&cp.to_be_bytes());
        });
    let file = temp_file_with(&data);
    assert!(!is_binary(file.path(), true).unwrap());
}

// ─── Extension check ───────────────────────────────────────────────────────

#[test]
fn test_extension_png() {
    // Text content with .png extension → binary by extension
    let mut file = NamedTempFile::new().expect("temp");
    file.write_all(b"This is not a PNG file.").unwrap();
    let png_path = PathBuf::from(file.path()).with_extension("png");
    std::fs::rename(file.path(), &png_path).unwrap();
    assert!(is_binary(&png_path, true).unwrap());
    assert!(!is_binary(&png_path, false).unwrap()); // content says text
    std::fs::remove_file(&png_path).unwrap();
}

#[test]
fn test_extension_pyc() {
    let mut file = NamedTempFile::new().expect("temp");
    file.write_all(b"This is not bytecode.").unwrap();
    let pyc_path = PathBuf::from(file.path()).with_extension("pyc");
    std::fs::rename(file.path(), &pyc_path).unwrap();
    assert!(is_binary(&pyc_path, true).unwrap());
    std::fs::remove_file(&pyc_path).unwrap();
}

#[test]
fn test_extension_case_insensitive() {
    let mut file = NamedTempFile::new().expect("temp");
    file.write_all(b"Not a PNG.").unwrap();
    let upper_path = PathBuf::from(file.path()).with_extension("PNG");
    std::fs::rename(file.path(), &upper_path).unwrap();
    assert!(is_binary(&upper_path, true).unwrap());
    std::fs::remove_file(&upper_path).unwrap();
}

#[test]
fn test_text_extensions_not_binary() {
    let file = temp_file_with(b"Hello world.");
    assert!(!is_binary(file.path(), true).unwrap());
}

// ─── Magic signature bypass ────────────────────────────────────────────────

#[test]
fn test_png_signature_bypass() {
    // PNG signature should bypass the decision tree even with adversarial content
    let chunk = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x02\x00\x00\x00\x02\x00\x08\x04\x00\x00\x00^q\x1cq\x00\x00\x00\x04gAMA\x00\x00\xb1\x8f\x0b\xfca\x05";
    assert!(is_binary_string(chunk));
}

#[test]
fn test_plain_text_not_caught_by_magic() {
    let chunk = b"Hello, world! This is a plain text file with enough content.\n";
    assert!(!is_binary_string(chunk));
}

// ─── Feature vector ────────────────────────────────────────────────────────

#[test]
fn test_feature_vector_length() {
    use binaryornot_rs::features::compute_features;
    let mut chunk = b"\x89PNG\r\n\x1a\n".to_vec();
    chunk.extend(vec![0u8; 504]);
    let features = compute_features(&chunk);
    assert_eq!(features.len(), 24);
}

#[test]
fn test_magic_feature_for_png() {
    use binaryornot_rs::features::compute_features;
    let mut chunk = b"\x89PNG\r\n\x1a\n".to_vec();
    chunk.extend(vec![0u8; 505]);
    let features = compute_features(&chunk);
    assert_eq!(features[23], 1.0);
}

#[test]
fn test_magic_feature_unset_for_text() {
    use binaryornot_rs::features::compute_features;
    let chunk = b"Hello, world! This is plain text. "
        .repeat(16);
    let features = compute_features(&chunk[..512]);
    assert_eq!(features[23], 0.0);
}

// ─── Error handling ────────────────────────────────────────────────────────

#[test]
fn test_nonexistent_file() {
    let result = is_binary("/nonexistent/file/path.txt", true);
    assert!(result.is_err());
}

// ─── Signature tests ───────────────────────────────────────────────────────

#[test]
fn test_elf_signature() {
    let data = b"\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    assert!(is_binary_string(data));
}

#[test]
fn test_macho_signature() {
    let data = b"\xcf\xfa\xed\xfe\x07\x00\x00\x01";
    assert!(is_binary_string(data));
}

#[test]
fn test_mz_signature() {
    let data = b"MZ\x90\x00\x03\x00\x00\x00\x04\x00";
    assert!(is_binary_string(data));
}

#[test]
fn test_java_class_signature() {
    let data = b"\xca\xfe\xba\xbe\x00\x00\x00\x34";
    assert!(is_binary_string(data));
}

#[test]
fn test_bmp_signature() {
    let data = b"BM\x36\x74\x00\x00\x00\x00\x00\x00";
    assert!(is_binary_string(data));
}

#[test]
fn test_tiff_signature() {
    let data = b"II*\x00\x08\x00\x00\x00";
    assert!(is_binary_string(data));
}

// ─── Font files ────────────────────────────────────────────────────────────

#[test]
fn test_woff_signature() {
    let data = b"wOFF\x00\x01\x00\x00";
    assert!(is_binary_string(data));
}

#[test]
fn test_otf_signature() {
    let data = b"OTTO\x00\x01\x00\x00";
    assert!(is_binary_string(data));
}

#[test]
fn test_ttf_signature() {
    let data = b"\x00\x01\x00\x00\x00\x08\x00\x80";
    assert!(is_binary_string(data));
}
