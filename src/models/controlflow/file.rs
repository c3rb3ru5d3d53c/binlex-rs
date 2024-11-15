use std::io::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use crate::models::controlflow::graph::GraphOptions;

/// Represents a JSON-serializable structure containing file metadata.
#[derive(Serialize, Deserialize)]
pub struct FileJson {
    /// The SHA-256 hash of the file, if available.
    pub sha256: Option<String>,
    /// The TLSH (Trend Micro Locality Sensitive Hash) of the file, if available.
    pub tlsh: Option<String>,
    /// The size of the file in bytes, if available.
    pub size: Option<u64>,
}

/// Represents file metadata derived from `GraphOptions`.
pub struct File {
    /// Options containing file-specific metadata, such as hashes and size.
    pub options: GraphOptions,
}

impl File {
    /// Creates a new `File` instance with the provided `GraphOptions`.
    ///
    /// # Arguments
    ///
    /// * `options` - A `GraphOptions` instance containing the file metadata.
    ///
    /// # Returns
    ///
    /// Returns a new `File` instance.
    pub fn new(options: GraphOptions) -> Self {
        Self {
            options: options,
        }
    }

    /// Retrieves the TLSH (Locality Sensitive Hash) of the file, if available.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the TLSH hash, or `None` if unavailable.
    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        self.options.file_tlsh.clone()
    }

    /// Retrieves the SHA-256 hash of the file, if available.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the SHA-256 hash, or `None` if unavailable.
    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        self.options.file_sha256.clone()
    }

    /// Retrieves the size of the file in bytes, if available.
    ///
    /// # Returns
    ///
    /// Returns `Some(u64)` containing the file size, or `None` if unavailable.
    #[allow(dead_code)]
    pub fn size(&self) -> Option<u64> {
        self.options.file_size.clone()
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
            size: self.size(),
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
