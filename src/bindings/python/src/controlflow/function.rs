use pyo3::prelude::*;

use pyo3::Py;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use binlex::controlflow::Function as InnerFunction;
use crate::controlflow::Graph;

use std::sync::Arc;
use std::sync::Mutex;

#[pyclass]
pub struct Function {
    pub address: u64,
    pub cfg: Py<Graph>,
    inner_function_cache: Arc<Mutex<Option<InnerFunction<'static>>>>,
}

impl Function {
    fn with_inner_function<F, R>(&self, py: Python, f: F) -> PyResult<R>
    where
        F: FnOnce(&InnerFunction<'static>) -> PyResult<R>,
    {
        let mut cache = self.inner_function_cache.lock().unwrap();

        if cache.is_none() {
            let binding = self.cfg.borrow(py);
            let inner = binding.inner.lock().unwrap();

            let inner_ref: &'static _ = unsafe { std::mem::transmute(&*inner) };
            let inner_block = InnerFunction::new(self.address, inner_ref)?;
            *cache = Some(inner_block);
        }

        f(cache.as_ref().unwrap())
    }
}

#[pymethods]
impl Function {
    #[new]
    #[pyo3(text_signature = "(address, cfg)")]
    fn new(address: u64, cfg: Py<Graph>) -> PyResult<Self> {
        Ok(Self {
            address,
            cfg,
            inner_function_cache: Arc::new(Mutex::new(None)),
        })
    }

    #[pyo3(text_signature = "($self)")]
    fn bytes(&self, py: Python) -> PyResult<Option<Vec<u8>>> {
        self.with_inner_function(py, |function| Ok(function.bytes()))
    }

    #[pyo3(text_signature = "($self)")]
    fn bytes_to_hex(&self, py: Python) -> PyResult<Option<String>> {
        self.with_inner_function(py, |function| Ok(function.bytes_to_hex()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn is_prologue(&self, py: Python) -> PyResult<bool> {
        self.with_inner_function(py, |function| Ok(function.is_prologue()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn edges(&self, py: Python) -> PyResult<usize> {
        self.with_inner_function(py, |function| Ok(function.edges()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn entropy(&self, py: Python) -> PyResult<Option<f64>> {
        self.with_inner_function(py, |function| Ok(function.entropy()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn block_addresses(&self, py: Python) -> PyResult<BTreeSet<u64>> {
        self.with_inner_function(py, |function| Ok(function.block_addresses()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn instruction_count(&self, py: Python) -> PyResult<usize> {
        self.with_inner_function(py, |function| Ok(function.instruction_count()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn functions(&self, py: Python) -> PyResult<BTreeMap<u64, u64>> {
        self.with_inner_function(py, |function| Ok(function.functions()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn tlsh(&self, py: Python) -> PyResult<Option<String>> {
        self.with_inner_function(py, |function| Ok(function.tlsh()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn sha256(&self, py: Python) -> PyResult<Option<String>> {
        self.with_inner_function(py, |function| Ok(function.sha256()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn minhash(&self, py: Python) -> PyResult<Option<String>> {
        self.with_inner_function(py, |function| Ok(function.minhash()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn size(&self, py: Python) -> PyResult<Option<usize>> {
        self.with_inner_function(py, |function| Ok(function.size()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn is_contiguous(&self, py: Python) -> PyResult<bool> {
        self.with_inner_function(py, |function| Ok(function.is_contiguous()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn end(&self, py: Python) -> PyResult<Option<u64>> {
        self.with_inner_function(py, |function| Ok(function.end()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn print(&self, py: Python) -> PyResult<()> {
        self.with_inner_function(py, |function| Ok(function.print()))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn json(&self, py: Python) -> PyResult<String> {
        self.with_inner_function(py, |block| {
            block.json().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
        })
    }
}

#[pymodule]
#[pyo3(name = "function")]
pub fn function_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Function>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.controlflow.function", m)?;
    m.setattr("__name__", "binlex.controlflow.function")?;
    Ok(())
}
