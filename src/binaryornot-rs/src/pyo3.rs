/// Python bindings for binaryornot via PyO3.
///
/// Exposes the same API as the original Python package:
/// - `is_binary(filename, check_extensions=True) -> bool`
/// - `is_binary_string(chunk: bytes) -> bool`

use pyo3::prelude::*;
use std::path::Path;

use crate::check::{is_binary as rust_is_binary, is_binary_string as rust_is_binary_string};

/// Check if a file is binary or text.
///
/// :param filename: Path to the file to check.
/// :param check_extensions: If ``True`` (default), check the file extension
///     against a list of known binary types before reading the file.
///     Set to ``False`` to classify purely by file contents.
/// :returns: ``True`` if the file is binary, otherwise ``False``.
#[pyfunction(signature = (filename, check_extensions = true))]
#[pyo3(text_signature = "(filename, *, check_extensions=True) /")]
fn is_binary(_py: Python, filename: &str, check_extensions: bool) -> PyResult<bool> {
    let result = rust_is_binary(Path::new(filename), check_extensions)
        .map_err(|e| pyo3::exceptions::PyOSError::new_err(e.to_string()))?;
    Ok(result)
}

/// Check if a byte chunk appears to be binary or text.
///
/// :param chunk: Bytes to analyse (typically the first 512 bytes of a file).
/// :returns: ``True`` if the chunk appears to be binary, otherwise ``False``.
#[pyfunction]
#[pyo3(text_signature = "(chunk) /")]
fn is_binary_string(_py: Python, chunk: &[u8]) -> bool {
    rust_is_binary_string(chunk)
}

/// PyO3 module definition — produces the `binaryornot_rs` Python extension module.
#[pymodule]
fn binaryornot_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_binary, m)?)?;
    m.add_function(wrap_pyfunction!(is_binary_string, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
