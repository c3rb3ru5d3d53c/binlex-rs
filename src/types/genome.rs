use std::error::Error;
use std::path::PathBuf;
use crate::types::MemoryMappedFile;
use std::io::Error as IoError;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Gene {
    Wildcard,
    Value(u8),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct AllelePair {
    high: Gene,
    low: Gene,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Genome {
    genome: Vec<AllelePair>,
    rng: ThreadRng,
}

impl Genome {
    #[allow(dead_code)]
    pub fn new(genome: &str) -> Result<Self, Box<dyn Error>> {
        let parsed = Self::parse(genome)?;
        let rng = rand::thread_rng();
        Ok(Self { genome: parsed, rng: rng})
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
    fn gene_to_char(gene: Gene) -> String {
        match gene {
            Gene::Wildcard => "?".to_string(),
            Gene::Value(v) => format!("{:X}", v),
        }
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
        self.rng.gen_range(0..=15) as u8
    }

    #[allow(dead_code)]
    pub fn mutate_add_gene(&mut self) {
        let new_allele = AllelePair {
            high: if self.rng.gen_bool(0.5) {
                Gene::Wildcard
            } else {
                Gene::Value(self.create_mutated_gene())
            },
            low: if self.rng.gen_bool(0.5) {
                Gene::Wildcard
            } else {
                Gene::Value(self.create_mutated_gene())
            },
        };

        if self.rng.gen_bool(0.5) {
            self.genome.insert(0, new_allele);
        } else {
            self.genome.push(new_allele);
        }
    }

    #[allow(dead_code)]
    pub fn mutate_replace_with_wildcards(&mut self) {
        if self.genome.is_empty() { return; }
        let index = self.rng.gen_range(0..self.genome.len());
        let allele = &mut self.genome[index];
        if self.rng.gen_bool(0.5) {
            allele.high = Gene::Wildcard;
        } else {
            allele.low = Gene::Wildcard;
        }
    }


    #[allow(dead_code)]
    pub fn mutate_wildcards(&mut self) {
        let genome_len = self.genome.len();
        if genome_len == 0 { return; }
        let index = self.rng.gen_range(0..genome_len);
        if self.rng.gen_bool(0.5) {
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
            genome_string.push_str(&Self::gene_to_char(allele_pair.high));
            genome_string.push_str(&Self::gene_to_char(allele_pair.low));
        }
        write!(f, "{}", genome_string)
    }
}
