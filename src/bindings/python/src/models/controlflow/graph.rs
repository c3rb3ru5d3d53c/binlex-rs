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
        self.inner.set_processed_extend(addresses);
    }

    #[pyo3(text_signature = "($self, address)")]
    pub fn set_processed(&mut self, address: u64) {
        self.inner.set_processed(address);
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
}

#[pymethods]
impl Graph {
    #[new]
    #[pyo3(text_signature = "()")]
    pub fn new() -> Self {
        Self {
            inner: InnerGraph::new(),
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
    fn options(&self) -> GraphOptions{
        let mut options = GraphOptions::new();
        options.enable_entropy = self.inner.options.enable_entropy;
        options.enable_minhash = self.inner.options.enable_minhash;
        options.minhash_maximum_byte_size = self.inner.options.minhash_maximum_byte_size;
        options.minhash_number_of_hashes = self.inner.options.minhash_number_of_hashes;
        options.minhash_shingle_size = self.inner.options.minhash_shingle_size;
        options.minhash_seed = self.inner.options.minhash_seed;
        options.enable_entropy = self.inner.options.enable_entropy;
        options.enable_tlsh = self.inner.options.enable_tlsh;
        options.enable_sha256 = self.inner.options.enable_sha256;
        options.enable_feature = self.inner.options.enable_feature;
        options.tlsh_mininum_byte_size = self.inner.options.tlsh_mininum_byte_size;
        options.enable_normalized = self.inner.options.enable_normalized;
        options.file_sha256 = self.inner.options.file_sha256.clone();
        options.file_tlsh = self.inner.options.file_tlsh.clone();
        options.file_size = self.inner.options.file_size;
        options.tags = self.inner.options.tags.clone();
        return options;
    }

    #[setter]
    fn set_options(&mut self, options: &GraphOptions) {
        self.inner.options.enable_minhash = options.enable_minhash;
        self.inner.options.minhash_maximum_byte_size = options.minhash_maximum_byte_size;
        self.inner.options.minhash_number_of_hashes = options.minhash_number_of_hashes;
        self.inner.options.minhash_shingle_size = options.minhash_shingle_size;
        self.inner.options.minhash_seed = options.minhash_seed;
        self.inner.options.enable_entropy = options.enable_entropy;
        self.inner.options.enable_tlsh = options.enable_tlsh;
        self.inner.options.enable_sha256 = options.enable_sha256;
        self.inner.options.enable_feature = options.enable_feature;
        self.inner.options.tlsh_mininum_byte_size = options.tlsh_mininum_byte_size;
        self.inner.options.enable_normalized = options.enable_normalized;
        self.inner.options.file_sha256 = options.file_sha256.clone();
        self.inner.options.file_tlsh = options.file_tlsh.clone();
        self.inner.options.file_size = options.file_size;
        self.inner.options.tags = options.tags.clone();
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

