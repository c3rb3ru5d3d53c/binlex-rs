use tlsh;

/// Represents a wrapper around the TLSH (Trend Micro Locality Sensitive Hash) functionality.
///
/// This struct provides functionality for creating TLSH hashes from a slice of bytes with a minimum
/// byte size requirement, which ensures only sufficiently large data is hashed.
pub struct TLSH <'tlsh> {
    /// The slice of bytes to be hashed.
    pub bytes: &'tlsh [u8],
    /// The minimum required byte size for hashing.
    pub mininum_byte_size: usize,
}

impl <'tlsh> TLSH <'tlsh> {
    /// Creates a new `TLSH` instance with the provided bytes and minimum byte size.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A reference to the byte slice that will be hashed.
    /// * `mininum_byte_size` - The minimum size of `bytes` required for hashing.
    ///
    /// # Returns
    ///
    /// Returns a `TLSH` instance initialized with the provided byte slice and minimum byte size.
    #[allow(dead_code)]
    pub fn new(bytes: &'tlsh [u8], mininum_byte_size: usize) -> Self {
        Self {
            bytes: bytes,
            mininum_byte_size: mininum_byte_size,
        }
    }

    /// Computes the TLSH hash of the byte slice if it meets the minimum size requirement.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the hexadecimal digest of the TLSH hash if the byte slice
    /// length is greater than or equal to `mininum_byte_size`. Returns `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let data = vec![1, 2, 3, 4, 5];
    /// let tlsh = TLSH::new(&data, 5);
    /// if let Some(digest) = tlsh.hexdigest() {
    ///     println!("TLSH digest: {}", digest);
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn hexdigest(&self) -> Option<String> {
        if self.bytes.len() < self.mininum_byte_size { return None; }
        tlsh::hash_buf(&self.bytes).ok().map(|h| h.to_string())
    }

}
