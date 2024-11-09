use ring::digest;
use crate::models::binary::Binary;

pub struct SHA256 <'sha256> {
    pub bytes: &'sha256 [u8],
}

impl <'sha256> SHA256 <'sha256> {

    #[allow(dead_code)]
    pub fn new(bytes: &'sha256 [u8]) -> Self {
        Self {
            bytes: bytes
        }
    }

    #[allow(dead_code)]
    pub fn hexdigest(&self) -> Option<String> {
        let digest = digest::digest(&digest::SHA256, &self.bytes);
        return Some(Binary::to_hex(digest.as_ref()));
    }
}
