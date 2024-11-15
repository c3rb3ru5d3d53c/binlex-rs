pub mod models;
pub mod formats;

use crate::models::models_init;
use crate::formats::formats_init;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
fn binlex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(formats_init))?;
    m.add_wrapped(wrap_pymodule!(models_init))?;
    Ok(())
}
