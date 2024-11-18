use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Error;
use crate::models::binary::Binary;
use crate::models::controlflow::graph::Graph;
use crate::models::hashing::sha256::SHA256;
use crate::models::hashing::tlsh::TLSH;
use crate::models::hashing::minhash::MinHash32;

/// Represents a JSON-serializable structure containing metadata about a signature.
#[derive(Serialize, Deserialize)]
pub struct SignatureJson {
    /// The raw pattern string of the signature.
    pub pattern: String,
    /// The normalized form of the signature, if enabled.
    pub normalized: Option<String>,
    /// The feature vector extracted from the signature.
    pub feature: Vec<u8>,
    /// The entropy of the normalized signature, if enabled.
    pub entropy: Option<f64>,
    /// The SHA-256 hash of the normalized signature, if enabled.
    pub sha256: Option<String>,
    /// The MinHash of the normalized signature, if enabled.
    pub minhash: Option<String>,
    /// The TLSH (Locality Sensitive Hash) of the normalized signature, if enabled.
    pub tlsh: Option<String>,
}

/// Represents a signature within a control flow graph.
pub struct Signature<'a> {
    /// The starting address of the signature.
    pub start_address: u64,
    /// The ending address of the signature.
    pub end_address: u64,
    /// The control flow graph the signature belongs to.
    pub cfg: &'a Graph <'a>,
}

impl<'a> Signature<'a> {
    /// Creates a new `Signature` instance for a specified address range within a control flow graph.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The starting address of the signature.
    /// * `end_address` - The ending address of the signature.
    /// * `cfg` - A reference to the control flow graph the signature belongs to.
    /// * `options` - Graph options containing signature-related settings.
    ///
    /// # Returns
    ///
    /// Returns a new `Signature` instance.
    pub fn new(start_address: u64, end_address: u64, cfg: &'a Graph) -> Self {
        Self {
            start_address: start_address,
            end_address: end_address,
            cfg: cfg,
        }
    }

    /// Retrieves the raw bytes within the address range of the signature.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<u8>` containing the raw bytes of the signature.
    pub fn bytes(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for entry in self.cfg.instructions.range(self.start_address..=self.end_address){
            let instruction = entry.value();
            result.extend(instruction.bytes.clone());
        }
        return result;
    }

    /// Retrieves the pattern string representation of the signature.
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the pattern representation of the signature.
    pub fn pattern(&self) -> String {
        let mut result: String = String::new();
        for entry in self.cfg.instructions.range(self.start_address..=self.end_address){
            let instruction = entry.value();
            result += instruction.pattern.as_str();
        }
        return result;
    }

    /// Extracts the feature vector from the normalized signature, if enabled.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<u8>` containing the feature vector, or an empty vector if feature extraction is disabled.
    pub fn feature(&self) -> Vec<u8> {
        if !self.cfg.config.heuristics.features.enabled { return Vec::<u8>::new(); }
        self.normalize()
            .iter()
            .flat_map(|byte| vec![((byte & 0xf0) >> 4) as u8, (byte & 0x0f) as u8])
            .collect()
    }

    /// Normalizes the signature to remove unknown bytes and reconstruct valid bytes.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<u8>` containing the normalized bytes of the signature.
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

    /// Retrieves the normalized form of the signature as a hexadecimal string, if enabled.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the normalized hexadecimal representation, or `None` if normalization is disabled.
    pub fn normalized(&self) -> Option<String> {
        if !self.cfg.config.heuristics.normalization.enabled{ return None; }
        Some(Binary::to_hex(&self.normalize()))
    }

    /// Computes the TLSH (Locality Sensitive Hash) of the normalized signature, if enabled.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the TLSH, or `None` if TLSH is disabled.
    pub fn tlsh(&self) -> Option<String> {
        if !self.cfg.config.hashing.tlsh.enabled { return None; }
        return TLSH::new(&self.normalize(), self.cfg.config.hashing.tlsh.minimum_byte_size).hexdigest();
    }

    /// Computes the MinHash of the normalized signature, if enabled.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the MinHash, or `None` if MinHash is disabled.
    #[allow(dead_code)]
    pub fn minhash(&self) -> Option<String> {
        if !self.cfg.config.hashing.minhash.enabled { return None; }
        if self.normalize().len() > self.cfg.config.hashing.minhash.maximum_byte_size { return None; }
        return MinHash32::new(
            &self.normalize(),
            self.cfg.config.hashing.minhash.number_of_hashes,
            self.cfg.config.hashing.minhash.shingle_size,
            self.cfg.config.hashing.minhash.seed).hexdigest();
    }

    /// Computes the SHA-256 hash of the normalized signature, if enabled.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the SHA-256 hash, or `None` if SHA-256 is disabled.
    pub fn sha256(&self) -> Option<String> {
        if !self.cfg.config.hashing.sha256.enabled { return None; }
        SHA256::new(&self.normalize()).hexdigest()
    }

    /// Computes the entropy of the normalized signature, if enabled.
    ///
    /// # Returns
    ///
    /// Returns `Some(f64)` containing the entropy, or `None` if entropy calculation is disabled.
    pub fn entropy(&self) -> Option<f64> {
        if !self.cfg.config.heuristics.entropy.enabled { return None; }
        Binary::entropy(&self.normalize())
    }

    /// Processes the signature into its JSON-serializable representation.
    ///
    /// # Returns
    ///
    /// Returns a `SignatureJson` struct containing metadata about the signature.
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

    /// Converts the signature metadata into a JSON string representation.
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the JSON representation of the signature,
    /// or an `Err` if serialization fails.
    #[allow(dead_code)]
    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result =  serde_json::to_string(&raw)?;
        Ok(result)
    }

}
