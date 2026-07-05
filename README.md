# binaryornot-rs

Rust port of [binaryornot](https://github.com/binaryornot/binaryornot) — an ultra-lightweight library and CLI to check if a file is binary or text.

## Install

```sh
cargo install binaryornot
```

Or build from source:

```sh
git clone https://github.com/binaryornot/binaryornot-rs
cd binaryornot-rs
cargo build --release
```

### Python bindings

Build a wheel using Maturin:

```sh
pip install maturin
maturin build --release --features pyo3-ext
pip install target/wheels/binaryornot_rs-*.whl
```

```python
import binaryornot_rs

binaryornot_rs.is_binary("image.png")       # True
binaryornot_rs.is_binary("README.md")       # False
binaryornot_rs.is_binary_string(b"\x89PNG") # True
```

## Usage

### CLI

```sh
$ binaryornot image.png
true

$ binaryornot README.md
false

$ binaryornot image.png README.md Cargo.toml
true
false
false
```

### Library

```rust
use binaryornot::check::{is_binary, is_binary_string};

// Check a file by path (with extension check)
let result = is_binary("image.png", true).unwrap();
// result == true

// Check without extension check (content only)
let result = is_binary("image.png", false).unwrap();

// Check raw bytes directly
let is_binary = is_binary_string(&chunk);
```

## How it works

BinaryOrNot reads the first 512 bytes of a file and runs them through a trained decision tree that considers:

- **Byte class ratios**: null bytes, control characters, printable ASCII, high bytes
- **Shannon entropy**: overall byte distribution randomness
- **Encoding validity**: UTF-8, UTF-16 LE/BE, UTF-32 LE/BE, GB2312, Big5, Shift-JIS, EUC-JP, EUC-KR
- **BOM detection**: UTF-8, UTF-16, UTF-32 byte order marks
- **Magic signatures**: 50+ known binary file format headers (PNG, JPEG, GIF, PDF, etc.)
- **Printable run length**: longest consecutive run of printable characters

This handles edge cases that simple "check for null bytes" approaches miss:

- UTF-16 text files are full of null bytes but are text
- Big5/GB2312 text has high-ASCII bytes everywhere
- Font files (.woff, .eot) may not have null bytes in the first chunk

## Why not just check for null bytes?

That's the first thing everyone tries. It works until it doesn't:

- A UTF-16 text file is full of null bytes → false binary
- A Big5 or GB2312 text file has high-ASCII bytes everywhere → false binary
- A font file (.woff, .eot) is clearly binary but might not have null bytes

## Dependencies

- **encoding_rs** — for multi-byte encoding detection (GB2312, Big5, Shift-JIS, etc.)

No ML frameworks, no serialization, no runtime model loading. The decision tree is compiled into the binary.

## Project structure

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

Additional directories:

- **`tests/`** — Top-level test files and test fixtures (`files/`, `isBinaryFile/`)
- **`docs/`** — MkDocs documentation (api, installation, usage)
- **`scripts/`** — Development scripts (`generate_fixtures.py`, `train_detector.py`)
- **`CHANGELOG/`** — Per-version changelog entries (0.1.0 through 0.6.0)

## Documentation

- **`README.md`** — Overview, install, usage, benchmark results
- **`architecture.md`** — Module-by-module architecture, data flow, design decisions
- **`quickref.md`** — API reference, CLI, build, test, benchmark, full signature/extension tables

## Testing

The Rust test suite mirrors the original Python `binaryornot` tests:

```sh
cargo test --features pyo3-ext
```

Python tests (requires installed wheel):

```sh
pip install maturin pytest
maturin build --release --features pyo3-ext
pip install target/wheels/binaryornot_rs-*.whl
pytest tests/test_binaryornot_rs.py
pytest tests/test_encoding_coverage.py
pytest tests/test_encoding_warning.py
pytest tests/test_sdist.py
```

The test suite includes 63 core tests mirroring the original Python test suite, plus encoding coverage tests driven from the CSV data files in `src/binaryornot/data/` (37 text encodings, 49 binary formats), wheel structure tests, and encoding warning tests.

## Benchmark

The PyO3 bindings achieve approximately **7.5x median speedup** over the original pure Python implementation:

| API | Tests | Median Speedup | Mean Speedup |
|-----|-------|----------------|--------------|
| File-based (`is_binary`) | 35 | **5.6x** | 9.8x |
| String-based (`is_binary_string`) | 10 | **22.5x** | 24.1x |
| **Overall** | **45** | **7.5x** | **14.2x** |

See `src/binaryornot-rs/tests/benchmark.py` for methodology (10 iterations, 100 calls/iteration).

## License

MIT
