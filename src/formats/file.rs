use std::fs::File as StdFile;
use std::io::{Read, Error};
use crate::models::binary::Binary;

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
    pub fn sha256(&self) -> Option<String> {
        Binary::sha256(&self.data)
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn read(&mut self) -> Result<(), Error> {
        let mut file = StdFile::open(&self.path)?;
        file.read_to_end(&mut self.data)?;
        Ok(())
    }
}