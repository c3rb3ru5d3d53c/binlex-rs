pub mod capstone;

use crate::models::disassemblers::capstone::capstone_init;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
#[pyo3(name = "disassemblers")]
pub fn disassemblers_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(capstone_init))?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.disassemblers", m)?;
    m.setattr("__name__", "binlex.models.disassemblers")?;
    Ok(())
}