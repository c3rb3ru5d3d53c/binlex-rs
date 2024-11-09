use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use twox_hash::XxHash32;
use std::hash::{Hash, Hasher};

const PRIME_MODULUS: u32 = 4294967291;

pub struct MinHash32 <'minhash32> {
    a_coefficients: Vec<u32>,
    b_coefficients: Vec<u32>,
    num_hashes: usize,
    shingle_size: usize,
    bytes: &'minhash32 [u8],
}

impl <'minhash32> MinHash32 <'minhash32> {

    pub fn new(bytes: &'minhash32 [u8], num_hashes: usize, shingle_size: usize, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let max_hash: u32 = u32::MAX;
        let mut a_coefficients = Vec::with_capacity(num_hashes);
        let mut b_coefficients = Vec::with_capacity(num_hashes);

        for _ in 0..num_hashes {
            a_coefficients.push(rng.gen_range(1..max_hash));
            b_coefficients.push(rng.gen_range(0..max_hash));
        }

        Self {
            a_coefficients: a_coefficients,
            b_coefficients: b_coefficients,
            num_hashes: num_hashes,
            shingle_size: shingle_size,
            bytes: bytes,
        }
    }

    pub fn hash(&self) -> Option<Vec<u32>> {
        if self.bytes.len() < self.shingle_size { return None; }
        let mut min_hashes = vec![u32::MAX; self.num_hashes];
        for shingle in self.bytes.windows(self.shingle_size) {
            let mut hasher = XxHash32::default();
            shingle.hash(&mut hasher);
            let shingle_hash = hasher.finish() as u32;
            for i in 0..self.num_hashes {
                let a = self.a_coefficients[i];
                let b = self.b_coefficients[i];
                let hash_value = (a.wrapping_mul(shingle_hash).wrapping_add(b)) % PRIME_MODULUS;
                if hash_value < min_hashes[i] {
                    min_hashes[i] = hash_value;
                }
            }
        }
        Some(min_hashes)
    }

    #[allow(dead_code)]
    pub fn jaccard_similarity(hash1: &[u32], hash2: &[u32]) -> f64 {
        if hash1.len() != hash2.len() { return 0.0; }
        let mut intersection = 0;
        for i in 0..hash1.len() {
            if hash1[i] == hash2[i] {
                intersection += 1;
            }
        }
        intersection as f64 / hash1.len() as f64
    }

    pub fn hexdigest(&self) -> Option<String> {
        self.hash().map(|minhash| {
            minhash.iter()
                .map(|hash| format!("{:08x}", hash))
                .collect()
        })
    }
}

