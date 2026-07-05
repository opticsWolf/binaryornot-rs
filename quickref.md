# Quick Reference

## API

### `is_binary(filename, check_extensions=True)`

Check if a file is binary or text.

```rust
use binaryornot_rs::check::is_binary;

is_binary("image.png", true).unwrap();   // true
is_binary("README.md", true).unwrap();   // false
is_binary("image.png", false).unwrap();  // content-only check
```

**Parameters:**
- `filename`: Path to the file (string or `Path`)
- `check_extensions`: If `true`, check extension first (fast path). Default `true`.

**Returns:** `Result<bool, io::Error>` — `true` if binary, `false` if text.

---

### `is_binary_string(chunk)`

Check raw bytes directly.

```rust
use binaryornot_rs::check::is_binary_string;

is_binary_string(b"\x89PNG\r\n\x1a\n");  // true (PNG magic)
is_binary_string(b"Hello, world!");       // false (plain text)
is_binary_string(b"");                    // false (empty)
```

**Parameters:**
- `chunk`: Byte slice to analyze

**Returns:** `bool` — `true` if binary, `false` if text.

---

### `is_binary_unwrap(filename, check_extensions)`

Convenience wrapper that panics on I/O error.

```rust
use binaryornot_rs::check::is_binary_unwrap;

is_binary_unwrap("image.png", true);  // true
```

---

### `get_starting_chunk(path)`

Read the first `CHUNK_SIZE` (512) bytes from a file.

```rust
use binaryornot_rs::check::{get_starting_chunk, CHUNK_SIZE};

let chunk = get_starting_chunk("file.txt").unwrap();
assert_eq!(chunk.len(), CHUNK_SIZE);  // 512
```

---

### `binary_extensions()`

Return the full set of known binary extensions (for introspection).

```rust
use binaryornot_rs::extensions::binary_extensions;

let exts = binary_extensions();
assert!(exts.len() == 144);
```

---

### `has_binary_extension(ext)`

Check if a single extension is a known binary type.

```rust
use binaryornot_rs::extensions::has_binary_extension;

has_binary_extension("png");   // true
has_binary_extension("TXT");   // false (case-insensitive)
```

---

## Python API

```python
import binaryornot_rs

# Check a file
binaryornot_rs.is_binary("image.png")            # True
binaryornot_rs.is_binary("README.md")             # False
binaryornot_rs.is_binary("image.png", False)      # content-only

# Check raw bytes
binaryornot_rs.is_binary_string(b"\x89PNG")      # True
binaryornot_rs.is_binary_string(b"Hello")         # False

# Version
binaryornot_rs.__version__                        # "0.6.0"
```

---

## CLI

```sh
# Single file
binaryornot file.txt          # false
binaryornot image.png         # true

# Multiple files
binaryornot a.txt b.png c.pdf # false / true / true

# Usage
binaryornot                   # error: no arguments
```

Exit code 1 on any I/O error.

---

## Build

### Rust library

```sh
cd src/binaryornot-rs
cargo build --release          # requires Rust 1.70+
cargo test --features pyo3-ext # 60 tests (23 unit, 36 integration, 1 doc)
```

### Python wheel

```sh
pip install maturin
cd src/binaryornot-rs
maturin build --release --features pyo3-ext
pip install target/wheels/binaryornot_rs-*.whl
```

`pyproject.toml` configures Maturin:
```toml
[tool.maturin]
features = ["pyo3-ext"]
module-name = "binaryornot_rs"
```

---

## Test

```sh
# Rust tests (60 total)
cd src/binaryornot-rs
cargo test --features pyo3-ext

# Python tests (259 total: 255 passed, 4 xfailed)
cd src/binaryornot-rs
pytest tests/test_binaryornot_rs.py          # 63 tests
pytest tests/test_encoding_coverage.py       # 187 tests
pytest tests/test_encoding_warning.py        # 3 tests
pytest tests/test_sdist.py                   # 6 tests
```

---

## Benchmark

| API | Tests | Median Speedup | Mean Speedup |
|-----|-------|----------------|--------------|
| File-based (`is_binary`) | 35 | **5.6x** | 9.8x |
| String-based (`is_binary_string`) | 10 | **22.5x** | 24.1x |
| **Overall** | **45** | **7.5x** | **14.2x** |

See `src/binaryornot-rs/tests/benchmark.py` for methodology (10 iterations, 100 calls/iteration).

---

## Feature Vector (Internal)

The 24-element feature vector computed from each byte chunk:

| Index | Feature | Range |
|-------|---------|-------|
| 0 | `null_ratio` | [0.0, 1.0] |
| 1 | `control_ratio` | [0.0, 1.0] |
| 2 | `printable_ascii_ratio` | [0.0, 1.0] |
| 3 | `high_byte_ratio` | [0.0, 1.0] |
| 4 | `utf8_valid` | {0.0, 1.0} |
| 5 | `even_null_ratio` | [0.0, 1.0] |
| 6 | `odd_null_ratio` | [0.0, 1.0] |
| 7 | `entropy` | [0.0, 8.0] |
| 8-12 | `bom_*` | {0.0, 1.0} |
| 13-14 | `try_utf16*` | {0.0, 1.0} |
| 15-16 | `try_utf32*` | {0.0, 1.0} |
| 17 | `longest_printable_run` | [0.0, 1.0] |
| 18-22 | `try_*` (CJK) | {0.0, 1.0} |
| 23 | `has_magic` | {0.0, 1.0} |

---

## Magic Signatures (52)

| Category | Formats |
|----------|---------|
| Images | PNG, JPEG (JFIF/Exif), GIF87a/89a, BMP, TIFF (BE/LE), ICO, WebP, PSD, HEIF |
| Documents | PDF, SQLite, Parquet, .DS_Store |
| Archives | ZIP, GZIP, XZ, bzip2, 7-Zip, RAR5, Zstandard, LZ4, LZMA, Unix ar, cpio |
| Executables | ELF, Mach-O (32/64 BE/LE), MZ (PE/COFF), Java class, Dalvik, LLVM bitcode |
| Containers | RIFF (WAV/AVI), Ogg, FLAC, WebAssembly, Matroska, Git pack, Apache Arrow, Avro |
| Fonts | WOFF, WOFF2, OTF, TrueType |
| Media | MP4, MP3 (ID3v2), MIDI |
| Other | Java KeyStore, libpcap, Snappy |

See `src/binaryornot-rs/src/signatures.rs` for the full `SIGNATURES` constant.

---

## Binary Extensions (144)

| Category | Count | Extensions |
|----------|-------|------------|
| 3D models | 7 | 3ds, blend, dae, fbx, glb, stl, usdz |
| Archives | 14 | 7z, bz2, cab, gz, jar, lz, lz4, lzma, rar, tar, xz, z, zip, zst |
| Audio | 14 | aac, aif, aiff, alac, ape, flac, m4a, mid, midi, mp3, ogg, opus, wav, wma |
| CAD | 5 | dwg, dwf, dxb, step, stp |
| Compiled | 7 | class, dll, o, pyc, pyo, so, wasm |
| Databases | 6 | accdb, db, dbf, mdb, sqlite, sqlite3 |
| Disk images | 7 | dmg, img, iso, qcow2, vdi, vhd, vmdk |
| Documents | 11 | doc, docx, epub, mobi, odt, pages, pdf, ppt, pptx, xls, xlsx |
| Executables | 3 | com, exe, msi |
| Firmware | 3 | bin, fw, rom |
| Fonts | 6 | eot, otf, ttc, ttf, woff, woff2 |
| Game files | 8 | gba, n64, nds, nes, pak, sav, sfc, smc |
| GIS | 2 | shp, shx |
| Images | 23 | avif, bmp, cr2, cr3, dng, gif, heic, heif, ico, jfif, jp2, jpeg, jpg, jxl, nef, orf, png, psd, raw, tif, tiff, webp, xcf |
| Scientific data | 12 | arrow, avro, fit, fits, hdf5, mat, npy, npz, parquet, pcap, pcapng |
| Video | 11 | avi, flv, m4v, mkv, mov, mp4, mpeg, mpg, ogv, webm, wmv |

See `src/binaryornot-rs/src/extensions.rs` for the full `EXTENSIONS` constant.
