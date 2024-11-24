use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;
use crate::global::Format;
use crate::global::Architecture;

pub struct Mode {
    format: Format,
    architecture: Architecture,
}

impl Mode {
    pub fn new(mode: String) -> Result<Self, Error> {
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
        return Err(Error::new(ErrorKind::Other, "invalid mode"));
    }

    pub fn format(&self) -> Format {
        self.format
    }

    pub fn architecture(&self) -> Architecture {
        self.architecture
    }
}
