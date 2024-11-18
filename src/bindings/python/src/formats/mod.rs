pub mod file;
pub mod pe;

use crate::formats::file::file_init;
use crate::formats::pe::pe_init;
use crate::formats::pe::PE;
use crate::formats::file::File;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
#[pyo3(name = "formats")]
pub fn formats_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(file_init))?;
    m.add_wrapped(wrap_pymodule!(pe_init))?;
    m.add_class::<PE>()?;
    m.add_class::<File>()?;
     py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.formats", m)?;
    m.setattr("__name__", "binlex.formats")?;
    Ok(())
}
