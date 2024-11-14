use pyo3::prelude::*;

use pyo3::Py;
use std::io::Error;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use binlex::models::disassemblers::capstone::disassembler::Disassembler as InnerDisassembler;
use crate::models::binary::BinaryArchitecture;
use crate::models::controlflow::graph::Graph;
use pyo3::types::PyBytes;

#[pyclass(unsendable)]
pub struct Disassembler{
    image: Py<PyBytes>,
    machine: Py<BinaryArchitecture>,
    executable_address_ranges: BTreeMap<u64, u64>,
}

#[pymethods]
impl Disassembler {
    #[new]
    #[pyo3(text_signature = "(machine, image, executable_address_ranges)")]
    pub fn new(machine: Py<BinaryArchitecture>, image: Py<PyBytes>, executable_address_ranges: BTreeMap<u64, u64>) -> Self {
        Self {
            machine: machine,
            image: image,
            executable_address_ranges: executable_address_ranges,
        }
    }

    #[pyo3(text_signature = "($self, address, cfg)")]
    pub fn disassemble_function(&self, py: Python, address: u64, cfg: Py<Graph>) -> Result<u64, Error> {
        let machine_binding = &self.machine.borrow(py);
        let disassembler = InnerDisassembler::new(machine_binding.inner, self.image.as_bytes(py), self.executable_address_ranges.clone())?;
        let cfg_ref=  &mut cfg.borrow_mut(py);
        let result = disassembler.disassemble_function(address, &mut cfg_ref.inner)?;
        return Ok(result);
    }

    #[pyo3(text_signature = "($self, address, cfg)")]
    pub fn disassemble_block(&self, py: Python, address: u64, cfg: Py<Graph>) -> Result<u64, Error> {
        let machine_binding = &self.machine.borrow(py);
        let disassembler = InnerDisassembler::new(machine_binding.inner, self.image.as_bytes(py), self.executable_address_ranges.clone())?;
        let cfg_ref=  &mut cfg.borrow_mut(py);
        let result = disassembler.disassemble_block(address, &mut cfg_ref.inner)?;
        return Ok(result);
    }

    #[pyo3(text_signature = "($self, addresses, cfg)")]
    pub fn disassemble_controlflow(&self, py: Python, addresses: BTreeSet<u64>, cfg: Py<Graph>) -> Result<(), Error> {
        let machine_binding = &self.machine.borrow(py);
        let disassembler = InnerDisassembler::new(machine_binding.inner, self.image.as_bytes(py), self.executable_address_ranges.clone())?;
        let cfg_ref=  &mut cfg.borrow_mut(py);
        disassembler.disassemble_control_flow(addresses, &mut cfg_ref.inner)?;
        Ok(())
    }

    #[pyo3(text_signature = "($self, addresses, cfg)")]
    pub fn disassemble_linear_pass(&self, py: Python, valid_jump_threshold: usize, valid_instruction_threshold: usize) -> Result<BTreeSet<u64>, Error> {
        let machine_binding = &self.machine.borrow(py);
        let disassembler = InnerDisassembler::new(machine_binding.inner, self.image.as_bytes(py), self.executable_address_ranges.clone())?;
        let results = disassembler.disassemble_linear_pass(valid_jump_threshold, valid_instruction_threshold);
        let mut asdf = BTreeSet::<u64>::new();
        for result in results {
            asdf.insert(result);
        }
        Ok(asdf)
    }

}


#[pymodule]
#[pyo3(name = "disassembler")]
pub fn disassembler_init(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Disassembler>()?;
     py.import_bound("sys")?
        .getattr("modules")?
        .set_item("binlex.models.disassemblers.capstone.disassembler", m)?;
    m.setattr("__name__", "binlex.models.disassemblers.capstone.disassembler")?;
    Ok(())
}
