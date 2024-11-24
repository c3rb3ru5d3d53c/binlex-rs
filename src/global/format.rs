
use std::io::Error;
use std::str::FromStr;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[repr(u16)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Format {
    /// Raw File
    CODE = 0x00,
    /// Portable Executable
    PE = 0x01,
    /// Unknown formats
    UNKNOWN = 0x02,
}

impl Format {
    pub fn from_file(path: String) -> Result<Format, Error> {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 2];
        file.seek(SeekFrom::Start(0x00))?;
        file.read_exact(&mut buffer)?;
        if buffer == [0x4d, 0x5a] {
            file.seek(SeekFrom::Start(0x3c))?;
            let mut pe_offset = [0u8; 4];
            file.read_exact(&mut pe_offset)?;
            let pe_offset = u32::from_le_bytes(pe_offset);
            file.seek(SeekFrom::Start(pe_offset as u64))?;
            let mut pe_signature = [0u8; 4];
            file.read_exact(&mut pe_signature)?;
            if pe_signature == [0x50, 0x45, 0x00, 0x00] {
                return Ok(Format::PE);
            }
        }
        return Ok(Format::UNKNOWN);
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format: &str = match self {
            Format::CODE => "code",
            Format::PE => "pe",
            Format::UNKNOWN => "unknown",
        };
        write!(f, "{}", format)
    }
}

impl FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "code" => Ok(Format::CODE),
            "pe" => Ok(Format::PE),
            "unknown" => Ok(Format::UNKNOWN),
            _ => Err(format!("invalid format")),
        }
    }
}
