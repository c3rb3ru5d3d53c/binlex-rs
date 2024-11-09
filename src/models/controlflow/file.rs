use std::io::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use crate::models::controlflow::graph::GraphOptions;

#[derive(Serialize, Deserialize)]
pub struct FileJson {
    pub sha256: Option<String>,
    pub tlsh: Option<String>,
    pub size: Option<u64>,
}

pub struct File {
    pub options: GraphOptions,
}

impl File {
    pub fn new(options: GraphOptions) -> Self {
        Self {
            options: options,
        }
    }

    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        self.options.file_tlsh.clone()
    }

    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        self.options.file_sha256.clone()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Option<u64> {
        self.options.file_size.clone()
    }

    pub fn process(&self) -> FileJson {
        FileJson {
            sha256: self.sha256(),
            tlsh: self.tlsh(),
            size: self.size(),
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if let Ok(json) = self.json() {
            println!("{}", json);
        }
    }

    pub fn json(&self) -> Result<String, Error> {
        let raw = self.process();
        let result = serde_json::to_string(&raw)?;
        Ok(result)
    }

}