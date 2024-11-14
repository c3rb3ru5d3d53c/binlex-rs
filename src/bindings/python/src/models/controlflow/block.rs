use pyo3::prelude::*;

use crate::models::controlflow::graph::Graph;

use pyo3::Py;
use std::io::Error;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use binlex::models::controlflow::block::Block as InnerBlock;

#[pyclass]
pub struct Block {
    pub address: u64,
    pub cfg: Py<Graph>,
}

macro_rules! with_inner_block {
    ($self:ident, $py:ident, $method:ident) => {{
        let binding = &$self.cfg.borrow($py).inner;
        let b = InnerBlock::new($self.address, binding)?;
        b.$method()
    }};
}

#[pymethods]
impl Block {
    #[new]
    #[pyo3(text_signature = "(address, cfg)")]
    fn new(address: u64, cfg: Py<Graph>) -> PyResult<Self>  {
        Ok(Self { address: address, cfg:  cfg})
    }

    #[pyo3(text_signature = "($self)")]
    fn bytes(&self, py: Python) -> PyResult<Vec<u8>> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.bytes())
        Ok(with_inner_block!(self, py, bytes))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn is_prologue(&self, py: Python) -> PyResult<bool> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.is_prologue())
        Ok(with_inner_block!(self, py, is_prologue))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn edges(&self, py: Python) -> PyResult<usize> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.edges())
        Ok(with_inner_block!(self, py, edges))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn next(&self, py: Python) -> PyResult<Option<u64>> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.next())
        Ok(with_inner_block!(self, py, next))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn to(&self, py: Python) -> PyResult<BTreeSet<u64>> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.to())
        Ok(with_inner_block!(self, py, to))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn entropy(&self, py: Python) -> PyResult<Option<f64>> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.entropy())
        Ok(with_inner_block!(self, py, entropy))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn blocks(&self, py: Python) -> PyResult<BTreeSet<u64>> {
        let binding = &self.cfg.borrow(py).inner;
        let b = InnerBlock::new(self.address, binding)?;
        Ok(b.blocks())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn instruction_count(&self, py: Python) -> PyResult<usize> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.instruction_count())
        Ok(with_inner_block!(self, py, instruction_count))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn functions(&self, py: Python) -> PyResult<BTreeMap<u64, u64>> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.functions())
        Ok(with_inner_block!(self, py, functions))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn tlsh(&self, py: Python) -> PyResult<Option<String>> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.tlsh())
        Ok(with_inner_block!(self, py, tlsh))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn sha256(&self, py: Python) -> PyResult<Option<String>> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.sha256())
        Ok(with_inner_block!(self, py, sha256))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn minhash(&self, py: Python) -> PyResult<Option<String>> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.minhash())
        Ok(with_inner_block!(self, py, minhash))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn end(&self, py: Python) -> PyResult<u64> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.end())
        Ok(with_inner_block!(self, py, end))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn size(&self, py: Python) -> PyResult<usize> {
        // let binding = &self.cfg.borrow(py).inner;
        // let b = InnerBlock::new(self.address, binding)?;
        // Ok(b.size())
        Ok(with_inner_block!(self, py, size))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn json(&self, py: Python) -> Result<String, Error> {
        let binding = &self.cfg.borrow(py).inner;
        let b = InnerBlock::new(self.address, binding)?;
        b.json()
    }

}


#[pymodule]
#[pyo3(name = "block")]
pub fn block_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Block>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.controlflow.block", m)?;
    m.setattr("__name__", "binlex.models.controlflow.block")?;
    Ok(())
}
