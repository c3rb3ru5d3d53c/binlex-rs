use pyo3::prelude::*;
use std::collections::BTreeSet;
use binlex::controlflow::GraphQueue as InnerGraphQueue;
use binlex::controlflow::Graph as InnerGraph;
use crate::controlflow::Instruction;
use crate::BinaryArchitecture;
use crate::config::Config;
use std::sync::{Arc, Mutex};

#[pyclass]
pub struct GraphQueue {
    pub inner: InnerGraphQueue,
}

#[pymethods]
impl GraphQueue {
    #[new]
    #[pyo3(text_signature = "()")]
    pub fn new() -> Self {
        Self {
            inner: InnerGraphQueue::new(),
        }
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn insert_invalid(&mut self, address: u64) {
        self.inner.insert_invalid(address);
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn is_invalid(&self, address: u64) -> bool {
        self.inner.is_invalid(address)
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn is_valid(&self, address: u64) -> bool {
        self.inner.is_valid(address)
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn insert_valid(&mut self, address: u64) {
        self.inner.insert_valid(address);
    }

    #[pyo3(text_signature = "($self, addresses)")]
    pub fn set_processed_extend(&mut self, addresses: BTreeSet<u64>) {
        self.inner.insert_processed_extend(addresses);
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn set_processed(&mut self, address: u64) {
        self.inner.insert_processed(address);
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn is_processed(&self, address: u64) -> bool {
        self.inner.is_processed(address)
    }

    #[pyo3(text_signature = "($self, addresses)")]
    pub fn enqueue_extend(&mut self, addresses: BTreeSet<u64>) {
        self.inner.enqueue_extend(addresses);
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn enqueue(&mut self, address: u64) -> bool {
        self.inner.enqueue(address)
    }

    #[pyo3(text_signature = "($self)")]
    pub fn dequeue(&mut self) -> Option<u64> {
        self.inner.dequeue()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn dequeue_all(&mut self) -> BTreeSet<u64> {
        self.inner.dequeue_all()
    }

}

#[pyclass]
pub struct Graph {
    pub inner: Arc<Mutex<InnerGraph<'static>>>,
}

#[pymethods]
impl Graph {
    #[new]
    #[pyo3(text_signature = "(architecture, config)")]
    pub fn new(py: Python, architecture: Py<BinaryArchitecture>, config: &Bound<'_, Config>) -> Self {
        let config_arc_mutex = config.clone().unbind().borrow(py).inner.clone();
        let config_lock = config_arc_mutex.lock().unwrap();
        let config_clone = (*config_lock).clone();
        let boxed_config = Box::new(config_clone);
        let leaked_config = Box::leak(boxed_config);
        let inner = InnerGraph::new(architecture.borrow(py).inner, leaked_config);
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    #[pyo3(text_signature = "($self, instruction)")]
    pub fn insert_instruction(&mut self, py: Python, instruction: Py<Instruction>) {
        self.inner.lock().unwrap().insert_instruction(instruction.borrow_mut(py).inner.clone());
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn is_instruction_address(&self, address: u64) -> bool {
        self.inner.lock().unwrap().is_instruction_address(address)
    }

    #[getter]
    pub fn get_blocks(&self, py: Python) -> Py<GraphQueue> {
        Py::new(py, GraphQueue {
            inner: self.inner.lock().unwrap().blocks.clone(),
        }).expect("failed to create blocks graph queue")
    }

    #[setter]
    pub fn set_blocks(&mut self, py: Python, queue: Py<GraphQueue>) -> PyResult<()> {
        self.inner.lock().unwrap().blocks = queue.borrow_mut(py).inner.clone();
        Ok(())
    }

    #[getter]
    pub fn get_functions(&self, py: Python) -> Py<GraphQueue> {
        Py::new(py, GraphQueue{
            inner: self.inner.lock().unwrap().functions.clone(),
        }).expect("failed to create functions graph queue")
    }

    #[setter]
    pub fn set_functions(&mut self, py: Python, queue: Py<GraphQueue>) -> PyResult<()> {
        self.inner.lock().unwrap().functions = queue.borrow_mut(py).inner.clone();
        Ok(())
    }

    #[pyo3(text_signature = "($self, cfg)")]
    pub fn absorb(&mut self, py: Python, cfg: Py<Self>) {
        self.inner.lock().unwrap().absorb(&mut cfg.borrow_mut(py).inner.lock().unwrap());
    }

}


#[pymodule]
#[pyo3(name = "graph")]
pub fn graph_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GraphQueue>()?;
    m.add_class::<Graph>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.controlflow.graph", m)?;
    m.setattr("__name__", "binlex.models.controlflow.graph")?;
    Ok(())
}
