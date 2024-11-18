use pyo3::prelude::*;
use pyo3::types::PyType;
use binlex::config::Config as InnerConfig;
use std::io::Error;

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
