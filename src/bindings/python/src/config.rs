use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use binlex::config::Config as InnerConfig;

#[pyclass]
pub struct ConfigDisassembler {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigDisassembler {

    #[getter]
    pub fn get_sweep(&self) -> PyResult<ConfigDisassemblerSweep> {
        Ok(ConfigDisassemblerSweep {
            inner: Arc::clone(&self.inner),
        })
    }
}

#[pyclass]
pub struct ConfigDisassemblerSweep {
    inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigDisassemblerSweep {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.disassembler.sweep.enabled
    }

    #[setter]
    pub fn set_enabled(&self, value: bool) {
        let mut inner = self.inner.lock().unwrap();
        inner.disassembler.sweep.enabled = value;
    }
}

#[pyclass]
pub struct ConfigMmapCache {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigMmapCache {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.mmap.cache.enabled
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) -> PyResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.mmap.cache.enabled = value;
        Ok(())
    }
}

#[pyclass]
pub struct ConfigMmap {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigMmap {
    #[getter]
    pub fn get_directory(&self) -> String {
        let inner = self.inner.lock().unwrap();
        inner.mmap.directory.clone()
    }

    #[setter]
    pub fn set_directory(&mut self, value: String) -> PyResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.mmap.directory = value;
        Ok(())
    }

    #[getter]
    pub fn get_cache(&self) -> PyResult<ConfigMmapCache> {
        Ok(ConfigMmapCache {
            inner: Arc::clone(&self.inner),
        })
    }
}


#[pyclass]
pub struct ConfigHeuristicEntropy {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigHeuristicEntropy {
    #[getter]
    pub fn get_enabled(&self) -> PyResult<bool> {
        let inner = self.inner.lock().unwrap();
        Ok(inner.heuristics.entropy.enabled)
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) -> PyResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.heuristics.entropy.enabled = value;
        Ok(())
    }
}

#[pyclass]
pub struct ConfigHeuristicNormalization {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigHeuristicNormalization {
    #[getter]
    pub fn get_enabled(&self) -> PyResult<bool> {
        let inner = self.inner.lock().unwrap();
        Ok(inner.heuristics.normalization.enabled)
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) -> PyResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.heuristics.normalization.enabled = value;
        Ok(())
    }
}

#[pyclass]
pub struct ConfigHeuristicFeatures {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigHeuristicFeatures {
    #[getter]
    pub fn get_enabled(&self) -> PyResult<bool> {
        let inner = self.inner.lock().unwrap();
        Ok(inner.heuristics.features.enabled)
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) -> PyResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.heuristics.features.enabled = value;
        Ok(())
    }
}

#[pyclass]
pub struct ConfigFileHashes {
    inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigFileHashes {

    #[getter]
    pub fn get_sha256(&self) -> ConfigSHA256 {
        ConfigSHA256 {
            inner: Arc::clone(&self.inner)
        }
    }

    #[getter]
    pub fn get_tlsh(&self) -> ConfigTLSH {
        ConfigTLSH {
            inner: Arc::clone(&self.inner)
        }
    }
}

#[pyclass]
pub struct ConfigTLSH {
    inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigTLSH {
    #[getter]
    pub fn get_enabled(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.hashing.tlsh.enabled
    }

    #[setter]
    pub fn set_enabled(&self, value: bool) {
        let mut inner = self.inner.lock().unwrap();
        inner.hashing.tlsh.enabled = value;
    }

    #[getter]
    pub fn get_minimum_byte_size(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.hashing.tlsh.minimum_byte_size
    }

    #[setter]
    pub fn set_minimum_byte_size(&mut self, value: usize) {
        let mut inner = self.inner.lock().unwrap();
        inner.hashing.tlsh.minimum_byte_size = value;
    }

}

#[pyclass]
pub struct Config {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl Config {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(InnerConfig::new())),
        }
    }

    #[getter]
    pub fn get_hashing(&self) -> PyResult<ConfigHashing> {
        Ok(ConfigHashing {
            inner: Arc::clone(&self.inner),
        })
    }

    #[getter]
    pub fn get_general(&self) -> PyResult<ConfigGeneral> {
        Ok(ConfigGeneral {
            inner: Arc::clone(&self.inner),
        })
    }

    #[getter]
    pub fn get_heuristics(&self) -> PyResult<ConfigHeuristics> {
        Ok(ConfigHeuristics {
            inner: Arc::clone(&self.inner),
        })
    }

    #[getter]
    pub fn get_mmap(&self) -> PyResult<ConfigMmap> {
        Ok(ConfigMmap {
            inner: Arc::clone(&self.inner),
        })
    }

    #[getter]
    pub fn get_disassembler(&self) -> PyResult<ConfigDisassembler> {
        Ok(ConfigDisassembler {
            inner: Arc::clone(&self.inner),
        })
    }
}

#[pyclass]
pub struct ConfigGeneral {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigGeneral {
    #[getter]
    pub fn get_threads(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.general.threads
    }

    #[setter]
    pub fn set_threads(&self, value: usize) {
        let mut inner = self.inner.lock().unwrap();
        inner.general.threads = value;
    }

    #[getter]
    pub fn get_minimal(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.general.minimal
    }

    #[setter]
    pub fn set_minimal(&self, value: bool) {
        let mut inner = self.inner.lock().unwrap();
        inner.general.minimal = value;
    }

    #[getter]
    pub fn get_debug(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.general.debug
    }

    #[setter]
    pub fn set_debug(&self, value: bool) {
        let mut inner = self.inner.lock().unwrap();
        inner.general.debug = value;
    }

}



#[pyclass]
pub struct ConfigHeuristics {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigHeuristics {
    #[getter]
    pub fn get_features(&self) -> PyResult<ConfigHeuristicFeatures> {
        Ok(ConfigHeuristicFeatures {
            inner: Arc::clone(&self.inner),
        })
    }
    #[getter]
    pub fn get_normalization(&self) -> PyResult<ConfigHeuristicNormalization> {
        Ok(ConfigHeuristicNormalization {
            inner: Arc::clone(&self.inner),
        })
    }
    #[getter]
    pub fn get_entropy(&self) -> PyResult<ConfigHeuristicEntropy> {
        Ok(ConfigHeuristicEntropy {
            inner: Arc::clone(&self.inner),
        })
    }
}


#[pyclass]
pub struct ConfigHashing {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigHashing {
    #[getter]
    pub fn get_minhash(&self) -> PyResult<ConfigMinhash> {
        Ok(ConfigMinhash {
            inner: Arc::clone(&self.inner),
        })
    }

    #[getter]
    pub fn get_sha256(&self) -> PyResult<ConfigSHA256> {
        Ok(ConfigSHA256 {
            inner: Arc::clone(&self.inner),
        })
    }

    #[getter]
    pub fn get_tlsh(&self) -> PyResult<ConfigTLSH> {
        Ok(ConfigTLSH {
            inner: Arc::clone(&self.inner),
        })
    }

    #[getter]
    pub fn get_file(&self) -> PyResult<ConfigFileHashes> {
        Ok(ConfigFileHashes {
            inner: Arc::clone(&self.inner),
        })
    }
}

#[pyclass]
pub struct ConfigSHA256 {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigSHA256 {
    #[getter]
    pub fn get_enabled(&self) -> PyResult<bool> {
        let inner = self.inner.lock().unwrap();
        Ok(inner.hashing.sha256.enabled)
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) -> PyResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.hashing.sha256.enabled = value;
        Ok(())
    }

}

#[pyclass]
pub struct ConfigMinhash {
    pub inner: Arc<Mutex<InnerConfig>>,
}

#[pymethods]
impl ConfigMinhash {
    #[getter]
    pub fn enabled(&self) -> PyResult<bool> {
        let inner = self.inner.lock().unwrap();
        Ok(inner.hashing.minhash.enabled)
    }

    #[setter]
    pub fn set_enabled(&mut self, value: bool) -> PyResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.hashing.minhash.enabled = value;
        Ok(())
    }

    #[getter]
    pub fn number_of_hashes(&self) -> PyResult<usize> {
        let inner = self.inner.lock().unwrap();
        Ok(inner.hashing.minhash.number_of_hashes)
    }

    #[setter]
    pub fn set_number_of_hashes(&mut self, value: usize) -> PyResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.hashing.minhash.number_of_hashes = value;
        Ok(())
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
