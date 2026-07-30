# binaryornot-rs

[![CI](https://github.com/opticsWolf/binaryornot-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/opticsWolf/binaryornot-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/binaryornot-rs.svg)](https://crates.io/crates/binaryornot-rs)
[![downloads](https://img.shields.io/crates/d/binaryornot-rs.svg)](https://crates.io/crates/binaryornot-rs)
[![PyPI version](https://img.shields.io/pypi/v/binaryornot-rs.svg)](https://pypi.org/project/binaryornot-rs/)
[![PyPI downloads](https://img.shields.io/pypi/dm/binaryornot-rs.svg)](https://pypi.org/project/binaryornot-rs/)
[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Rust port of [binaryornot](https://github.com/binaryornot/binaryornot) — an ultra-lightweight library and CLI to check if a file is binary or text. ~7.2x faster than the original Python with PyO3 bindings included.

## Install

```sh
cargo install binaryornot
```

Or build from source:

```sh
git clone https://github.com/opticsWolf/binaryornot-rs
cd binaryornot-rs
cargo build --release
```

### Python bindings

Build a wheel using Maturin (PyO3 is enabled by default for maturin builds):

```sh
pip install maturin
maturin build --release
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
├── Cargo.toml             # Rust crate (pyo3 optional feature)
├── pyproject.toml         # Python package config (maturin)
├── src/
│   ├── check.rs
│   ├── extensions.rs
│   ├── features.rs
│   ├── lib.rs
│   ├── main.rs            # CLI
│   ├── pyo3.rs            # Python bindings (gated by `python` feature)
│   ├── signatures.rs
│   └── tree.rs
└── tests/
    ├── benchmark.py
    ├── test_check.rs      # Rust integration tests
    └── python/
        ├── test_binaryornot_rs.py
        ├── test_encoding_warning.py
        └── test_sdist.py
```

Additional directories:

- **`tests/`** — Test files and fixtures (`files/`, `isBinaryFile/`)

## Documentation

- **`README.md`** — Overview, install, usage, benchmark results
- **`architecture.md`** — Module-by-module architecture, data flow, design decisions
- **`quickref.md`** — API reference, CLI, build, test, benchmark, full signature/extension tables

## Testing

Rust tests:

```sh
cargo test
```

Python tests (requires installed wheel):

```sh
pip install maturin pytest pytest-timeout
maturin develop --release
pytest tests/python/ -v --timeout=60 --timeout-method=thread
```

The test suite includes 67 core tests mirroring the original Python test suite, plus wheel structure tests and encoding warning tests.

## Benchmark

The PyO3 bindings achieve approximately **7.2x median speedup** over the original pure Python implementation (averaged across 10 runs):

| API | Tests | Median Speedup | Mean Speedup |
|-----|-------|----------------|--------------|
| File-based (`is_binary`) | 35 | **5.6x** | 10.4x |
| String-based (`is_binary_string`) | 10 | **20.4x** | 25.4x |
| **Overall** | **45** | **7.2x** | **13.7x** |

See `tests/benchmark.py` for methodology (10 iterations, 100 calls/iteration).

## License

MIT
