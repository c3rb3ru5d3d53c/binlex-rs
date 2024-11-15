use pyo3::prelude::*;

use std::collections::BTreeSet;
use binlex::models::controlflow::graph::GraphOptions as InnerGraphOptions;
use binlex::models::controlflow::graph::GraphQueue as InnerGraphQueue;
use binlex::models::controlflow::graph::Graph as InnerGraph;
use crate::models::controlflow::instruction::Instruction;

#[pyclass]
pub struct GraphOptions {
    #[pyo3(get, set)]
    pub enable_minhash: bool,
    #[pyo3(get, set)]
    pub minhash_maximum_byte_size: usize,
    #[pyo3(get, set)]
    pub minhash_number_of_hashes: usize,
    #[pyo3(get, set)]
    pub minhash_shingle_size: usize,
    #[pyo3(get, set)]
    pub minhash_seed: u64,
    #[pyo3(get, set)]
    pub enable_tlsh: bool,
    #[pyo3(get, set)]
    pub enable_sha256: bool,
    #[pyo3(get, set)]
    pub enable_entropy: bool,
    #[pyo3(get, set)]
    pub enable_feature: bool,
    #[pyo3(get, set)]
    pub tlsh_mininum_byte_size: usize,
    #[pyo3(get, set)]
    pub enable_normalized: bool,
    #[pyo3(get, set)]
    pub file_sha256: Option<String>,
    #[pyo3(get, set)]
    pub file_tlsh: Option<String>,
    #[pyo3(get, set)]
    pub file_size: Option<u64>,
    #[pyo3(get, set)]
    pub tags: Vec<String>,
}

#[pymethods]
impl GraphOptions {
    #[new]
    #[pyo3(text_signature = "()")]
    pub fn new() -> Self {
        let inner = InnerGraphOptions::new();
        return Self {
            enable_minhash: inner.enable_minhash,
            minhash_maximum_byte_size: inner.minhash_maximum_byte_size,
            minhash_number_of_hashes: inner.minhash_number_of_hashes,
            minhash_shingle_size: inner.minhash_shingle_size,
            minhash_seed: inner.minhash_seed,
            enable_entropy: inner.enable_entropy,
            enable_tlsh: inner.enable_tlsh,
            enable_sha256: inner.enable_sha256,
            enable_feature: inner.enable_feature,
            tlsh_mininum_byte_size: inner.tlsh_mininum_byte_size,
            enable_normalized: inner.enable_normalized,
            file_sha256: inner.file_sha256,
            file_tlsh: inner.file_tlsh,
            file_size: inner.file_size,
            tags: inner.tags,
        };
    }
}

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
    pub inner: InnerGraph,
    pub inner_options: InnerGraphOptions,
}

#[pymethods]
impl Graph {
    #[new]
    #[pyo3(text_signature = "()")]
    pub fn new() -> Self {
        let inner = InnerGraph::new();
        let inner_options = inner.options.clone();
        Self {
            inner: inner,
            inner_options: inner_options,
        }
    }

    #[pyo3(text_signature = "($self, instruction)")]
    pub fn insert_instruction(&mut self, py: Python, instruction: Py<Instruction>) {
        self.inner.insert_instruction(instruction.borrow_mut(py).inner.clone())
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn is_instruction_address(&self, address: u64) -> bool {
        self.inner.is_instruction_address(address)
    }

    #[getter]
    pub fn get_blocks(&self, py: Python) -> Py<GraphQueue> {
        Py::new(py, GraphQueue{
            inner: self.inner.blocks.clone(),
        }).expect("failed to create blocks graph queue")
    }

    #[setter]
    pub fn set_blocks(&mut self, py: Python, queue: Py<GraphQueue>) -> PyResult<()> {
        self.inner.blocks = queue.borrow_mut(py).inner.clone();
        Ok(())
    }

    #[getter]
    pub fn get_functions(&self, py: Python) -> Py<GraphQueue> {
        Py::new(py, GraphQueue{
            inner: self.inner.functions.clone(),
        }).expect("failed to create functions graph queue")
    }

    #[setter]
    pub fn set_functions(&mut self, py: Python, queue: Py<GraphQueue>) -> PyResult<()> {
        self.inner.functions = queue.borrow_mut(py).inner.clone();
        Ok(())
    }

    #[setter]
    fn set_option_enable_minhash(&mut self, enable_minhash: bool) {
        self.inner.options.enable_minhash = enable_minhash;
    }

    #[getter]
    fn get_option_enable_minhash(&self) -> bool {
        self.inner.options.enable_minhash
    }

    #[setter]
    fn set_option_minhash_maximum_byte_size(&mut self, minhash_maximum_byte_size: usize) {
        self.inner.options.minhash_maximum_byte_size = minhash_maximum_byte_size;
    }

    #[getter]
    fn get_option_minhash_maximum_byte_size(&self) -> usize {
        self.inner.options.minhash_maximum_byte_size
    }

    #[setter]
    fn set_option_minhash_number_of_hashes(&mut self, minhash_number_of_hashes: usize) {
        self.inner.options.minhash_number_of_hashes = minhash_number_of_hashes;
    }

    #[getter]
    fn get_option_minhash_number_of_hashes(&self) -> usize {
        self.inner.options.minhash_number_of_hashes
    }

    #[setter]
    fn set_option_minhash_shingle_size(&mut self, minhash_shingle_size: usize) {
        self.inner.options.minhash_shingle_size = minhash_shingle_size;
    }

    #[getter]
    fn get_option_minhash_shingle_size(&self) -> usize {
        self.inner.options.minhash_shingle_size
    }

    #[setter]
    fn set_option_minhash_seed(&mut self, minhash_seed: u64) {
        self.inner.options.minhash_seed = minhash_seed;
    }

    #[getter]
    fn get_option_minhash_seed(&self) -> u64 {
        self.inner.options.minhash_seed
    }

    #[setter]
    fn set_option_enable_entropy(&mut self, enable_entropy: bool) {
        self.inner.options.enable_entropy = enable_entropy;
    }

    #[getter]
    fn get_option_enable_entropy(&self) -> bool {
        self.inner.options.enable_entropy
    }

    #[setter]
    fn set_option_enable_tlsh(&mut self, enable_tlsh: bool) {
        self.inner.options.enable_tlsh = enable_tlsh;
    }

    #[getter]
    fn get_option_enable_tlsh(&self) -> bool {
        self.inner.options.enable_tlsh
    }

    #[setter]
    fn set_option_enable_sha256(&mut self, enable_sha256: bool) {
        self.inner.options.enable_sha256 = enable_sha256;
    }

    #[getter]
    fn get_option_enable_sha256(&self) -> bool {
        self.inner.options.enable_sha256
    }

    #[setter]
    fn set_option_enable_feature(&mut self, enable_feature: bool) {
        self.inner.options.enable_feature = enable_feature;
    }

    #[getter]
    fn get_option_enable_feature(&self) -> bool {
        self.inner.options.enable_feature
    }

    #[setter]
    fn set_option_tlsh_mininum_byte_size(&mut self, tlsh_mininum_byte_size: usize) {
        self.inner.options.tlsh_mininum_byte_size = tlsh_mininum_byte_size;
    }

    #[getter]
    fn get_option_tlsh_mininum_byte_size(&self) -> usize {
        self.inner.options.tlsh_mininum_byte_size
    }

    #[setter]
    fn set_option_enable_normalized(&mut self, enable_normalized: bool) {
        self.inner.options.enable_normalized = enable_normalized;
    }

    #[getter]
    fn get_option_enable_normalized(&self) -> bool {
        self.inner.options.enable_normalized
    }

    #[setter]
    fn set_option_file_sha256(&mut self, file_sha256: String) {
        self.inner.options.file_sha256 = Some(file_sha256);
    }

    #[getter]
    fn get_option_file_sha256(&self) -> Option<String> {
        self.inner.options.file_sha256.clone()
    }

    #[setter]
    fn set_option_file_tlsh(&mut self, file_tlsh: String) {
        self.inner.options.file_tlsh = Some(file_tlsh);
    }

    #[getter]
    fn get_option_file_tlsh(&self) -> Option<String> {
        self.inner.options.file_tlsh.clone()
    }

    #[setter]
    fn set_option_file_size(&mut self, file_size: u64) {
        self.inner.options.file_size = Some(file_size);
    }

    #[getter]
    fn get_option_file_size(&self) -> Option<u64> {
        self.inner.options.file_size
    }

    #[setter]
    fn set_option_tags(&mut self, tags: Vec<String>) {
        self.inner.options.tags = tags;
    }

    #[getter]
    fn get_option_tags(&self) -> Vec<String> {
        self.inner.options.tags.clone()
    }

    #[pyo3(text_signature = "($self, cfg)")]
    pub fn absorb(&mut self, py: Python, cfg: Py<Self>) {
        let mut a = cfg.borrow_mut(py);
        self.inner.absorb(&mut a.inner);
    }

}




#[pymodule]
#[pyo3(name = "graph")]
pub fn graph_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GraphOptions>()?;
    m.add_class::<GraphQueue>()?;
    m.add_class::<Graph>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.controlflow.graph", m)?;
    m.setattr("__name__", "binlex.models.controlflow.graph")?;
    Ok(())
}

