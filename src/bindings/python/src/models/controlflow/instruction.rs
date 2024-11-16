use pyo3::prelude::*;

use std::io::Error;
use std::collections::BTreeSet;
use binlex::models::controlflow::instruction::Instruction as InnerInstruction;

use crate::models::binary::BinaryArchitecture;

#[pyclass]
pub struct Instruction {
    pub inner: InnerInstruction,
}

#[pymethods]
impl Instruction {
    #[new]
    #[pyo3(text_signature = "(address, architecture)")]
    pub fn new(py: Python, address: u64, architecture: Py<BinaryArchitecture>) -> Self {
        Self {
            inner: InnerInstruction::new(address, architecture.borrow(py).inner),
        }
    }

    #[pyo3(text_signature = "($self)")]
    pub fn blocks(&self) -> BTreeSet<u64> {
        self.inner.blocks()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn next(&self) -> Option<u64> {
        self.inner.next()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn to(&self) -> BTreeSet<u64> {
        self.inner.to()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn functions(&self) -> BTreeSet<u64> {
        self.inner.functions()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn json(&self) -> Result<String, Error> {
        self.inner.json()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn print(&self) {
        self.inner.print()
    }

}

#[pymodule]
#[pyo3(name = "instruction")]
pub fn instruction_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Instruction>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.controlflow.instruction", m)?;
    m.setattr("__name__", "binlex.models.controlflow.instruction")?;
    Ok(())
}
