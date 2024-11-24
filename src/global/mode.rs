use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;
use std::fmt;
use crate::global::Format;
use crate::global::Architecture;

#[derive(Clone, Debug)]
pub enum Modes {
    CodeI386,
    CodeAmd64,
}

impl Modes {
    pub fn list() -> String {
        Modes::to_vec().join(", ")
    }
}

impl Modes {
    pub fn to_vec() -> Vec<String> {
        vec![
            Modes::CodeI386.to_string(),
            Modes::CodeAmd64.to_string(),
        ]
    }
}

impl fmt::Display for Modes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Modes::CodeI386 => "code:i386",
                Modes::CodeAmd64 => "code:amd64",
            }
        )
    }
}

impl std::str::FromStr for Modes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "code:i386" => Ok(Modes::CodeI386),
            "code:amd64" => Ok(Modes::CodeAmd64),
            _ => Err(format!("invalid mode")),
        }
    }
}


pub struct Mode {
    format: Format,
    architecture: Architecture,
}

impl Mode {
    pub fn new(mode: String) -> Result<Self, Error> {
        if !Modes::to_vec().contains(&mode) {
            return Err(Error::new(ErrorKind::InvalidInput, "unsupported or invalid mode"));
        }
        if let Some((format, architecture)) = mode.split_once(":") {
            let format = Format::from_str(format)
                .map_err(|_| ErrorKind::InvalidInput)?;
            let architecture = Architecture::from_str(architecture)
                .map_err(|_| ErrorKind::InvalidInput)?;
            return Ok(Self{
                format: format,
                architecture: architecture,
            });
        }
        return Err(Error::new(ErrorKind::InvalidInput, "unsupported or invalid mode"));
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn architecture(&self) -> Architecture {
        self.architecture
    }
}
