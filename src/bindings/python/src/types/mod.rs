pub mod memorymappedfile;

use crate::types::memorymappedfile::memorymappedfile_init;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
#[pyo3(name = "types")]
pub fn types_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(memorymappedfile_init))?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.types", m)?;
    m.setattr("__name__", "binlex.types")?;
    Ok(())
}
