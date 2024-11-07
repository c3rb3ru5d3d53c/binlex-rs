use std::convert::From;
use lz4::block::{compress, decompress};

pub struct LZ4String {
    compressed_data: Vec<u8>,
    uncompressed_size: usize,
}

impl LZ4String {

    #[allow(dead_code)]
    pub fn new(data: &str) -> Self {
        let compressed = compress(data.as_bytes(), None, false).expect("lz4string compression failed");
        LZ4String {
            compressed_data: compressed,
            uncompressed_size: data.len(),
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let decompressed = decompress(&self.compressed_data, Some(self.uncompressed_size as i32))
            .expect("lz4string decompression failed");
        String::from_utf8(decompressed).expect("lz4string invalid utf8")
    }
}

impl From<String> for LZ4String {
    fn from(data: String) -> Self {
        let compressed = compress(data.as_bytes(), None, false).expect("lz4string compression failed");
        LZ4String {
            compressed_data: compressed,
            uncompressed_size: data.len(),
        }
    }
}

impl std::fmt::Display for LZ4String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.to_string();
        write!(f, "{}", s)
    }
}