pub mod binary;
pub mod hashing;
pub mod disassemblers;
pub mod controlflow;

use crate::models::binary::binary_init;
use crate::models::hashing::hashing_init;
use crate::models::disassemblers::disassemblers_init;
use crate::models::controlflow::controlflow_init;
use crate::models::binary::Binary;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
#[pyo3(name = "models")]
pub fn models_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(binary_init))?;
    m.add_wrapped(wrap_pymodule!(hashing_init))?;
    m.add_wrapped(wrap_pymodule!(disassemblers_init))?;
    m.add_wrapped(wrap_pymodule!(controlflow_init))?;
    m.add_class::<Binary>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models", m)?;
    m.setattr("__name__", "binlex.models")?;
    Ok(())
}
