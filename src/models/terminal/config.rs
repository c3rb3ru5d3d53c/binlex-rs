use dirs;
use std::{fs, path::PathBuf};
use std::io::Error;
use std::io::ErrorKind;
use std::env;
use serde::{Deserialize, Serialize};
use serde;

pub const DIRECTORY: &str = "binlex";
pub const FILE_NAME: &str = "binlex.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub general: ConfigGeneral,
    pub heuristics: ConfigHeuristics,
    pub hashing: ConfigHashing,
    pub file_mapping: ConfigFileMapping,
    pub disassembler: ConfigDisassembler,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigDisassembler {
    pub sweep: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigHeuristics {
    pub features: bool,
    pub normalization: bool,
    pub entropy: bool,

}

#[derive(Serialize, Deserialize)]
pub struct ConfigHashing {
    pub sha256: ConfigSHA256,
    pub tlsh: ConfigTLSH,
    pub minhash: ConfigMinhash,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct ConfigFileMapping {
    pub enable: bool,
    pub directory: String,
    pub caching: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigMinhash {
    pub enable: bool,
    pub number_of_hashes: usize,
    pub shingle_size: usize,
    pub maximum_byte_size: usize,
    pub seed: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigTLSH {
    pub enable: bool,
    pub minimum_byte_size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigSHA256 {
    pub enable: bool,
}

impl Config {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Config {
            general: ConfigGeneral{
                input: None,
                output: None,
                threads: 1,
                minimal: false,
                debug: false,
                tags: Vec::<String>::new(),
            },
            heuristics: ConfigHeuristics {
                features: true,
                normalization: false,
                entropy: true,
            },
            hashing: ConfigHashing {
                sha256: ConfigSHA256 {
                    enable: true,
                },
                tlsh: ConfigTLSH {
                    enable: true,
                    minimum_byte_size: 50,
                },
                minhash: ConfigMinhash {
                    enable: true,
                    number_of_hashes: 64,
                    shingle_size: 4,
                    maximum_byte_size: 50,
                    seed: 0,
                },
            },
            file_mapping: ConfigFileMapping {
                enable: false,
                directory: Config::default_file_mapping_directory(),
                caching: false,
            },
            disassembler: ConfigDisassembler {
                sweep: true,
            }
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
    pub fn to_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    pub fn from_file(file_path: &str) -> Result<Config, Error> {
        let toml_string = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&toml_string).expect("failed to deserialize binlex configuration file");
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
