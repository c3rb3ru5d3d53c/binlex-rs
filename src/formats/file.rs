use std::fs::File as StdFile;
use std::io::{Read, Error};

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
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn read(&mut self) -> Result<(), Error> {
        let mut file = StdFile::open(&self.path)?;
        file.read_to_end(&mut self.data)?;
        Ok(())
    }
}