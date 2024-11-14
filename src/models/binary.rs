use std::collections::HashMap;
pub struct Binary;

#[repr(u16)]
#[derive(Copy, Clone, PartialEq)]
pub enum BinaryArchitecture {
    AMD64 = 0x00,
    I386 = 0x01,
    UNKNOWN= 0x03,
}

impl Binary {

    pub fn entropy(bytes: &Vec<u8>) -> Option<f64> {
        let mut frequency: HashMap<u8, usize> = HashMap::new();
        for &byte in bytes {
            *frequency.entry(byte).or_insert(0) += 1;
        }

        let data_len = bytes.len() as f64;
        if data_len == 0.0 {
            return None;
        }

        let entropy = frequency.values().fold(0.0, |entropy, &count| {
            let probability = count as f64 / data_len;
            entropy - probability * probability.log2()
        });

        Some(entropy)
    }

    pub fn to_hex(data: &[u8]) -> String {
        data.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    #[allow(dead_code)]
    pub fn hexdump(data: &[u8], address: u64) -> String {
        const BYTES_PER_LINE: usize = 16;
        let mut result = String::new();
        for (i, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
            let current_address = address as usize + i * BYTES_PER_LINE;
            let hex_repr = format!("{:08x}: ", current_address);
            result.push_str(&hex_repr);
            let hex_values: String = chunk.iter().map(|byte| format!("{:02x} ", byte)).collect();
            result.push_str(&hex_values);
            let padding = "   ".repeat(BYTES_PER_LINE - chunk.len());
            result.push_str(&padding);
            let ascii_values: String = chunk
                .iter()
                .map(|&byte| if byte.is_ascii_graphic() || byte == b' ' { byte as char } else { '.' })
                .collect();
            result.push('|');
            result.push_str(&ascii_values);
            result.push_str("|\n");
        }

        result
    }


}
