use std::error::Error;
use std::path::PathBuf;
use crate::types::MemoryMappedFile;
use std::io::Error as IoError;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::fmt;
use crate::hashing::SHA256;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use crate::genomics::AllelePair;
use crate::genomics::Gene;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Genome {
    genome: Vec<AllelePair>,
    rng: Arc<Mutex<StdRng>>,
    states: HashSet<String>,
}

impl Genome {
    #[allow(dead_code)]
    pub fn new(genome: &str, seed: u64) -> Result<Self, Box<dyn Error>> {
        let parsed = Self::parse(genome)?;
        let rng = StdRng::seed_from_u64(seed);
        Ok(Self { genome: parsed, rng: Arc::new(Mutex::new(rng)), states: HashSet::<String>::new() })
    }

    pub fn sha256(&self) -> Option<String> {
        SHA256::new(self.to_string().as_bytes()).hexdigest()
    }

    #[allow(dead_code)]
    pub fn save(&mut self) {
        if let Some(sha256) = self.sha256() {
            self.states.insert(sha256);
        }
    }

    pub fn is_previous_state(&mut self) -> bool {
        let sha256 = self.sha256();
        if sha256.is_none() { return false }
        self.states.contains(&sha256.unwrap())
    }

    #[allow(dead_code)]
    fn parse(genome: &str) -> Result<Vec<AllelePair>, Box<dyn Error>> {
        if genome.len() % 2 != 0 {
            return Err("genome length must be even".into());
        }
        let mut parsed = Vec::new();
        let chars: Vec<char> = genome.chars().collect();
        for chunk in chars.chunks(2) {
            let high = Self::parse_gene(chunk[0])?;
            let low = Self::parse_gene(chunk[1])?;

            parsed.push(AllelePair { high, low });
        }

        Ok(parsed)
    }

    #[allow(dead_code)]
    pub fn number_of_mutations(&self) -> usize {
        self.states.len()
    }

    #[allow(dead_code)]
    pub fn is_permissive(&mut self) -> bool {
        if self.is_previous_state() {
            return false;
        } else { self.save(); }
        if self.wildcard_ratio() > 0.5 {
            self.save();
            return false;
        }
        return true;
    }

    #[allow(dead_code)]
    pub fn wildcard_ratio(&self) -> f64 {
        let total_genes = self.genome.len() * 2;
        if total_genes == 0 {
            return 0.0;
        }
        let wildcard_count = self.genome.iter().fold(0, |acc, pair| {
            acc + match pair.high {
                Gene::Wildcard => 1,
                _ => 0,
            } + match pair.low {
                Gene::Wildcard => 1,
                _ => 0,
            }
        });
        wildcard_count as f64 / total_genes as f64
    }

    #[allow(dead_code)]
    fn parse_gene(c: char) -> Result<Gene, Box<dyn Error>> {
        match c {
            '?' => Ok(Gene::Wildcard),
            _ if c.is_ascii_hexdigit() => {
                let value = u8::from_str_radix(&c.to_string(), 16)?;
                Ok(Gene::Value(value))
            }
            _ => Err(format!("invalid character in genome: {}", c).into()),
        }
    }

    #[allow(dead_code)]
    pub fn create_mutated_gene(&mut self) -> u8 {
        self.rng.lock().unwrap().gen_range(0..=15) as u8
    }

    #[allow(dead_code)]
    pub fn mutate_add_gene(&mut self) {
        let new_allele = AllelePair {
            high: Gene::Value(self.create_mutated_gene()),
            low: Gene::Value(self.create_mutated_gene()),
        };
        if self.rng.lock().unwrap().gen_bool(0.5) {
            self.genome.insert(0, new_allele);
        } else {
            self.genome.push(new_allele);
        }
    }

    #[allow(dead_code)]
    pub fn mutate_wildcard(&mut self) {
        let genome_len = self.genome.len();
        if genome_len <= 2 {
            return;
        }
        let index = self.rng.lock().unwrap().gen_range(1..(genome_len - 1));
        if self.rng.lock().unwrap().gen_bool(0.5) {
            self.genome[index].high = Gene::Wildcard;
        } else {
            self.genome[index].low = Gene::Wildcard;
        }
    }

    #[allow(dead_code)]
    pub fn matches_buffer(&self, data: &[u8]) -> bool {
        if self.genome.len() > data.len() {
            return false;
        }
        for start in 0..=(data.len() - self.genome.len()) {
            let mut match_found = true;
            for (i, byte_pattern) in self.genome.iter().enumerate() {
                let data_byte = data[start + i];

                if !Self::matches_byte(byte_pattern, data_byte) {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn matches_file(&self, path: PathBuf) -> Result<bool, IoError> {
        let mapped_file = MemoryMappedFile::new_readonly(path)?;
        let data = mapped_file.mmap()?;
        Ok(self.matches_buffer(&data))
    }

    #[allow(dead_code)]
    fn matches_byte(pattern: &AllelePair, data_byte: u8) -> bool {
        let high_nibble = (data_byte >> 4) & 0x0F;
        let low_nibble = data_byte & 0x0F;
        (match pattern.high {
            Gene::Wildcard => true,
            Gene::Value(v) => v == high_nibble,
        }) && (match pattern.low {
            Gene::Wildcard => true,
            Gene::Value(v) => v == low_nibble,
        })
    }
}

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut genome_string = String::new();
        for allele_pair in &self.genome {
            genome_string.push_str(&allele_pair.high.to_char());
            genome_string.push_str(&allele_pair.low.to_char());
        }
        write!(f, "{}", genome_string)
    }
}
