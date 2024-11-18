use dirs;
use std::{fs, path::PathBuf};
use std::io::Error;
use std::io::ErrorKind;
use std::env;
use serde::{Deserialize, Serialize};
use serde;

pub const DIRECTORY: &str = "binlex";
pub const FILE_NAME: &str = "binlex.toml";

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: ConfigGeneral,
    pub heuristics: ConfigHeuristics,
    pub hashing: ConfigHashing,
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
    pub normalization: ConfigHeuristicNormalization,
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
    pub file: ConfigFileHashes,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFileHashes {
    pub enabled: bool,
    #[serde(skip)]
    pub sha256: Option<String>,
    #[serde(skip)]
    pub tlsh: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigGeneral {
    #[serde(skip)]
    pub input: Option<String>,
    #[serde(skip)]
    pub output: Option<String>,
    pub threads: usize,
    pub minimal: bool,
    pub debug: bool,
    #[serde(skip)]
    pub tags: Vec<String>,
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
                input: None,
                output: None,
                threads: 1,
                minimal: false,
                debug: false,
                tags: Vec::<String>::new(),
            },
            heuristics: ConfigHeuristics {
                features: ConfigHeuristicFeatures {
                    enabled: true,
                },
                normalization: ConfigHeuristicNormalization {
                    enabled: false,
                },
                entropy: ConfigHeuristicEntropy {
                    enabled: true,
                },
            },
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
                },
                file: ConfigFileHashes {
                    enabled: true,
                    sha256: None,
                    tlsh: None,
                }
            },
            mmap: ConfigMmap {
                directory: Config::default_file_mapping_directory(),
                cache: ConfigMmapCache {
                    enabled: false,
                },
            },
            disassembler: ConfigDisassembler {
                sweep: ConfigDisassemblerSweep {
                    enabled: true,
                },
            },
        }
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
