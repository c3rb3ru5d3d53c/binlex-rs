use std::fs::File as StdFile;
use std::io::{Read, Error};
use crate::models::hashing::sha256::SHA256;
use crate::models::hashing::tlsh::TLSH;
use std::io::ErrorKind;

/// Represents a file with its contents and an optional file path.
pub struct File {
    /// The contents of the file as a byte vector.
    pub data: Vec<u8>,
    /// The path of the file, if available.
    pub path: Option<String>,
}

impl File {
    /// Creates a new `File` instance with a given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A `String` representing the path to the file.
    ///
    /// # Returns
    ///
    /// A `File` instance with the given path and empty data.
    pub fn new(path: String) -> Self {
        Self {
            data: Vec::new(),
            path: Some(path),
        }
    }

    /// Creates a new `File` instance from the provided byte data.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A `Vec<u8>` representing the byte data of the file.
    ///
    /// # Returns
    ///
    /// A `File` instance with the given byte data and no path.
    #[allow(dead_code)]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            data: bytes,
            path: None,
        }
    }

    /// Computes the TLSH (Trend Locality Sensitive Hashing) of the file's data.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the hexadecimal representation of the TLSH,
    /// or `None` if the file's size is zero or less.
    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        if self.size() <= 0 { return None; }
        TLSH::new(&self.data, 50).hexdigest()
    }


    /// Computes the SHA-256 hash of the file's data.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the hexadecimal representation of the SHA-256 hash,
    /// or `None` if the file's size is zero or less.
    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        if self.size() <= 0 { return None; }
        SHA256::new(&self.data).hexdigest()
    }

    /// Returns the size of the file in bytes.
    ///
    /// # Returns
    ///
    /// The size of the file in bytes as a `u64`.
    #[allow(dead_code)]
    pub fn size(&self) -> u64 {
        self.data.len() as u64
    }

    /// Reads the content of the file from the given path and stores it in `data`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    /// Returns `Ok(())` on success, or an `Err` with an `Error` if the file cannot be read.
    ///
    /// # Errors
    ///
    /// Returns an error if the file path is missing or the file cannot be opened or read.
    pub fn read(&mut self) -> Result<(), Error> {
        if self.path.is_none() { return Err(Error::new(ErrorKind::InvalidInput, "missing file path to write")); }
        let mut file = StdFile::open(&self.path.clone().unwrap())?;
        file.read_to_end(&mut self.data)?;
        Ok(())
    }

}
