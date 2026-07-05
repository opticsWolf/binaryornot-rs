/// Compute 24 features from a byte chunk for the binary/text decision tree.
///
/// Feature indices:
///   0: null_ratio           - fraction of 0x00 bytes
///   1: control_ratio        - fraction of control chars (0x01-0x08, 0x0E-0x1F)
///   2: printable_ascii_ratio - fraction of 0x20-0x7E
///   3: high_byte_ratio      - fraction of 0x80-0xFF
///   4: utf8_valid           - 1.0 if chunk decodes as valid UTF-8
///   5: even_null_ratio      - fraction of even-index bytes that are 0x00
///   6: odd_null_ratio       - fraction of odd-index bytes that are 0x00
///   7: byte_entropy         - Shannon entropy of byte distribution
///   8-12: BOM flags         - UTF-32 LE/BE, UTF-16 LE/BE, UTF-8 BOM
///  13: try_utf16le          - 1.0 if chunk decodes as UTF-16-LE
///  14: try_utf16be          - 1.0 if chunk decodes as UTF-16-BE
///  15: try_utf32le          - 1.0 if chunk decodes as UTF-32-LE
///  16: try_utf32be          - 1.0 if chunk decodes as UTF-32-BE
///  17: longest_printable_run - longest run of printable chars / length
///  18: try_gb2312           - 1.0 if chunk decodes as GB2312
///  19: try_big5             - 1.0 if chunk decodes as Big5
///  20: try_shift_jis        - 1.0 if chunk decodes as Shift-JIS
///  21: try_euc_jp           - 1.0 if chunk decodes as EUC-JP
///  22: try_euc_kr           - 1.0 if chunk decodes as EUC-KR
///  23: has_magic_signature  - 1.0 if chunk starts with a known binary signature

use encoding_rs::{GB18030, BIG5, EUC_KR, EUC_JP, SHIFT_JIS, UTF_16LE, UTF_16BE, UTF_8};

use crate::signatures::has_known_binary_signature;

/// Check if a byte is a control character (excluding tab=9, newline=10, carriage return=13).
const fn is_control(b: u8) -> bool {
    b < 32 && b != 9 && b != 10 && b != 13
}

/// Check if a byte is printable ASCII (0x20-0x7E) or common whitespace (tab, newline, CR).
fn is_printable_or_whitespace(b: u8) -> bool {
    (b >= 0x20 && b <= 0x7E) || b == 9 || b == 10 || b == 13
}

/// Compute all 24 features from a byte chunk.
///
/// Returns a `[f64; 24]` feature vector suitable for the decision tree.
pub fn compute_features(chunk: &[u8]) -> [f64; 24] {
    let n = chunk.len() as f64;

    // Byte class counts
    let mut null_count: usize = 0;
    let mut control_count: usize = 0;
    let mut printable_count: usize = 0;
    let mut high_count: usize = 0;

    // Byte histogram for entropy
    let mut hist = [0u32; 256];

    // Even/odd null counts
    let mut even_nulls: usize = 0;
    let mut odd_nulls: usize = 0;
    let even_total = (chunk.len() + 1) / 2;
    let odd_total = chunk.len() / 2;

    // Longest printable run
    let mut max_run: usize = 0;
    let mut current_run: usize = 0;

    for (i, &b) in chunk.iter().enumerate() {
        hist[b as usize] += 1;

        if b == 0 {
            null_count += 1;
            if i % 2 == 0 {
                even_nulls += 1;
            } else {
                odd_nulls += 1;
            }
        }

        if is_control(b) {
            control_count += 1;
        }

        if (0x20..=0x7E).contains(&b) {
            printable_count += 1;
        }

        if b >= 0x80 {
            high_count += 1;
        }

        // Track longest printable run
        if is_printable_or_whitespace(b) {
            current_run += 1;
            if current_run > max_run {
                max_run = current_run;
            }
        } else {
            current_run = 0;
        }
    }

    let null_ratio = null_count as f64 / n;
    let control_ratio = control_count as f64 / n;
    let printable_ascii_ratio = printable_count as f64 / n;
    let high_byte_ratio = high_count as f64 / n;

    // UTF-8 validity (encoding_rs returns (Cow<str>, &Encoding, bool_failed))
    let utf8_valid = if UTF_8.decode(chunk).2 { 0.0 } else { 1.0 };

    // Even/odd null ratios
    let even_null_ratio = if even_total > 0 { even_nulls as f64 / even_total as f64 } else { 0.0 };
    let odd_null_ratio = if odd_total > 0 { odd_nulls as f64 / odd_total as f64 } else { 0.0 };

    // Shannon entropy
    let entropy = compute_entropy(&hist, chunk.len());

    // BOM detection
    let bom_utf32le = if chunk.starts_with(b"\xff\xfe\x00\x00") { 1.0 } else { 0.0 };
    let bom_utf32be = if chunk.starts_with(b"\x00\x00\xfe\xff") { 1.0 } else { 0.0 };
    let bom_utf16le = if chunk.starts_with(b"\xff\xfe") && !chunk.starts_with(b"\xff\xfe\x00\x00") { 1.0 } else { 0.0 };
    let bom_utf16be = if chunk.starts_with(b"\xfe\xff") { 1.0 } else { 0.0 };
    let bom_utf8 = if chunk.starts_with(b"\xef\xbb\xbf") { 1.0 } else { 0.0 };

    // Encoding validity checks (minimum length requirements match Python version)
    let chunk_len = chunk.len();
    let (try_utf16le, try_utf16be, try_utf32le, try_utf32be) = if chunk_len >= 10 {
        let try_utf16le = if chunk_len % 2 == 0 && !UTF_16LE.decode(chunk).2 { 1.0 } else { 0.0 };
        let try_utf16be = if chunk_len % 2 == 0 && !UTF_16BE.decode(chunk).2 { 1.0 } else { 0.0 };
        (try_utf16le, try_utf16be, 0.0, 0.0)
    } else {
        (0.0, 0.0, 0.0, 0.0)
    };
    let (try_utf32le, try_utf32be) = if chunk_len >= 16 {
        let try_utf32le = if chunk_len % 4 == 0 && decode_utf32le(chunk).is_ok() { 1.0 } else { 0.0 };
        let try_utf32be = if chunk_len % 4 == 0 && decode_utf32be(chunk).is_ok() { 1.0 } else { 0.0 };
        (try_utf32le, try_utf32be)
    } else {
        (try_utf32le, try_utf32be)
    };

    let longest_printable_run = max_run as f64 / n;

    // CJK encoding checks (minimum length 10, matching Python)
    // encoding_rs returns (Cow<str>, &Encoding, bool_failed) — index 2 is "had errors"
    let try_gb2312 = if chunk_len >= 10 && !GB18030.decode(chunk).2 { 1.0 } else { 0.0 };
    let try_big5 = if chunk_len >= 10 && !BIG5.decode(chunk).2 { 1.0 } else { 0.0 };
    let try_shift_jis = if chunk_len >= 10 && !SHIFT_JIS.decode(chunk).2 { 1.0 } else { 0.0 };
    let try_euc_jp = if chunk_len >= 10 && !EUC_JP.decode(chunk).2 { 1.0 } else { 0.0 };
    let try_euc_kr = if chunk_len >= 10 && !EUC_KR.decode(chunk).2 { 1.0 } else { 0.0 };

    let has_magic = if has_known_binary_signature(chunk) { 1.0 } else { 0.0 };

    [
        null_ratio,           // 0
        control_ratio,        // 1
        printable_ascii_ratio,// 2
        high_byte_ratio,      // 3
        utf8_valid,           // 4
        even_null_ratio,      // 5
        odd_null_ratio,       // 6
        entropy,              // 7
        bom_utf32le,          // 8
        bom_utf32be,          // 9
        bom_utf16le,          // 10
        bom_utf16be,          // 11
        bom_utf8,             // 12
        try_utf16le,          // 13
        try_utf16be,          // 14
        try_utf32le,          // 15
        try_utf32be,          // 16
        longest_printable_run,// 17
        try_gb2312,           // 18
        try_big5,             // 19
        try_shift_jis,        // 20
        try_euc_jp,           // 21
        try_euc_kr,           // 22
        has_magic,            // 23
    ]
}

/// Compute Shannon entropy of a byte distribution.
fn compute_entropy(hist: &[u32; 256], total: usize) -> f64 {
    let n = total as f64;
    let mut entropy = 0.0f64;
    for &count in hist.iter() {
        if count > 0 {
            let p = count as f64 / n;
            entropy -= p * p.log2();
        }
    }
    entropy
}

/// Decode UTF-32-LE bytes. `encoding_rs` doesn't include UTF-32, so we do it manually.
fn decode_utf32le(bytes: &[u8]) -> Result<(), ()> {
    if bytes.len() % 4 != 0 {
        return Err(());
    }
    for chunk in bytes.chunks_exact(4) {
        let cp = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        if cp > 0x10FFFF || (cp >= 0xD800 && cp <= 0xDFFF) {
            return Err(());
        }
    }
    Ok(())
}

/// Decode UTF-32-BE bytes.
fn decode_utf32be(bytes: &[u8]) -> Result<(), ()> {
    if bytes.len() % 4 != 0 {
        return Err(());
    }
    for chunk in bytes.chunks_exact(4) {
        let cp = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        if cp > 0x10FFFF || (cp >= 0xD800 && cp <= 0xDFFF) {
            return Err(());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_vector_length() {
        let mut chunk = b"\x89PNG\r\n\x1a\n".to_vec();
        chunk.extend(vec![0u8; 504]);
        let features = compute_features(&chunk);
        assert_eq!(features.len(), 24);
    }

    #[test]
    fn test_plain_text_features() {
        let chunk = b"Hello, world! This is plain text content for testing.";
        let features = compute_features(chunk);
        assert_eq!(features[0], 0.0); // no nulls
        assert!((features[2] - 1.0).abs() < 0.01); // almost all printable
        assert_eq!(features[4], 1.0); // valid UTF-8
        assert_eq!(features[3], 0.0); // no high bytes
    }

    #[test]
    fn test_binary_features() {
        let chunk: Vec<u8> = (0..512).map(|i| (i % 256) as u8).collect();
        let features = compute_features(&chunk);
        assert!(features[0] > 0.0); // has nulls
        assert!(features[7] > 7.0); // high entropy
        assert_eq!(features[4], 0.0); // not valid UTF-8
    }

    #[test]
    fn test_png_magic_feature() {
        let mut chunk = b"\x89PNG\r\n\x1a\n".to_vec();
        chunk.extend(vec![0u8; 505]);
        let features = compute_features(&chunk);
        assert_eq!(features[23], 1.0); // has magic signature
    }

    #[test]
    fn test_entropy_calculation() {
        // Uniform distribution of 4 values: entropy = log2(4) = 2.0
        let chunk = b"\x00\x01\x02\x03\x00\x01\x02\x03";
        let features = compute_features(chunk);
        assert!((features[7] - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_utf16le_detection() {
        let chunk = "Hello, world!".encode_utf16().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>();
        let features = compute_features(&chunk);
        assert_eq!(features[13], 1.0); // try_utf16le
    }
}
