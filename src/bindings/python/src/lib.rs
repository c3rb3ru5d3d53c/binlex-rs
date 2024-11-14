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

// Define the main Python module
// #[pymodule]
// fn binlex(m: Bound<'_, PyModule>) -> PyResult<()> {
//     let models_submodule = PyModule::new_bound(m.py(), "models")?;
//     let _ = models_submodule.add_wrapped(wrap_pymodule!(models_init));
//     m.add_submodule(&models_submodule);
//     //m.add_submodule(&models_submodule)?;

//     let formats_submodule = PyModule::new_bound(m.py(), "formats")?;
//     m.add_submodule(&formats_submodule)?;

//     Ok(())
// }