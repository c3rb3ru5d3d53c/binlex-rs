use std::fs::File as StdFile;
use std::io::{Read, Error};
use ring::digest;

pub struct File {
    pub data: Vec<u8>,
    pub path: String,
    pub sha256: String,
}

impl File {
    pub fn new(path: String) -> Self {
        Self {
            data: Vec::new(),
            path,
            sha256: String::new(),
        }
    }

    fn to_hexdigest(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn read(&mut self) -> Result<(), Error> {
        let mut file = match StdFile::open(&self.path) {
            Ok(file) => {file},
            Err(error) => return Err(error),
        };
        match file.read_to_end(&mut self.data) {
            Ok(_) => {
                let sha256 = digest::digest(&digest::SHA256, &self.data);
                self.sha256 = Self::to_hexdigest(sha256.as_ref());
                Ok(())
            },
            Err(error) => return Err(error),
        }
    }
}