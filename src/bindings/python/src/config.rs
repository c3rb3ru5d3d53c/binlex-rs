use pyo3::prelude::*;
use pyo3::types::PyType;
use binlex::config::Config as InnerConfig;
use binlex::config::ConfigGeneral as InnerConfigGeneral;
use binlex::config::ConfigHeuristicFeatures as InnerConfigHeuristicFeatures;
use binlex::config::ConfigHeuristicNormalization as InnerConfigHeuristicNormalization;
use binlex::config::ConfigHeuristicEntropy as InnerConfigHeuristicEntropy;
use std::io::Error;
use binlex::config::ConfigHeuristics as InnerConfigHeuristics;
use binlex::config::ConfigSHA256 as InnerConfigSHA256;
use binlex::config::ConfigTLSH as InnerConfigTLSH;
use binlex::config::ConfigMinhash as InnerConfigMinhash;
use binlex::config::ConfigHashing as InnerConfigHashing;
use binlex::config::ConfigFileHashes as InnerConfigFileHashes;
use binlex::config::ConfigMmap as InnerConfigMmap;
use binlex::config::ConfigMmapCache as InnerConfigMmapCache;
use binlex::config::ConfigDisassembler as InnerConfigDisassembler;
use binlex::config::ConfigDisassemblerSweep as InnerConfigDisassemblerSweep;

#[pyclass]
pub struct ConfigDisassemblerSweep {
    inner: InnerConfigDisassemblerSweep,
}

#[pymethods]
impl ConfigDisassemblerSweep {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }
}

#[pyclass]
pub struct ConfigDisassembler {
    inner: InnerConfigDisassembler,
}

#[pymethods]
impl ConfigDisassembler {
    #[getter]
    pub fn get_sweep(&self) -> ConfigDisassemblerSweep {
        ConfigDisassemblerSweep {
            inner: self.inner.sweep.clone()
        }
    }

    #[setter]
    pub fn set_sweep(&mut self, py: Python, value: Py<ConfigDisassemblerSweep>) {
        self.inner.sweep = value.borrow(py).inner.clone();
    }
}

#[pyclass]
pub struct ConfigMmapCache {
    inner: InnerConfigMmapCache,
}

#[pymethods]
impl ConfigMmapCache {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }
}

#[pyclass]
pub struct ConfigMmap {
    inner: InnerConfigMmap,
}

#[pymethods]
impl ConfigMmap {
    #[getter]
    pub fn get_directory(&self) -> String {
        self.inner.directory.clone()
    }

    #[setter]
    pub fn set_directory(&mut self, value: String) {
        self.inner.directory = value;
    }

    #[getter]
    pub fn get_cache(&self) -> ConfigMmapCache {
        ConfigMmapCache {
            inner: self.inner.cache.clone()
        }
    }

    #[setter]
    pub fn set_cache(&mut self, py: Python, value: Py<ConfigMmapCache>) {
        self.inner.cache = value.borrow(py).inner.clone();
    }

}

#[pyclass]
pub struct ConfigFileHashes {
    inner: InnerConfigFileHashes,
}

#[pymethods]
impl ConfigFileHashes {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }
}

#[pyclass]
pub struct ConfigHashing {
    inner: InnerConfigHashing,
}

#[pymethods]
impl ConfigHashing {
    #[getter]
    pub fn get_sha256(&self) -> ConfigSHA256 {
        ConfigSHA256 {
            inner: self.inner.sha256.clone()
        }
    }

    #[setter]
    pub fn set_sha256(&mut self, py: Python, value: Py<ConfigSHA256>) {
        self.inner.sha256 = value.borrow(py).inner.clone();
    }

    #[getter]
    pub fn get_tlsh(&self) -> ConfigTLSH {
        ConfigTLSH {
            inner: self.inner.tlsh.clone()
        }
    }

    #[setter]
    pub fn set_tlsh(&mut self, py: Python, value: Py<ConfigTLSH>) {
        self.inner.tlsh = value.borrow(py).inner.clone();
    }

    #[getter]
    pub fn get_minhash(&self) -> ConfigMinhash {
        ConfigMinhash {
            inner: self.inner.minhash.clone()
        }
    }

    #[setter]
    pub fn set_minhash(&mut self, py: Python, value: Py<ConfigMinhash>) {
        self.inner.minhash = value.borrow(py).inner.clone();
    }

    #[getter]
    pub fn get_file(&self) -> ConfigFileHashes {
        ConfigFileHashes {
            inner: self.inner.file.clone()
        }
    }

    #[setter]
    pub fn set_file(&mut self, py: Python, value: Py<ConfigFileHashes>) {
        self.inner.file = value.borrow(py).inner.clone();
    }

}

#[pyclass]
pub struct ConfigMinhash {
    inner: InnerConfigMinhash,
}

#[pymethods]
impl ConfigMinhash {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }

    #[getter]
    pub fn get_number_of_hashes(&self) -> usize {
        self.inner.number_of_hashes
    }

    #[setter]
    pub fn set_number_of_hashes(&mut self, value: usize) {
        self.inner.number_of_hashes = value;
    }

    #[getter]
    pub fn get_shingle_size(&self) -> usize {
        self.inner.shingle_size
    }

    #[setter]
    pub fn set_shingle_size(&mut self, value: usize) {
        self.inner.shingle_size = value;
    }

    #[getter]
    pub fn get_maximum_byte_size(&self) -> usize {
        self.inner.maximum_byte_size
    }

    #[setter]
    pub fn set_maximum_byte_size(&mut self, value: usize) {
        self.inner.maximum_byte_size = value;
    }

    #[getter]
    pub fn get_seed(&self) -> u64 {
        self.inner.seed
    }

    #[setter]
    pub fn set_seed(&mut self, value: u64) {
        self.inner.seed = value;
    }


}

#[pyclass]
pub struct ConfigTLSH {
    inner: InnerConfigTLSH,
}

#[pymethods]
impl ConfigTLSH {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }

    #[getter]
    pub fn get_minimum_byte_size(&self) -> usize {
        self.inner.minimum_byte_size
    }

    #[setter]
    pub fn set_minimum_byte_size(&mut self, value: usize) {
        self.inner.minimum_byte_size = value
    }
}


#[pyclass]
pub struct ConfigSHA256 {
    inner: InnerConfigSHA256,
}

#[pymethods]
impl ConfigSHA256 {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }
}

#[pyclass]
pub struct ConfigHeuristics {
    inner: InnerConfigHeuristics,
}

#[pymethods]
impl ConfigHeuristics{
    #[getter]
    pub fn get_features(&self) -> ConfigHeuristicFeatures {
        ConfigHeuristicFeatures {
            inner: self.inner.features.clone(),
        }
    }

    #[setter]
    pub fn set_features(&mut self, py: Python, value: Py<ConfigHeuristicFeatures>) {
        self.inner.features = value.borrow(py).inner.clone();
    }

    #[getter]
    pub fn get_normalization(&self) -> ConfigHeuristicNormalization {
        ConfigHeuristicNormalization {
            inner: self.inner.normalization.clone(),
        }
    }

    #[setter]
    pub fn set_normalization(&mut self, py: Python, value: Py<ConfigHeuristicNormalization>) {
        self.inner.normalization = value.borrow(py).inner.clone();
    }

    #[getter]
    pub fn get_entropy(&self) -> ConfigHeuristicEntropy {
        ConfigHeuristicEntropy {
            inner: self.inner.entropy.clone(),
        }
    }

    #[setter]
    pub fn set_entropy(&mut self, py: Python, value: Py<ConfigHeuristicEntropy>) {
        self.inner.entropy = value.borrow(py).inner.clone();
    }
}

#[pyclass]
pub struct ConfigHeuristicEntropy {
    inner: InnerConfigHeuristicEntropy,
}

#[pymethods]
impl ConfigHeuristicEntropy {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }
}

#[pyclass]
pub struct ConfigHeuristicNormalization {
    inner: InnerConfigHeuristicNormalization,
}

#[pymethods]
impl ConfigHeuristicNormalization {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }
}

#[pyclass]
pub struct ConfigHeuristicFeatures {
    inner: InnerConfigHeuristicFeatures,
}

#[pymethods]
impl ConfigHeuristicFeatures {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        self.inner.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.enabled = value;
    }
}

#[pyclass]
pub struct ConfigGeneral {
    inner: InnerConfigGeneral,
}

#[pymethods]
impl ConfigGeneral {
    #[getter]
    pub fn get_threads(&self) -> usize {
        self.inner.threads
    }

    #[setter]
    pub fn set_threads(&mut self, value: usize) {
        self.inner.threads = value
    }

    #[getter]
    pub fn get_minimal(&self) -> bool {
        self.inner.minimal
    }

    #[setter]
    pub fn set_minimal(&mut self, value: bool) {
        self.inner.minimal = value;
    }

    #[getter]
    pub fn get_debug(&self) -> bool {
        self.inner.debug
    }

    #[setter]
    pub fn set_debug(&mut self, value: bool) {
        self.inner.debug = value;
    }

    #[getter]
    pub fn get_tags(&self) -> Vec<String> {
        self.inner.tags.clone()
    }

    #[setter]
    pub fn set_tags(&mut self, value: Vec<String>) {
        self.inner.tags = value;
    }
}

#[pyclass]
pub struct Config {
    pub inner: InnerConfig,
}

#[pymethods]
impl Config {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: InnerConfig::new(),
        }
    }

    #[getter]
    fn get_heuristics(&self) -> ConfigHeuristics {
        ConfigHeuristics {
            inner: self.inner.heuristics.clone(),
        }
    }

    #[setter]
    fn set_heuristics(&mut self, py: Python, value: Py<ConfigHeuristics>) {
        self.inner.heuristics = value.borrow(py).inner.clone();
    }

    #[getter]
    fn get_general(&self) -> ConfigGeneral {
        ConfigGeneral {
            inner: self.inner.general.clone(),
        }
    }

    #[setter]
    fn set_general(&mut self, py: Python, value: Py<ConfigGeneral>) {
        self.inner.general = value.borrow(py).inner.clone();
    }

    #[getter]
    fn get_hashing(&self) -> ConfigHashing {
        ConfigHashing {
            inner: self.inner.hashing.clone(),
        }
    }

    #[setter]
    fn set_hashing(&mut self, py: Python, value: Py<ConfigHashing>) {
        self.inner.hashing = value.borrow(py).inner.clone();
    }

    #[getter]
    fn get_disassembler(&self) -> ConfigDisassembler {
        ConfigDisassembler {
            inner: self.inner.disassembler.clone(),
        }
    }

    #[setter]
    fn set_disassembler(&mut self, py: Python, value: Py<ConfigDisassembler>) {
        self.inner.disassembler = value.borrow(py).inner.clone();
    }

    #[pyo3(text_signature = "($self)")]
    pub fn print(&self) {
        self.inner.print();
    }

    #[pyo3(text_signature = "($self, file_path)")]
    pub fn write_to_file(&self, file_path: String) -> Result<(), Error> {
        self.inner.write_to_file(&file_path)
    }

    #[staticmethod]
    #[pyo3(text_signature = "()")]
    pub fn default_file_mapping_directory() -> String {
        InnerConfig::default_file_mapping_directory()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn to_string(&self) -> Result<String, Error> {
        self.inner.to_string()
    }

    #[classmethod]
    #[pyo3(text_signature = "(file_path)")]
    pub fn from_file(_cls: Py<PyType>, file_path: String) -> Result<Self, Error> {
        let inner = InnerConfig::from_file(&file_path)?;
        Ok(Self {
            inner: inner,
        })
    }

    pub fn write_default(&self) -> Result<(), Error> {
        self.inner.write_default()
    }

    pub fn from_default(&mut self) -> Result<(), Error> {
        self.inner.from_default()
    }
}

#[pymodule]
#[pyo3(name = "config")]
pub fn config_init(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Config>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.config", m)?;
    m.setattr("__name__", "binlex.config")?;
    Ok(())
}
