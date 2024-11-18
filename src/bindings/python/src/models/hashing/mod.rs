pub mod sha256;
pub mod tlsh;
pub mod minhash;

use crate::models::hashing::sha256::sha256_init;
use crate::models::hashing::tlsh::tlsh_init;
use crate::models::hashing::minhash::minhash_init;
use crate::models::hashing::minhash::MinHash32;
use crate::models::hashing::tlsh::TLSH;
use crate::models::hashing::sha256::SHA256;

use pyo3::{prelude::*, wrap_pymodule};

#[pymodule]
#[pyo3(name = "hashing")]
pub fn hashing_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(sha256_init))?;
    m.add_wrapped(wrap_pymodule!(tlsh_init))?;
    m.add_wrapped(wrap_pymodule!(minhash_init))?;
    m.add_class::<SHA256>()?;
    m.add_class::<MinHash32>()?;
    m.add_class::<TLSH>()?;
    py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.hashing", m)?;
    m.setattr("__name__", "binlex.models.hashing")?;
    Ok(())
}
