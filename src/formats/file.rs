use std::fs::File as StdFile;
use std::io::{Read, Error};
use crate::models::hashing::sha256::SHA256;
use crate::models::hashing::tlsh::TLSH;
use std::io::ErrorKind;

pub struct File {
    pub data: Vec<u8>,
    pub path: Option<String>,
}

impl File {

    pub fn new(path: String) -> Self {
        Self {
            data: Vec::new(),
            path: Some(path),
        }
    }

    #[allow(dead_code)]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            data: bytes,
            path: None,
        }
    }

    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        if self.size() <= 0 { return None; }
        TLSH::new(&self.data, 50).hexdigest()
    }

    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        if self.size() <= 0 { return None; }
        SHA256::new(&self.data).hexdigest()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> u64 {
        self.data.len() as u64
    }

    pub fn read(&mut self) -> Result<(), Error> {
        if self.path.is_none() { return Err(Error::new(ErrorKind::InvalidInput, "missing file path to write")); }
        let mut file = StdFile::open(&self.path.clone().unwrap())?;
        file.read_to_end(&mut self.data)?;
        Ok(())
    }

}
