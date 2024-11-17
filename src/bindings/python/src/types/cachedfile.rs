use pyo3::prelude::*;
use pyo3::exceptions;
use pyo3::types::{PyBytes, PyMemoryView};
use memmap2::Mmap;
use binlex::types::memorymappedfile::MemoryMappedFile as InnerMemoryMappedFile;

#[pyclass]
pub struct MemoryMappedFile {
    inner: InnerMemoryMappedFile,
}

#[pymethods]
impl MemoryMappedFile {
    #[new]
    pub fn new(path: &str, append: bool, cache: bool) -> PyResult<Self> {
        let path = std::path::PathBuf::from(path);
        let inner = InnerMemoryMappedFile::new(path, append, cache)
            .map_err(|e| exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(MemoryMappedFile { inner })
    }

    #[getter]
    pub fn is_cached(&self) -> bool {
        self.inner.is_cached()
    }

    #[getter]
    pub fn path(&self) -> String {
        self.inner.path()
    }

    pub fn write(&mut self, data: &[u8]) -> PyResult<u64> {
        let mut reader = std::io::Cursor::new(data);
        self.inner
            .write(&mut reader)
            .map_err(|e| exceptions::PyIOError::new_err(e.to_string()))
    }

    pub fn write_padding(&mut self, length: usize) -> PyResult<()> {
        self.inner
            .write_padding(length)
            .map_err(|e| exceptions::PyIOError::new_err(e.to_string()))
    }

    #[getter]
    pub fn size(&self) -> PyResult<u64> {
        self.inner
            .size()
            .map_err(|e| exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Maps the file into memory and returns a MappedFile object.
    pub fn mmap(&self) -> PyResult<MappedFile> {
        let mmap = self
            .inner
            .mmap()
            .map_err(|e| exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(MappedFile { mmap })
    }
}

#[pyclass]
pub struct MappedFile {
    mmap: Mmap,
}

#[pymethods]
impl MappedFile {
    /// Returns a memoryview of the mapped file.
    pub fn as_memoryview<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyMemoryView>> {
        let data = &self.mmap[..];
        let py_bytes = PyBytes::new_bound(py, data);
        PyMemoryView::from_bound(&py_bytes)
    }
}

#[pymodule]
#[pyo3(name = "memorymappedfile")]
pub fn cachedfile_init(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MemoryMappedFile>()?;
    m.add_class::<MappedFile>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.types.memorymappedfile", m)?;
    m.setattr("__name__", "binlex.types.memorymappedfile")?;
    Ok(())
}
