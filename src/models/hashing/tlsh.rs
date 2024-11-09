use tlsh;

pub struct TLSH <'tlsh> {
    pub bytes: &'tlsh [u8],
    pub mininum_byte_size: usize,
}

impl <'tlsh> TLSH <'tlsh> {

    #[allow(dead_code)]
    pub fn new(bytes: &'tlsh [u8], mininum_byte_size: usize) -> Self {
        Self {
            bytes: bytes,
            mininum_byte_size: mininum_byte_size,
        }
    }

    #[allow(dead_code)]
    pub fn hexdigest(&self) -> Option<String> {
        if self.bytes.len() < self.mininum_byte_size { return None; }
        tlsh::hash_buf(&self.bytes).ok().map(|h| h.to_string())
    }

}
