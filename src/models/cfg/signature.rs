use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Error;
use crate::models::binary::Binary;
use crate::models::cfg::graph::Graph;
use crate::models::cfg::graph::GraphOptions;

#[derive(Serialize, Deserialize)]
pub struct SignatureJson {
    pub pattern: String,
    pub normalized: Option<String>,
    pub feature: Vec<u8>,
    pub entropy: Option<f64>,
    pub sha256: Option<String>,
    pub minhash: Option<String>,
    pub tlsh: Option<String>,
}

pub struct Signature<'a> {
    pub start_address: u64,
    pub end_address: u64,
    pub cfg: &'a Graph,
    pub options: GraphOptions,
}

impl<'a> Signature<'a> {
    pub fn new(start_address: u64, end_address: u64, cfg: &'a Graph, options: GraphOptions) -> Self {
        Self {
            start_address: start_address,
            end_address: end_address,
            cfg: cfg,
            options: options,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for entry in self.cfg.instructions.range(self.start_address..=self.end_address){
            let instruction = entry.value();
            result.extend(instruction.bytes.clone());
        }
        return result;
    }

    pub fn pattern(&self) -> String {
        let mut result: String = String::new();
        for entry in self.cfg.instructions.range(self.start_address..=self.end_address){
            let instruction = entry.value();
            result += instruction.signature.as_str();
        }
        return result;
    }

    pub fn feature(&self) -> Vec<u8> {
        if !self.options.enable_feature { return Vec::<u8>::new(); }
        self.normalize()
            .iter()
            .flat_map(|byte| vec![((byte & 0xf0) >> 4) as u8, (byte & 0x0f) as u8])
            .collect()
    }

    pub fn normalize(&self) -> Vec<u8> {
        let signature: Vec<char> = self.pattern().chars().collect();
        let mut bytes = Vec::<u8>::new();
        let mut byte_accumulator = 0u8;
        let mut nibble_count = 0;
        for (i, &byte) in self.bytes().iter().enumerate() {
            if signature.get(i * 2).copied() != Some('?') {
                byte_accumulator = (byte & 0xf0) >> 4;
                nibble_count += 1;
            }

            if signature.get(i * 2 + 1).copied() != Some('?') {
                byte_accumulator = (byte_accumulator << 4) | (byte & 0x0f);
                nibble_count += 1;
            }

            if nibble_count == 2 {
                bytes.push(byte_accumulator);
                nibble_count = 0;
            }
        }
        bytes
    }

    pub fn normalized(&self) -> Option<String> {
        if !self.options.enable_normalized { return None; }
        Some(Binary::to_hex(&self.normalize()))
    }

    pub fn tlsh(&self) -> Option<String> {
        if !self.options.enable_tlsh { return None; }
        Binary::tlsh(&self.normalize(), self.options.tlsh_mininum_byte_size)
    }

    #[allow(dead_code)]
    pub fn minhash(&self) -> Option<String> {
        if !self.options.enable_minhash { return None; }
        Binary::minhash(
            self.options.minhash_maximum_byte_size,
            self.options.minhash_number_of_hashes,
            self.options.minhash_shingle_size,
            self.options.minhash_seed,
            &self.normalize())
    }

    pub fn sha256(&self) -> Option<String> {
        if !self.options.enable_sha256 { return None; }
        Binary::sha256(&self.normalize())
    }

    pub fn entropy(&self) -> Option<f64> {
        if !self.options.enable_entropy { return None; }
        Binary::entropy(&self.normalize())
    }

    pub fn process(&self) -> SignatureJson {
        SignatureJson {
            pattern: self.pattern(),
            normalized: self.normalized(),
            feature: self.feature(),
            sha256: self.sha256(),
            entropy: self.entropy(),
            minhash: self.minhash(),
            tlsh: self.tlsh(),
        }
    }

    #[allow(dead_code)]
    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result =  serde_json::to_string(&raw)?;
        Ok(result)
    }

}