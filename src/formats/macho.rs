use lief::Binary;
use std::io::{Cursor, Error, ErrorKind};
use crate::Architecture;
use crate::formats::File;
use crate::Config;
use lief::macho::header::CpuType as MachoCpuType;

pub struct MACHO {
    pub macho: lief::macho::FatBinary,
    pub file: File,
    pub config: Config,
}

impl MACHO {
    /// Creates a new `MACHO` instance by reading a ELF file from the provided path.
    ///
    /// # Parameters
    /// - `path`: The file path to the MACHO file to be loaded.
    ///
    /// # Returns
    /// A `Result` containing the `MACHO` object on success or an `Error` on failure.
    pub fn new(path: String, config: Config) -> Result<Self, Error> {
        let mut file = File::new(path.clone(), config.clone())?;
        match file.read() {
            Ok(_) => (),
            Err(_) => {
                return Err(Error::new(ErrorKind::InvalidInput, "failed to read macho file"));
            }
        };
        let binary = Binary::parse(&path);
        if let Some(Binary::MachO(macho)) = binary {
            return Ok(Self {
                macho: macho,
                file: file,
                config: config,
            });
        }
        return Err(Error::new(ErrorKind::InvalidInput, "invalid macho file"));
    }

    /// Creates a new `MACHO` instance from a byte vector containing MACHO file data.
    ///
    /// # Parameters
    /// - `bytes`: A vector of bytes representing the PE file data.
    ///
    /// # Returns
    /// A `Result` containing the `MACHO` object on success or an `Error` on failure.
    #[allow(dead_code)]
    pub fn from_bytes(bytes: Vec<u8>, config: Config) -> Result<Self, Error> {
        let file = File::from_bytes(bytes, config.clone());
        let mut cursor = Cursor::new(&file.data);
        if let Some(Binary::MachO(macho)) = Binary::from(&mut cursor) {
            return Ok(Self{
                macho: macho,
                file: file,
                config: config,
            })
        }
        return Err(Error::new(ErrorKind::InvalidInput, "invalid macho file"));
    }

    pub fn number_of_binaries(&self) -> usize {
        self.macho.iter().count()
    }

    pub fn entrypoint(&self, index: usize) -> Option<u64> {
        Some(self.macho.iter().nth(index)?.main_command()?.entrypoint())
    }

    pub fn architecture(&self, index: usize) -> Option<Architecture> {
        let cpu_type = self.macho.iter().nth(index).map(|b|b.header().cpu_type());
        if cpu_type.is_none() { return None; }
        let architecture = match cpu_type.unwrap() {
            MachoCpuType::X86 => Architecture::I386,
            MachoCpuType::X86_64 => Architecture::AMD64,
            _ => { return None; },
        };
        Some(architecture)
    }

}
