use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;

#[pyfunction(name = "compressToBase64")]
pub fn compress_to_base64(input: &PyString) -> PyResult<String> {
    let input = input.to_str()?;
    Ok(lz_str::compress_to_base64(input))
}

#[pyfunction(name = "decompressFromBase64")]
pub fn decompress_from_base64(input: &PyString) -> PyResult<String> {
    let input = input.to_str()?;
    match lz_str::decompress_from_base64(input) {
        Some(result) => {
            // TODO: Make string from invalid unicode
            match String::from_utf16(&result) {
                Ok(value) => Ok(value),
                Err(_e) => Err(PyValueError::new_err("invalid unicode output")),
            }
        }
        None => Err(PyValueError::new_err("invalid input")),
    }
}

#[pymodule]
fn lz_str_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compress_to_base64, m)?)?;
    m.add_function(wrap_pyfunction!(decompress_from_base64, m)?)?;
    Ok(())
}
