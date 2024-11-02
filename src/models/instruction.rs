use std::io::Error;
use crate::models::binary::Binary;
use serde::{Deserialize, Serialize};
use serde_json;

pub struct Instruction {
    pub address: u64,
    pub bytes: Vec<u8>,
    pub signature: String,
    pub is_ret: bool,
}

#[derive(Serialize, Deserialize)]
pub struct InstructionJson {
    pub address: u64,
    pub bytes: String,
    pub signature: String,
}

impl Instruction {
    #[allow(dead_code)]
    pub fn new(address: u64, bytes: Vec<u8>, signature: String, is_ret: bool) -> Self {
        Self {
            address: address,
            bytes: bytes,
            signature: signature,
            is_ret: is_ret,
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        return self.bytes.len();
    }

    #[allow(dead_code)]
    pub fn process(&self) -> InstructionJson {
        InstructionJson {
            address: self.address,
            bytes: Binary::to_hex(&self.bytes),
            signature: self.signature.clone(),
        }
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