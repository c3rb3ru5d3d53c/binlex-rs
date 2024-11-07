use std::{collections::BTreeSet, io::Error};
use crate::models::binary::Binary;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone)]
pub struct Instruction {
    pub address: u64,
    pub is_prologue: bool,
    pub is_block_start: bool,
    pub is_function_start: bool,
    pub bytes: Vec<u8>,
    pub signature: String,
    pub is_return: bool,
    pub is_call: bool,
    pub functions: BTreeSet<u64>,
    pub is_jump: bool,
    pub is_conditional: bool,
    pub is_trap: bool,
    pub to: BTreeSet<u64>,
    pub edges: usize,
}

#[derive(Serialize, Deserialize)]
pub struct InstructionJson {
    #[serde(rename = "type")]
    pub type_: String,
    pub address: u64,
    pub is_prologue: bool,
    pub is_block_start: bool,
    pub is_function_start: bool,
    pub is_call: bool,
    pub is_return: bool,
    pub is_jump: bool,
    pub is_trap: bool,
    pub is_conditional: bool,
    pub edges: usize,
    pub bytes: String,
    pub size: usize,
    pub signature: String,
    pub functions: BTreeSet<u64>,
    pub blocks: BTreeSet<u64>,
    pub to: BTreeSet<u64>,
    pub next: Option<u64>,
}

impl Instruction {
    #[allow(dead_code)]
    pub fn new(address: u64) -> Self {
        Self {
            address: address,
            is_prologue: false,
            is_block_start: false,
            is_function_start: false,
            bytes: Vec::<u8>::new(),
            signature: String::new(),
            is_call: false,
            is_return: false,
            functions: BTreeSet::<u64>::new(),
            is_conditional: false,
            is_jump: false,
            to: BTreeSet::<u64>::new(),
            edges: 0,
            is_trap: false,
        }
    }

    pub fn blocks(&self) -> BTreeSet<u64> {
        let mut result = BTreeSet::new();
        for item in self.to.iter().map(|ref_multi| *ref_multi).chain(self.next()) {
            result.insert(item);
        }
        result
    }

    pub fn next(&self) -> Option<u64> {
        if self.is_return { return None; }
        if self.is_trap { return None; }
        Some(self.address + self.size() as u64)
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        return self.bytes.len();
    }

    #[allow(dead_code)]
    pub fn process(&self) -> InstructionJson {
        InstructionJson {
            type_: "instruction".to_string(),
            address: self.address,
            is_block_start: self.is_block_start,
            bytes: Binary::to_hex(&self.bytes),
            size: self.size(),
            signature: self.signature.clone(),
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

    pub fn to(&self) -> BTreeSet<u64> {
       return self.to.clone();
    }

    pub fn functions(&self) -> BTreeSet<u64> {
        return self.functions.clone();
    }

    #[allow(dead_code)]
    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result = serde_json::to_string(&raw)?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if let Ok(json) = self.json() {
            println!("{}", json);
        }
    }
}