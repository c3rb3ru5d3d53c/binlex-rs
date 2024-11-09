use std::fs::File as StdFile;
use std::io::{Read, Error};
use crate::models::hashing::sha256::SHA256;
use crate::models::hashing::tlsh::TLSH;

pub struct File {
    pub data: Vec<u8>,
    pub path: String,
}

impl File {
    pub fn new(path: String) -> Self {
        Self {
            data: Vec::new(),
            path,
        }
    }

    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        TLSH::new(&self.data, 50).hexdigest()
    }

    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        SHA256::new(&self.data).hexdigest()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> u64 {
        self.data.len() as u64
    }

    pub fn read(&mut self) -> Result<(), Error> {
        let mut file = StdFile::open(&self.path)?;
        file.read_to_end(&mut self.data)?;
        Ok(())
    }
}