use std::io::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use crate::controlflow::Graph;

/// Represents a JSON-serializable structure containing file metadata.
#[derive(Serialize, Deserialize)]
pub struct FileJson {
    /// The SHA-256 hash of the file, if available.
    pub sha256: Option<String>,
    /// The TLSH (Trend Micro Locality Sensitive Hash) of the file, if available.
    pub tlsh: Option<String>,
}

/// Represents file metadata derived from `GraphOptions`.
pub struct File <'file> {
    /// Options containing file-specific metadata, such as hashes and size.
    pub cfg: &'file Graph <'file>,
}

impl <'file> File <'file> {
    /// Creates a new `File` instance with the provided `GraphOptions`.
    ///
    /// # Arguments
    ///
    /// * `options` - A `GraphOptions` instance containing the file metadata.
    ///
    /// # Returns
    ///
    /// Returns a new `File` instance.
    pub fn new(cfg: &'file Graph) -> Self {
        Self {
            cfg: cfg,
        }
    }

    /// Retrieves the TLSH (Locality Sensitive Hash) of the file, if available.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the TLSH hash, or `None` if unavailable.
    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        self.cfg.config.hashing.file.tlsh.hexdigest.clone()
    }

    /// Retrieves the SHA-256 hash of the file, if available.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the SHA-256 hash, or `None` if unavailable.
    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        self.cfg.config.hashing.file.sha256.hexdigest.clone()
    }

    /// Processes the file metadata into a JSON-serializable `FileJson` structure.
    ///
    /// # Returns
    ///
    /// Returns a `FileJson` struct containing the file's SHA-256 hash, TLSH hash, and size.
    pub fn process(&self) -> FileJson {
        FileJson {
            sha256: self.sha256(),
            tlsh: self.tlsh(),
        }
    }

    /// Prints the JSON representation of the file metadata to standard output.
    #[allow(dead_code)]
    pub fn print(&self) {
        if let Ok(json) = self.json() {
            println!("{}", json);
        }
    }

    /// Converts the file metadata into a JSON string representation.
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the JSON representation of the file metadata,
    /// or an `Err` if serialization fails.
    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result = serde_json::to_string(&raw)?;
        Ok(result)
    }

}
