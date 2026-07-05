//! # BinaryOrNot
//!
//! Ultra-lightweight Rust library and CLI to check if a file is binary or text.
//! Zero dependencies on ML frameworks — uses a compiled-in decision tree.
//!
//! ## Quick start
//!
//! ```rust
//! use binaryornot_rs::check::{is_binary, is_binary_string};
//!
//! // Check a file by path
//! let result = is_binary("README.md", true).unwrap();
//! assert!(!result); // README.md is text
//!
//! // Check raw bytes
//! let is_binary = is_binary_string(b"\x89PNG\r\n\x1a\n");
//! assert!(is_binary); // PNG magic bytes → binary
//! ```
//!
//! ## CLI
//!
//! ```sh
//! $ binaryornot image.png
//! true
//!
//! $ binaryornot README.md
//! false
//! ```
//!
//! ## Python bindings
//!
//! Build with `cargo build --features pyo3-ext` to produce a Python extension module.
//! Use `maturin build` for a wheel distribution.

pub mod check;
pub mod extensions;
pub mod features;
pub mod signatures;
pub mod tree;

#[cfg(feature = "pyo3-ext")]
mod pyo3;
