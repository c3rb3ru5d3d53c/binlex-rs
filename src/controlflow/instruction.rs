use std::{collections::BTreeSet, io::Error};
use crate::binary::{Binary, BinaryArchitecture};
use serde::{Deserialize, Serialize};
use serde_json;

/// Represents a single instruction in disassembled binary code.
///
/// This struct encapsulates metadata and properties of an instruction,
/// such as its address, type, and relationships with other instructions.
#[derive(Clone)]
pub struct Instruction {
    /// The binary architecture
    #[allow(dead_code)]
    pub architecture: BinaryArchitecture,
    /// The address of the instruction in memory.
    pub address: u64,
    /// Indicates whether this instruction is part of a function prologue.
    pub is_prologue: bool,
    /// Indicates whether this instruction is the start of a basic block.
    pub is_block_start: bool,
    /// Indicates whether this instruction is the start of a function.
    pub is_function_start: bool,
    /// The raw bytes of the instruction.
    pub bytes: Vec<u8>,
    /// The signature pattern of the instruction.
    pub pattern: String,
    /// Indicates whether this instruction is a return instruction.
    pub is_return: bool,
    /// Indicates whether this instruction is a call instruction.
    pub is_call: bool,
    /// A set of functions that this instruction may belong to.
    pub functions: BTreeSet<u64>,
    /// Indicates whether this instruction is a jump instruction.
    pub is_jump: bool,
    /// Indicates whether this instruction is a conditional instruction.
    pub is_conditional: bool,
    /// Indicates whether this instruction is a trap instruction.
    pub is_trap: bool,
    /// A set of addresses this instruction may jump or branch to.
    pub to: BTreeSet<u64>,
    /// The number of edges (connections) for this instruction.
    pub edges: usize,
}

/// Represents a JSON-serializable view of an `Instruction`.
#[derive(Serialize, Deserialize)]
pub struct InstructionJson {
    /// The type of this entity, always `"instruction"`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The address of the instruction in memory.
    pub address: u64,
    /// Indicates whether this instruction is part of a function prologue.
    pub is_prologue: bool,
    /// Indicates whether this instruction is the start of a basic block.
    pub is_block_start: bool,
    /// Indicates whether this instruction is the start of a function.
    pub is_function_start: bool,
    /// Indicates whether this instruction is a call instruction.
    pub is_call: bool,
    /// Indicates whether this instruction is a return instruction.
    pub is_return: bool,
    /// Indicates whether this instruction is a jump instruction.
    pub is_jump: bool,
    /// Indicates whether this instruction is a trap instruction.
    pub is_trap: bool,
    /// Indicates whether this instruction is conditional.
    pub is_conditional: bool,
    /// The number of edges (connections) for this instruction.
    pub edges: usize,
    /// The raw bytes of the instruction in hexadecimal format.
    pub bytes: String,
    /// The size of the instruction in bytes.
    pub size: usize,
    /// The signature pattern of the instruction.
    pub signature: String,
    /// A set of functions that this instruction may belong to.
    pub functions: BTreeSet<u64>,
    /// A set of addresses for the blocks this instruction may branch to.
    pub blocks: BTreeSet<u64>,
    /// A set of addresses this instruction may jump or branch to.
    pub to: BTreeSet<u64>,
    /// The address of the next sequential instruction, if any.
    pub next: Option<u64>,
}

impl Instruction {
    /// Creates a new `Instruction` with the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The memory address of the instruction.
    ///
    /// # Returns
    ///
    /// Returns a new `Instruction` with default values for its properties.
    #[allow(dead_code)]
    pub fn new(address: u64, architecture: BinaryArchitecture) -> Self {
        Self {
            address: address,
            is_prologue: false,
            is_block_start: false,
            is_function_start: false,
            bytes: Vec::<u8>::new(),
            pattern: String::new(),
            is_call: false,
            is_return: false,
            functions: BTreeSet::<u64>::new(),
            is_conditional: false,
            is_jump: false,
            to: BTreeSet::<u64>::new(),
            edges: 0,
            is_trap: false,
            architecture: architecture,
        }
    }

    /// Retrieves the set of addresses for the blocks this instruction may branch to.
    ///
    /// # Returns
    ///
    /// Returns a `BTreeSet<u64>` containing the block addresses.
    pub fn blocks(&self) -> BTreeSet<u64> {
        let mut result = BTreeSet::new();
        for item in self.to.iter().map(|ref_multi| *ref_multi).chain(self.next()) {
            result.insert(item);
        }
        result
    }

    /// Retrieves the address of the next sequential instruction.
    ///
    /// # Returns
    ///
    /// Returns `Some(u64)` containing the address of the next instruction, or `None`
    /// if the current instruction is a return or trap instruction.
    pub fn next(&self) -> Option<u64> {
        if self.is_return { return None; }
        if self.is_trap { return None; }
        Some(self.address + self.size() as u64)
    }

    /// Computes the size of the instruction in bytes.
    ///
    /// # Returns
    ///
    /// Returns the size of the instruction as a `usize`.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        return self.bytes.len();
    }

    /// Converts the `Instruction` into its JSON-serializable representation.
    ///
    /// # Returns
    ///
    /// Returns an `InstructionJson` struct containing the properties of the instruction.
    #[allow(dead_code)]
    pub fn process(&self) -> InstructionJson {
        InstructionJson {
            type_: "instruction".to_string(),
            address: self.address,
            is_block_start: self.is_block_start,
            bytes: Binary::to_hex(&self.bytes),
            size: self.size(),
            signature: self.pattern.clone(),
            is_return: self.is_return,
            is_trap: self.is_trap,
            is_call: self.is_call,
            is_jump: self.is_jump,
            is_conditional: self.is_conditional,
            is_function_start: self.is_function_start,
            is_prologue: self.is_prologue,
            edges: self.edges,
            functions: self.functions(),
            blocks: self.blocks(),
            to: self.to(),
            next: self.next(),
        }
    }

    /// Retrieves the set of addresses this instruction may jump or branch to.
    ///
    /// # Returns
    ///
    /// Returns a `BTreeSet<u64>` containing the target addresses.
    pub fn to(&self) -> BTreeSet<u64> {
       return self.to.clone();
    }

    /// Retrieves the set of functions this instruction may belong to.
    ///
    /// # Returns
    ///
    /// Returns a `BTreeSet<u64>` containing the function addresses.
    pub fn functions(&self) -> BTreeSet<u64> {
        return self.functions.clone();
    }

    /// Converts the `Instruction` into a JSON string representation.
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the JSON representation, or an `Err(Error)` if serialization fails.
    #[allow(dead_code)]
    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result = serde_json::to_string(&raw)?;
        Ok(result)
    }

    /// Prints the JSON representation of the `Instruction` to standard output.
    #[allow(dead_code)]
    pub fn print(&self) {
        if let Ok(json) = self.json() {
            println!("{}", json);
        }
    }
}