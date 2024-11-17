use pyo3::prelude::*;

use std::io::Error;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use binlex::formats::pe::PE as InnerPe;
use crate::models::binary::BinaryArchitecture;
use crate::types::memorymappedfile::MemoryMappedFile;
use pyo3::types::PyType;

#[pyclass(unsendable)]
pub struct PE {
    pub inner: InnerPe,
}

#[pymethods]
impl PE {
    #[new]
    #[pyo3(text_signature = "(path)")]
    pub fn new(path: String) -> Result<Self, Error> {
        let inner = InnerPe::new(path)?;
        Ok(Self{
            inner: inner,
        })
    }

    #[classmethod]
    #[pyo3(text_signature = "(bytes)")]
    pub fn from_bytes(_: &Bound<'_, PyType>, bytes: Vec<u8>) -> PyResult<Self> {
        let inner = InnerPe::from_bytes(bytes)?;
        Ok(Self { inner })
    }

    #[pyo3(text_signature = "($self, relative_virtual_address)")]
    pub fn relative_virtual_address_to_virtual_address(&self, relative_virtual_address: u64) -> u64 {
        self.inner.relative_virtual_address_to_virtual_address(relative_virtual_address)
    }

    #[pyo3(text_signature = "($self, offset)")]
    pub fn file_offset_to_virtual_address(&self, file_offset: u64) -> Option<u64> {
        self.inner.file_offset_to_virtual_address(file_offset)
    }

    #[pyo3(text_signature = "($self)")]
    pub fn architecture(&self) -> BinaryArchitecture {
        return BinaryArchitecture::new(self.inner.architecture() as u16);
    }

    #[pyo3(text_signature = "($self)")]
    pub fn executable_virtual_address_ranges(&self) -> BTreeMap<u64, u64> {
        self.inner.executable_virtual_address_ranges()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn pogos(&self) -> HashMap<u64, String> {
        self.inner.pogos()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn tlscallbacks(&self) -> BTreeSet<u64> {
        self.inner.tlscallbacks()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn functions(&self) -> BTreeSet<u64> {
        self.inner.functions()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn entrypoint(&self) -> u64  {
        self.inner.entrypoint()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn sizeofheaders(&self) -> u64 {
        self.inner.sizeofheaders()
    }

    #[pyo3(text_signature = "($self, file_path, cache)")]
    pub fn image(&self, py: Python<'_>, file_path: String, cache: bool) -> PyResult<Py<MemoryMappedFile>> {
        let result = self.inner.image(file_path, cache).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(e.to_string())
        })?;
        let py_memory_mapped_file = Py::new(py, MemoryMappedFile { inner: result })?;
        Ok(py_memory_mapped_file)
    }

    #[pyo3(text_signature = "($self)")]
    pub fn size(&self) -> u64 {
        self.inner.size()
    }

    #[staticmethod]
    pub fn align_section_virtual_address(value: u64, section_alignment: u64, file_alignment: u64) -> u64 {
        InnerPe::align_section_virtual_address(value, section_alignment, file_alignment)
    }

    #[pyo3(text_signature = "($self)")]
    pub fn exports(&self) -> BTreeSet<u64> {
        self.inner.exports()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn tlsh(&self) -> Option<String> {
        self.inner.tlsh()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn sha256(&self) -> Option<String> {
        self.inner.sha256()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn imagebase(&self) -> u64 {
        self.inner.imagebase()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn section_alignment(&self) -> u64 {
        self.inner.section_alignment()
    }

    #[pyo3(text_signature = "($self)")]
    pub fn file_alignment(&self) -> u64 {
        self.inner.file_alignment()
    }

}

#[pymodule]
#[pyo3(name = "pe")]
pub fn pe_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PE>()?;
     py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.formats.pe", m)?;
    m.setattr("__name__", "binlex.formats.pe")?;
    Ok(())
}
