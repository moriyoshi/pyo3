use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::{Path, PathBuf};

#[pyfunction]
fn make_path() -> PathBuf {
    Path::new("/root").to_owned()
}

#[pyfunction]
fn take_pathbuf(path: PathBuf) -> PathBuf {
    path
}

#[pymodule]
fn path(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_path, m)?)?;
    m.add_function(wrap_pyfunction!(take_pathbuf, m)?)?;

    Ok(())
}
