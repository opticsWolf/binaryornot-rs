# Architecture

## Overview

`binaryornot-rs` is a Rust library and CLI that determines whether a file is binary or text by analyzing the first 512 bytes through a compiled decision tree. The architecture follows a layered design: public API → feature extraction → classification.

## Build Configuration

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust package: name `binaryornot`, lib crate `binaryornot_rs`, `rust-version = "1.70"`, `pyo3-ext` optional feature, `tempfile` dev-dependency |
| `pyproject.toml` | Maturin build config: `module-name = "binaryornot_rs"`, `features = ["pyo3-ext"]`, Python 3.8+ classifiers |

## Module Structure

```
src/
   └── binaryornot-rs/        # Rust source (new)
       ├── Cargo.toml
       ├── pyproject.toml
       ├── src/
       │   ├── check.rs
       │   ├── extensions.rs
       │   ├── features.rs
       │   ├── lib.rs
       │   ├── main.rs
       │   ├── pyo3.rs
       │   ├── signatures.rs
       │   └── tree.rs
       └── tests/
           ├── benchmark.py
           ├── test_binaryornot_rs.py
           ├── test_check.rs
           ├── test_encoding_coverage.py
           ├── test_encoding_warning.py
           └── test_sdist.py
```

### `check.rs` — Public API

The entry point for all user-facing functionality:

- **`CHUNK_SIZE`** — Constant of 512 bytes, the number of bytes read from the start of a file for analysis.
- **`is_binary(path, check_extensions)`** — Check a file by path. Optionally checks the extension first (fast path, no I/O), then falls back to content analysis.
- **`is_binary_string(bytes)`** — Check raw bytes directly. Fast path: known binary signatures bypass the decision tree entirely.
- **`get_starting_chunk(path)`** — Read the first `CHUNK_SIZE` bytes from a file using `std::io::BufReader`.
- **`is_binary_unwrap(path, check_extensions)`** — Convenience wrapper that panics on I/O error with message "Failed to read file".

### `extensions.rs` — Extension Lookup

Maintains a static list of 144 binary file extensions organized into 16 categories (3D models, Archives, Audio, CAD, Compiled, Databases, Disk images, Documents, Executables, Firmware, Fonts, Game files, GIS, Images, Scientific data, Video). The `binary_extensions()` function returns a `OnceLock`-initialized `HashSet` of all extensions (public for introspection). The `has_binary_extension()` function normalizes the extension (lowercase, strip dot) and checks membership. This enables zero-I/O classification for files with known binary extensions.

### `features.rs` — Feature Extraction

Converts a byte chunk into a 24-element feature vector (`[f64; 24]`):

| Index | Feature | Description |
|-------|---------|-------------|
| 0 | `null_ratio` | Fraction of 0x00 bytes |
| 1 | `control_ratio` | Fraction of control characters (0x01-0x08, 0x0E-0x1F) |
| 2 | `printable_ascii_ratio` | Fraction of 0x20-0x7E bytes |
| 3 | `high_byte_ratio` | Fraction of 0x80-0xFF bytes |
| 4 | `utf8_valid` | 1.0 if chunk decodes as valid UTF-8, 0.0 otherwise |
| 5 | `even_null_ratio` | Fraction of even-index bytes that are 0x00 |
| 6 | `odd_null_ratio` | Fraction of odd-index bytes that are 0x00 |
| 7 | `entropy` | Shannon entropy of byte distribution |
| 8-12 | `bom_*` | BOM flags: UTF-32 LE/BE, UTF-16 LE/BE, UTF-8 |
| 13-14 | `try_utf16*` | 1.0 if chunk decodes as UTF-16-LE/BE |
| 15-16 | `try_utf32*` | 1.0 if chunk decodes as UTF-32-LE/BE |
| 17 | `longest_printable_run` | Longest run of printable chars / length |
| 18-22 | `try_*` | CJK encoding checks: GB2312, Big5, Shift-JIS, EUC-JP, EUC-KR |
| 23 | `has_magic` | 1.0 if chunk starts with a known binary signature |

**Helper functions:**
- `is_control(b)` — const fn: true if byte is a control character (excludes tab, newline, CR)
- `is_printable_or_whitespace(b)` — true if byte is printable ASCII or common whitespace
- `compute_entropy(hist, total)` — Shannon entropy calculation from byte histogram
- `decode_utf32le(bytes)` — Manual UTF-32-LE decoder (encoding_rs doesn't include UTF-32)
- `decode_utf32be(bytes)` — Manual UTF-32-BE decoder

**Feature gating:** Encoding checks (UTF-16, UTF-32, CJK) only run when the chunk length meets minimum requirements (10 bytes for UTF-16/CJK, 16 bytes for UTF-32), matching the Python version's behavior.

### `signatures.rs` — Magic Signatures

Contains the `SIGNATURES` constant: 52 hardcoded magic byte sequences for known binary formats (PNG, JPEG, GIF, PDF, ELF, Mach-O, ZIP, etc.). The `has_known_binary_signature()` function checks if the chunk starts with any of these sequences via linear scan. This provides an O(n) fast path (n = 52 signatures) that bypasses the decision tree entirely.

### `tree.rs` — Decision Tree

A compiled decision tree (port of the Python `tree.py`) that takes the 24-element feature vector and returns `true` for binary, `false` for text. The tree is trained on labeled data and encodes the classification logic as nested conditional branches. No external model files or ML dependencies are required.

### `pyo3.rs` — Python Bindings (optional)

Exposes the Rust API via PyO3 when the `pyo3-ext` feature is enabled:

- **`is_binary(filename, check_extensions=True)`** — Python-compatible signature. Maps Rust `io::Error` to Python `OSError`.
- **`is_binary_string(chunk: bytes)`** — Accepts Python bytes directly.
- **`__version__`** — Module-level version string from `CARGO_PKG_VERSION`.

### `main.rs` — CLI Entry Point

Reads command-line arguments (skipping `argv[0]`), iterates over each file path, and calls `is_binary(path, true)`. Prints `true`/`false` per line to stdout. Prints errors to stderr and exits with code 1 if any file fails to read.

### `lib.rs` — Module Exports

Re-exports the public modules (`check`, `extensions`, `features`, `signatures`, `tree`) and conditionally includes `pyo3` when the `pyo3-ext` feature is enabled. Contains doctests for the quick-start example.

## Data Flow

```
is_binary(path, check_extensions)
  │
  ├─ check_extensions && has_binary_extension(path.ext) → true (fast path)
  │
  └─ get_starting_chunk(path) → bytes
     │
     └─ is_binary_string(bytes)
        │
        ├─ bytes.is_empty() → false
        ├─ has_known_binary_signature(bytes) → true (fast path)
        │
        └─ compute_features(bytes) → [f64; 24]
           │
           └─ tree.is_binary(features) → bool
```

## Performance Characteristics

- **Extension check**: O(1) hash set lookup, no I/O
- **Magic signature check**: O(n) where n = 52 signatures, no I/O
- **Feature extraction**: O(m) where m = chunk length (512 bytes), single pass
- **Classification**: O(1) — compiled decision tree, no allocations

## Testing Strategy

The test suite mirrors the original Python `binaryornot` tests:

| Test file | Tests | Description |
|-----------|-------|-------------|
| `src/binaryornot-rs/tests/test_check.rs` | 36 | Rust integration tests (magic signatures, file I/O, extensions) |
| Module unit tests | 23 | Per-module unit tests (check, extensions, features, signatures, tree) |
| Doc tests | 1 | README example in `lib.rs` |
| `src/binaryornot-rs/tests/test_binaryornot_rs.py` | 63 | Python tests mirroring original `test_check.py` |
| `src/binaryornot-rs/tests/test_encoding_coverage.py` | 187 | CSV-driven coverage (37 text encodings, 49 binary formats) |
| `src/binaryornot-rs/tests/test_encoding_warning.py` | 3 | `EncodingWarning` immunity tests |
| `src/binaryornot-rs/tests/test_sdist.py` | 6 | Wheel structure tests |

**Total: 259 Python tests (255 passed, 4 xfailed), 60 Rust tests (all pass)**

The 4 xfailed tests are known encoding gaps: `iso-2022-kr`, `cp037`, `cp500`, `ebcdic-cp-us` (same as the original Python version).

## Design Decisions

1. **Static extension list** — No CSV parsing at runtime. Extensions are hardcoded for zero-dependency, zero-allocation lookups.
2. **Static magic signatures** — Hardcoded byte sequences avoid file I/O and external data dependencies.
3. **Feature gating** — Encoding checks (UTF-16, UTF-32, CJK) only run when the chunk length meets minimum requirements, matching the Python version's behavior.
4. **PyO3 feature gate** — The Python bindings are optional (`pyo3-ext` feature) so the Rust library can be used standalone without PyO3.
5. **24-element feature vector** — Fixed-size array avoids heap allocation in the hot path.
6. **Manual UTF-32 decoder** — `encoding_rs` doesn't include UTF-32, so `decode_utf32le`/`decode_utf32be` are implemented manually with surrogate pair validation.
7. **CLI uses `process::exit`** — The CLI exits with code 1 on any I/O error, matching the original Python `binaryornot` CLI behavior.
