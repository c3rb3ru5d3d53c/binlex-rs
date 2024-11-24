use dirs;
use std::{fs, path::PathBuf};
use std::io::Error;
use std::io::ErrorKind;
use std::env;
use serde::{Deserialize, Serialize};
use serde;
use std::str::FromStr;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "@c3rb3ru5d3d53c";
pub const DIRECTORY: &str = "binlex";
pub const FILE_NAME: &str = "binlex.toml";

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

/// Represents the different architectures of a binary.
#[repr(u16)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Architecture {
    /// 64-bit AMD architecture.
    AMD64 = 0x00,
    /// 32-bit Intel architecture.
    I386 = 0x01,
    /// Unknown architecture.
    UNKNOWN= 0x03,
}

/// Implements Display for `BinaryArchitecture` enum
impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let architecture = match self {
            Architecture::AMD64 => "amd64",
            Architecture::I386 => "i386",
            Architecture::UNKNOWN => "unknown",
        };
        write!(f, "{}", architecture)
    }
}

impl FromStr for Architecture {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amd64" => Ok(Architecture::AMD64),
            "i386" => Ok(Architecture::I386),
            _ => Err(format!("invalid architecutre")),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigBlocks {
    pub hashing: ConfigHashing,
    pub heuristics: ConfigHeuristics,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigSignatures {
    pub hashing: ConfigHashing,
    pub heuristics: ConfigHeuristics,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFunctions {
    pub hashing: ConfigHashing,
    pub heuristics: ConfigHeuristics,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    pub hashing: ConfigHashing,
    pub heuristics: ConfigHeuristics,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFormats {
    pub file: ConfigFile,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: ConfigGeneral,
    pub formats: ConfigFormats,
    pub blocks: ConfigBlocks,
    pub functions: ConfigFunctions,
    pub signatures: ConfigSignatures,
    pub mmap: ConfigMmap,
    pub disassembler: ConfigDisassembler,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigDisassembler {
    pub sweep: ConfigDisassemblerSweep,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigDisassemblerSweep {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigHeuristics {
    pub features: ConfigHeuristicFeatures,
    pub normalized: ConfigHeuristicNormalization,
    pub entropy: ConfigHeuristicEntropy,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigHeuristicFeatures {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigHeuristicNormalization {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigHeuristicEntropy {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigHashing {
    pub sha256: ConfigSHA256,
    pub tlsh: ConfigTLSH,
    pub minhash: ConfigMinhash,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFileHashes {
    pub sha256: ConfigSHA256,
    pub tlsh: ConfigTLSH,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigGeneral {
    pub threads: usize,
    pub minimal: bool,
    pub debug: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigMmap {
    pub directory: String,
    pub cache: ConfigMmapCache,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigMmapCache {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigMinhash {
    pub enabled: bool,
    pub number_of_hashes: usize,
    pub shingle_size: usize,
    pub maximum_byte_size: usize,
    pub seed: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigTLSH {
    pub enabled: bool,
    pub minimum_byte_size: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigSHA256 {
    pub enabled: bool,
}

impl Config {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Config {
            general: ConfigGeneral {
                threads: 1,
                minimal: false,
                debug: false,
            },
            formats: ConfigFormats {
                file: ConfigFile {
                    hashing: ConfigHashing {
                        sha256: ConfigSHA256 {
                            enabled: true,
                        },
                        tlsh: ConfigTLSH {
                            enabled: true,
                            minimum_byte_size: 50,
                        },
                        minhash: ConfigMinhash {
                            enabled: true,
                            number_of_hashes: 64,
                            shingle_size: 4,
                            maximum_byte_size: 50,
                            seed: 0,
                        }
                    },
                    heuristics: ConfigHeuristics {
                        features: ConfigHeuristicFeatures {
                            enabled: true,
                        },
                        normalized: ConfigHeuristicNormalization {
                            enabled: false,
                        },
                        entropy: ConfigHeuristicEntropy {
                            enabled: true,
                        }
                    }
                }
            },
            blocks: ConfigBlocks {
                hashing: ConfigHashing {
                    sha256: ConfigSHA256 {
                        enabled: true,
                    },
                    tlsh: ConfigTLSH {
                        enabled: true,
                        minimum_byte_size: 50,
                    },
                    minhash: ConfigMinhash {
                        enabled: true,
                        number_of_hashes: 64,
                        shingle_size: 4,
                        maximum_byte_size: 50,
                        seed: 0,
                    }
                },
                heuristics: ConfigHeuristics {
                    features: ConfigHeuristicFeatures {
                        enabled: true,
                    },
                    normalized: ConfigHeuristicNormalization {
                        enabled: false,
                    },
                    entropy: ConfigHeuristicEntropy {
                        enabled: true,
                    }
                }
            },
            functions: ConfigFunctions {
                hashing: ConfigHashing {
                    sha256: ConfigSHA256 {
                        enabled: true,
                    },
                    tlsh: ConfigTLSH {
                        enabled: true,
                        minimum_byte_size: 50,
                    },
                    minhash: ConfigMinhash {
                        enabled: true,
                        number_of_hashes: 64,
                        shingle_size: 4,
                        maximum_byte_size: 50,
                        seed: 0,
                    }
                },
                heuristics: ConfigHeuristics {
                    features: ConfigHeuristicFeatures {
                        enabled: true,
                    },
                    normalized: ConfigHeuristicNormalization {
                        enabled: false,
                    },
                    entropy: ConfigHeuristicEntropy {
                        enabled: true,
                    }
                }
            },
            signatures: ConfigSignatures {
                hashing: ConfigHashing {
                    sha256: ConfigSHA256 {
                        enabled: true,
                    },
                    tlsh: ConfigTLSH {
                        enabled: true,
                        minimum_byte_size: 50,
                    },
                    minhash: ConfigMinhash {
                        enabled: true,
                        number_of_hashes: 64,
                        shingle_size: 4,
                        maximum_byte_size: 50,
                        seed: 0,
                    }
                },
                heuristics: ConfigHeuristics {
                    features: ConfigHeuristicFeatures {
                        enabled: true,
                    },
                    normalized: ConfigHeuristicNormalization {
                        enabled: false,
                    },
                    entropy: ConfigHeuristicEntropy {
                        enabled: true,
                    }
                }
            },
            mmap: ConfigMmap {
                directory: Config::default_file_mapping_directory(),
                cache: ConfigMmapCache {
                    enabled: false,
                }
            },
            disassembler: ConfigDisassembler {
                sweep: ConfigDisassemblerSweep {
                    enabled: true,
                }
            }
        }
    }

    pub fn enable_minimal(&mut self) {
        self.general.minimal = true;
        self.disable_heuristics();
        self.disable_hashing();
    }

    pub fn disable_hashing(&mut self) {
        self.disable_block_hashing();
        self.disable_function_hashing();
        self.disable_signature_hashing();
        self.disable_file_hashing();
    }

    pub fn disable_signature_heuristics(&mut self) {
        self.signatures.heuristics.entropy.enabled = false;
        self.signatures.heuristics.features.enabled = false;
        self.signatures.heuristics.normalized.enabled = false;
    }

    pub fn disable_block_hashing(&mut self){
        self.blocks.hashing.sha256.enabled = false;
        self.blocks.hashing.tlsh.enabled = false;
        self.blocks.hashing.minhash.enabled = false;
    }

    pub fn disable_file_hashing(&mut self) {
        self.formats.file.hashing.sha256.enabled = false;
        self.formats.file.hashing.tlsh.enabled = false;
        self.formats.file.hashing.minhash.enabled = false;
    }

    pub fn disable_file_heuristics(&mut self) {
        self.formats.file.heuristics.entropy.enabled = false;
        self.formats.file.heuristics.features.enabled = false;
        self.formats.file.heuristics.normalized.enabled = false;
    }

    pub fn disable_heuristics(&mut self) {
        self.disable_block_heuristics();
        self.disable_function_heuristics();
        self.disable_signature_heuristics();
        self.disable_file_heuristics();
    }

    pub fn disable_signature_hashing(&mut self) {
        self.signatures.hashing.sha256.enabled = false;
        self.signatures.hashing.tlsh.enabled = false;
        self.signatures.hashing.minhash.enabled = false;
    }

    pub fn disable_function_hashing(&mut self) {
        self.functions.hashing.sha256.enabled = false;
        self.functions.hashing.tlsh.enabled = false;
        self.functions.hashing.minhash.enabled = false;
    }

    pub fn disable_block_heuristics(&mut self) {
        self.blocks.heuristics.entropy.enabled = false;
        self.blocks.heuristics.features.enabled = false;
        self.blocks.heuristics.normalized.enabled = false;
    }

    pub fn disable_function_heuristics(&mut self) {
        self.functions.heuristics.entropy.enabled = false;
        self.functions.heuristics.features.enabled = false;
        self.functions.heuristics.normalized.enabled = false;
    }

    // Get Default File Mapping Directory
    #[allow(dead_code)]
    pub fn default_file_mapping_directory() -> String {
        env::temp_dir()
            .join(DIRECTORY)
            .to_str()
            .expect("failed to convert file mapping directory to string")
            .to_owned()
    }

    /// Prints the Current Configuration
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.to_string().unwrap());
    }

    /// Convert Config to a TOML String
    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String, Error> {
        toml::to_string_pretty(self).map_err(|e| Error::new(ErrorKind::Other, e))
    }

    /// Reads the Configuration TOML from a File Path
    pub fn from_file(file_path: &str) -> Result<Config, Error> {
        let toml_string = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&toml_string)
            .map_err(|error| Error::new(ErrorKind::InvalidData, format!("failed to read configuration file {}\n\n{}", file_path, error)))?;
        Ok(config)
    }

    /// Write the configuration TOML to a file
    #[allow(dead_code)]
    pub fn write_to_file(&self, file_path: &str) -> Result<(), Error> {
        let toml_string = self.to_string()
            .expect("failed to serialize binlex configration to toml format");
        fs::write(file_path, toml_string)?;
        Ok(())
    }

    /// Writes Default TOML Configuration File To Configuration Directory
    #[allow(dead_code)]
    pub fn write_default(&self) -> Result<(), Error> {
        if let Some(config_directory) = dirs::config_dir() {
            let config_file_path: PathBuf = config_directory.join(format!("{}/{}", DIRECTORY, FILE_NAME));
            if let Some(parent_directory) = config_file_path.parent() {
                if !parent_directory.exists() {
                    fs::create_dir_all(parent_directory).expect("failed to create binlex configuration directory");
                }
            }
            if !config_file_path.exists() {
                return self.write_to_file(config_file_path.to_str().unwrap());
            }
        }
        return Err(Error::new(ErrorKind::Other, format!("default configuration already exists")));
    }

    /// Reads the default TOML Configuration File
    #[allow(dead_code)]
    pub fn from_default(&mut self) -> Result<(), Error> {
        if let Some(config_directory) = dirs::config_dir() {
            let config_file_path: PathBuf = config_directory.join(format!("{}/{}", DIRECTORY, FILE_NAME));
            if config_file_path.exists() {
                match Config::from_file(config_file_path.to_str().unwrap()) {
                    Ok(config) => return {
                        *self = config;
                        Ok(())
                    },
                    Err(error) => return Err(error),
                }
            }
        }
        return Err(Error::new(ErrorKind::Other, format!("unable to read binlex default configuration file")));
    }

}
