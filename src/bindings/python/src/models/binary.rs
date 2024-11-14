use pyo3::prelude::*;

use binlex::models::binary::Binary as InnerBinary;
use binlex::models::binary::BinaryArchitecture as InnerBinaryArchitecture;

#[pyclass(eq)]
#[derive(PartialEq)]
pub struct BinaryArchitecture {
    pub inner: InnerBinaryArchitecture,
}

#[pymethods]
impl BinaryArchitecture {
    #[new]
    pub fn new(value: u16) -> Self {
        let inner = match value {
            0x00 => InnerBinaryArchitecture::AMD64,
            0x01 => InnerBinaryArchitecture::I386,
            _ => InnerBinaryArchitecture::UNKNOWN,
        };
        BinaryArchitecture { inner }
    }

    #[getter]
    pub fn get_value(&self) -> u16 {
        self.inner as u16
    }
}

#[pyclass]
pub struct Binary;

#[pymethods]
impl Binary {
    #[staticmethod]
    pub fn entropy(bytes: Vec<u8>) -> Option<f64> {
        InnerBinary::entropy(&bytes)
    }
    #[staticmethod]
    pub fn to_hex(bytes: Vec<u8>) -> String {
        InnerBinary::to_hex(&bytes)
    }
    #[staticmethod]
    pub fn hexdump(bytes: Vec<u8>, address: u64) -> String {
        InnerBinary::hexdump(&bytes, address)
    }
}

#[pymodule]
#[pyo3(name = "binary")]
pub fn binary_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BinaryArchitecture>()?;
    m.add_class::<Binary>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.binary", m)?;
    m.setattr("__name__", "binlex.models.binary")?;
    Ok(())
}
