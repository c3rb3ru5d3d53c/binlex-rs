pub mod models;
pub mod formats;
pub mod types;
pub mod config;

use crate::models::models_init;
use crate::formats::formats_init;
use crate::types::types_init;
use crate::config::config_init;
use crate::config::Config;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
fn binlex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(formats_init))?;
    m.add_wrapped(wrap_pymodule!(models_init))?;
    m.add_wrapped(wrap_pymodule!(types_init))?;
    m.add_wrapped(wrap_pymodule!(config_init))?;
    m.add_class::<Config>()?;
    Ok(())
}
