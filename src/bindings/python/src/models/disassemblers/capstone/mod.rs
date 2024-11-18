pub mod disassembler;

use pyo3::{prelude::*, wrap_pymodule};

use crate::models::disassemblers::capstone::disassembler::disassembler_init;
use crate::models::disassemblers::capstone::disassembler::Disassembler;

#[pymodule]
#[pyo3(name = "capstone")]
pub fn capstone_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(disassembler_init))?;
    m.add_class::<Disassembler>()?;
     py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.disassemblers.capstone", m)?;
    m.setattr("__name__", "binlex.models.disassemblers.capstone")?;
    Ok(())
}
