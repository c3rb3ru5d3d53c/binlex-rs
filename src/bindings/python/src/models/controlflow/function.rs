use pyo3::prelude::*;

use pyo3::Py;
use std::io::Error;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use binlex::controlflow::function::Function as InnerFunction;
use crate::models::controlflow::graph::Graph;

#[pyclass]
pub struct Function {
    pub address: u64,
    pub cfg: Py<Graph>,
}

#[pymethods]
impl Function {
    #[new]
    #[pyo3(text_signature = "(address, cfg)")]
    fn new(address: u64, cfg: Py<Graph>) -> PyResult<Self>  {
        Ok(Self{
            address: address,
            cfg: cfg,
        })
    }
    #[pyo3(text_signature = "($self)")]
    fn bytes(&self, py: Python) -> PyResult<Option<Vec<u8>>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.bytes())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn edges(&self, py: Python) -> PyResult<usize> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.edges())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn bytes_to_hex(&self, py: Python) -> PyResult<Option<String>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.bytes_to_hex())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn size(&self, py: Python) -> PyResult<Option<usize>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.size())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn is_prologue(&self, py: Python) -> PyResult<bool> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.is_prologue())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn instruction_count(&self, py: Python) -> PyResult<usize> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.instruction_count())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn block_addresses(&self, py: Python) -> PyResult<BTreeSet<u64>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.block_addresses())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn end(&self, py: Python) -> PyResult<Option<u64>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.end())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn entropy(&self, py: Python) -> PyResult<Option<f64>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.entropy())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn sha256(&self, py: Python) -> PyResult<Option<String>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.sha256())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn tlsh(&self, py: Python) -> PyResult<Option<String>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.tlsh())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn minhash(&self, py: Python) -> PyResult<Option<String>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.minhash())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn functions(&self, py: Python) -> PyResult<BTreeMap<u64, u64>> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.functions())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn is_contiguous(&self, py: Python) -> PyResult<bool> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        Ok(function.is_contiguous())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn print(&self, py: Python)  -> PyResult<()> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        function.print();
        Ok(())
    }

    #[pyo3(text_signature = "($self)")]
    pub fn json(&self, py: Python) -> Result<String, Error> {
        let binding = self.cfg.borrow(py);
        let inner = binding.inner.lock().unwrap();
        let function = InnerFunction::new(self.address, &inner)?;
        function.json()
    }

}

#[pymodule]
#[pyo3(name = "function")]
pub fn function_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Function>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.controlflow.function", m)?;
    m.setattr("__name__", "binlex.models.controlflow.function")?;
    Ok(())
}
