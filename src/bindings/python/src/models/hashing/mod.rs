pub mod sha256;
pub mod tlsh;
pub mod minhash;

use crate::models::hashing::sha256::sha256_init;
use crate::models::hashing::tlsh::tlsh_init;
use crate::models::hashing::minhash::minhash_init;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
#[pyo3(name = "hashing")]
pub fn hashing_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(sha256_init))?;
    m.add_wrapped(wrap_pymodule!(tlsh_init))?;
    m.add_wrapped(wrap_pymodule!(minhash_init))?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.hashing", m)?;
    m.setattr("__name__", "binlex.models.hashing")?;
    Ok(())
}
