use std::error::Error;

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
    pattern: Vec<AllelePair>,
}

impl Genome {
    #[allow(dead_code)]
    pub fn new(genome: &str) -> Result<Self, Box<dyn Error>> {
        let parsed = Self::parse(genome)?;
        Ok(Self { pattern: parsed })
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
    pub fn matches(&self, data: &[u8]) -> bool {
        if self.pattern.len() > data.len() {
            return false;
        }
        for start in 0..=(data.len() - self.pattern.len()) {
            let mut match_found = true;
            for (i, byte_pattern) in self.pattern.iter().enumerate() {
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
