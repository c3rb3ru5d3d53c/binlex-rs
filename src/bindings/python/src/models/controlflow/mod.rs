pub mod graph;
pub mod instruction;
pub mod block;
pub mod function;

use crate::models::controlflow::graph::graph_init;
use crate::models::controlflow::instruction::instruction_init;
use crate::models::controlflow::block::block_init;
use crate::models::controlflow::function::function_init;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
#[pyo3(name = "controlflow")]
pub fn controlflow_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(graph_init))?;
    m.add_wrapped(wrap_pymodule!(instruction_init))?;
    m.add_wrapped(wrap_pymodule!(block_init))?;
    m.add_wrapped(wrap_pymodule!(function_init))?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.controlflow", m)?;
    m.setattr("__name__", "binlex.models.controlflow")?;
    Ok(())
}