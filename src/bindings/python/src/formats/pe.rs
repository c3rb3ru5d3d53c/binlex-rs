use pyo3::prelude::*;

use std::io::Error;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use binlex::formats::pe::PE as InnerPe;
use crate::models::binary::BinaryArchitecture;
use pyo3::types::PyBytes;
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

    #[pyo3(text_signature = "($self)")]
    pub fn machine(&self) -> BinaryArchitecture {
        return BinaryArchitecture::new(self.inner.machine() as u16);
    }

    #[pyo3(text_signature = "($self)")]
    pub fn executable_address_ranges(&self) -> BTreeMap<u64, u64> {
        self.inner.executable_address_ranges()
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

    #[pyo3(text_signature = "($self)")]
    pub fn image(&self, py: Python<'_>) -> PyObject  {
        let data: Vec<u8> = self.inner.image();
        PyBytes::new_bound(py, &data).into()
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