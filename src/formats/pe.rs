//use lief::generic::Binary;
use lief::Binary;
use lief::pe::section::Characteristics;
use std::io::{Cursor, Error, ErrorKind};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::path::PathBuf;
use lief::pe::headers::MachineType;
use crate::models::binary::BinaryArchitecture;
use crate::formats::file::File;
use std::collections::BTreeMap;
use lief::pe::debug::Entries;
use crate::types::cachedfile::CachedFile;

/// Represents a PE (Portable Executable) file, encapsulating the `lief::pe::Binary` and associated metadata.
pub struct PE {
    pub pe: lief::pe::Binary,
    pub file: File,
}

impl PE {
    /// Creates a new `PE` instance by reading a PE file from the provided path.
    ///
    /// # Parameters
    /// - `path`: The file path to the PE file to be loaded.
    ///
    /// # Returns
    /// A `Result` containing the `PE` object on success or an `Error` on failure.
    pub fn new(path: String) -> Result<Self, Error> {
        let mut file = File::new(path.clone());
        match file.read() {
            Ok(_) => (),
            Err(_) => {
                return Err(Error::new(ErrorKind::InvalidInput, "failed to read file"));
            }
        };
        if let Some(Binary::PE(pe)) = Binary::parse(&path) {
            return Ok(Self {
                pe: pe,
                file: file,
            });
        }
        return Err(Error::new(ErrorKind::InvalidInput, "invalid pe file"));
    }

    /// Creates a new `PE` instance from a byte vector containing PE file data.
    ///
    /// # Parameters
    /// - `bytes`: A vector of bytes representing the PE file data.
    ///
    /// # Returns
    /// A `Result` containing the `PE` object on success or an `Error` on failure.
    #[allow(dead_code)]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        let file = File::from_bytes(bytes);
        let mut cursor = Cursor::new(&file.data);
        if let Some(Binary::PE(pe)) = Binary::from(&mut cursor) {
            return Ok(Self{
                pe: pe,
                file: file
            })
        }
        return Err(Error::new(ErrorKind::InvalidInput, "invalid pe file"));
    }

    /// Returns the architecture of the PE file based on its machine type.
    ///
    /// # Returns
    /// The `BinaryArchitecture` enum value corresponding to the PE machine type (e.g., AMD64, I386, or UNKNOWN).
    #[allow(dead_code)]
    pub fn architecture(&self) -> BinaryArchitecture {
        let machine = match self.pe.header().machine() {
            MachineType::AMD64 => BinaryArchitecture::AMD64,
            MachineType::I386 => BinaryArchitecture::I386,
            _ => BinaryArchitecture::UNKNOWN,
        };
        return machine;
    }

    /// Returns the ranges of executable memory addresses within the PE file.
    ///
    /// This includes sections marked as executable (`MEM_EXECUTE`) and with valid data.
    ///
    /// # Returns
    /// A `BTreeMap` where the key is the start address of the executable range and the value is the end address.
    #[allow(dead_code)]
    pub fn executable_virtual_address_ranges(&self) -> BTreeMap<u64, u64> {
        let mut result = BTreeMap::<u64, u64>::new();
        for section in self.pe.sections() {
            if (section.characteristics().bits() & u64::from(Characteristics::MEM_EXECUTE)) == 0 { continue; }
            if section.virtual_size() == 0 { continue; }
            if section.sizeof_raw_data() == 0 { continue; }
            let section_virtual_adddress = PE::align_section_virtual_address(
                self.imagebase() + section.pointerto_raw_data() as u64,
                self.section_alignment(),
                self.file_alignment());
            result.insert(
                section_virtual_adddress,
                section_virtual_adddress + section.virtual_size() as u64);
        }
        return result;
    }

    /// Returns a map of Pogo (debug) entries found in the PE file, keyed by their start RVA (Relative Virtual Address).
    ///
    /// # Returns
    /// A `HashMap` where the key is the RVA of the start of the Pogo entry and the value is the name of the entry.
    #[allow(dead_code)]
    pub fn pogos(&self) -> HashMap<u64, String> {
        let mut result = HashMap::<u64, String>::new();
        for entry in self.pe.debug() {
            match entry {
                Entries::Pogo(pogos) => {
                    for pogo in pogos.entries() {
                        result.insert(self.imagebase() + pogo.start_rva() as u64, pogo.name());
                    }
                },
                _ => {}
            }

        }
        result
    }

    /// Returns a set of TLS (Thread Local Storage) callback addresses in the PE file.
    ///
    /// The method retrieves the TLS callbacks from the PE file's TLS data directory, if present.
    /// TLS callbacks are functions that are called when a thread is created or terminated, and they
    /// are often used in applications to initialize or clean up thread-local data.
    ///
    /// # Returns
    /// A `BTreeSet<u64>` containing the addresses of the TLS callback functions.
    pub fn tlscallbacks(&self) -> BTreeSet<u64> {
        self.pe.tls()
            .into_iter()
            .flat_map(|tls| tls.callbacks())
            .collect()
    }

    /// Returns a set of function addresses (entry point, exports, TLS callbacks, and Pogo entries) in the PE file.
    ///
    /// # Returns
    /// A `BTreeSet` of function addresses in the PE file.
    #[allow(dead_code)]
    pub fn functions(&self) -> BTreeSet<u64> {
        let mut addresses = BTreeSet::<u64>::new();
        addresses.insert(self.entrypoint());
        addresses.extend(self.exports());
        addresses.extend(self.tlscallbacks());
        addresses.extend(self.pogos().keys().cloned());
        return addresses;
    }

    /// Returns the entry point address of the PE file.
    ///
    /// # Returns
    /// The entry point address as a `u64` value.
    #[allow(dead_code)]
    pub fn entrypoint(&self) -> u64 {
        self.imagebase() + self.pe.optional_header().addressof_entrypoint() as u64
    }

    /// Returns the size of the headers of the PE file.
    ///
    /// # Returns
    /// The size of the headers as a `u64` value.
    #[allow(dead_code)]
    pub fn sizeofheaders(&self) -> u64 {
        self.pe.optional_header().sizeof_headers() as u64
    }

    /// Aligns a section's virtual address to the specified section and file alignment boundaries.
    ///
    /// # Parameters
    /// - `value`: The virtual address to align.
    /// - `section_alignment`: The section alignment boundary.
    /// - `file_alignment`: The file alignment boundary.
    ///
    /// # Returns
    /// The aligned virtual address.
    #[allow(dead_code)]
    pub fn align_section_virtual_address(value: u64, mut section_alignment: u64, file_alignment: u64) -> u64 {
        if section_alignment < 0x1000 {
            section_alignment = file_alignment;
        }
        if section_alignment != 0 && (value % section_alignment) != 0 {
            return section_alignment * ((value + section_alignment - 1) / section_alignment);
        }
        return value;
    }

    /// Returns the section alignment used in the PE file.
    ///
    /// # Returns
    /// The section alignment value as a `u64`.
    #[allow(dead_code)]
    pub fn section_alignment(&self) -> u64 {
        self.pe.optional_header().section_alignment() as u64
    }

    /// Returns the file alignment used in the PE file.
    ///
    /// # Returns
    /// The file alignment value as a `u64`.
    #[allow(dead_code)]
    pub fn file_alignment(&self) -> u64 {
        self.pe.optional_header().file_alignment() as u64
    }

    /// Converts a relative virtual address to a virtual address
    ///
    /// # Returns
    /// The virtual address as a `u64`.
    #[allow(dead_code)]
    pub fn relative_virtual_address_to_virtual_address(&self, relative_virtual_address: u64) -> u64 {
        self.imagebase() + relative_virtual_address
    }

    /// Converts a file offset to a virtual address.
    ///
    /// This method looks through the PE file's sections to determine which section contains the file offset.
    /// It then computes the corresponding virtual address within that section.
    ///
    /// # Parameters
    /// - `file_offset`: The file offset (raw data offset) to convert to a virtual address.
    ///
    /// # Returns
    /// The corresponding virtual address as a `u64`.
    #[allow(dead_code)]
    pub fn file_offset_to_virtual_address(&self, file_offset: u64) -> Option<u64> {
        for section in self.pe.sections() {
            let section_raw_data_offset = section.pointerto_raw_data() as u64;
            let section_raw_data_size = section.sizeof_raw_data() as u64;
            if file_offset >= section_raw_data_offset && file_offset < section_raw_data_offset + section_raw_data_size {
                let section_virtual_address = self.imagebase() + section.pointerto_raw_data() as u64;
                let section_offset = file_offset - section_raw_data_offset;
                let virtual_address = section_virtual_address + section_offset;
                return Some(virtual_address);
            }
        }
        None
    }

    /// Caches the PE file contents and returns a `CachedFile` object.
    ///
    /// # Parameters
    /// - `path`: The base path to store the cached file.
    /// - `cache`: Whether to cache the file or not.
    ///
    /// # Returns
    /// A `Result` containing the `CachedFile` object on success or an `Error` on failure.
    pub fn imagecache(&self, path: String, cache: bool) -> Result<CachedFile, Error> {
        let pathbuf = PathBuf::from(path)
            .join(self.file.sha256().unwrap());
        let mut tempmap = match CachedFile::new(pathbuf, true, cache) {
            Ok(tempmmap) => tempmmap,
            Err(error) => return Err(error),
        };
        if tempmap.is_cached() {
            return Ok(tempmap);
        }
        tempmap.write(&self.file.data[0..self.sizeofheaders() as usize])?;
        for section in self.pe.sections() {
            if section.virtual_size() == 0 { continue; }
            if section.sizeof_raw_data() == 0 { continue; }
            let section_virtual_adddress = PE::align_section_virtual_address(
                self.imagebase() + section.pointerto_raw_data() as u64,
                self.section_alignment(),
                self.file_alignment());
            if section_virtual_adddress > tempmap.size() as u64 {
                let padding_length = section_virtual_adddress - tempmap.size() as u64;
                tempmap.write(&mut Cursor::new(vec![0u8; padding_length as usize]))?;
            }
            let pointerto_raw_data = section.pointerto_raw_data() as usize;
            let sizeof_raw_data = section.sizeof_raw_data() as usize;
            tempmap.write(&self.file.data[pointerto_raw_data..pointerto_raw_data + sizeof_raw_data])?;
        }
        Ok(tempmap)
    }

    /// Returns the image data of the PE file, including headers and sections.
    ///
    /// # Returns
    /// A `Vec<u8>` containing the raw image data.
    #[allow(dead_code)]
    pub fn image(&self) -> Vec<u8> {
        let mut data = Vec::<u8>::new();
        data.extend_from_slice(&self.file.data[0..self.sizeofheaders() as usize]);
        for section in self.pe.sections() {
            if section.virtual_size() == 0 { continue; }
            if section.sizeof_raw_data() == 0 { continue; }
            let section_virtual_adddress = PE::align_section_virtual_address(
                self.imagebase() + section.pointerto_raw_data() as u64,
                self.section_alignment(),
                self.file_alignment());
            if section_virtual_adddress > data.len() as u64 {
                let padding_length = section_virtual_adddress - data.len() as u64;
                data.extend(vec![0u8; padding_length as usize]);
            }
            let pointerto_raw_data = section.pointerto_raw_data() as usize;
            let sizeof_raw_data = section.sizeof_raw_data() as usize;
            data.extend_from_slice(&self.file.data[pointerto_raw_data..pointerto_raw_data + sizeof_raw_data]);
        }
        return data;
    }

    /// Returns the size of the PE file.
    ///
    /// # Returns
    /// The size of the file as a `u64`.
    #[allow(dead_code)]
    pub fn size(&self) -> u64 {
        self.file.size()
    }

    /// Returns the TLS (Thread Local Storage) hash value if present in the PE file.
    ///
    /// # Returns
    /// An `Option<String>` containing the TLS hash if present, otherwise `None`.
    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        self.file.tlsh()
    }

    /// Returns the SHA-256 hash value of the PE file.
    ///
    /// # Returns
    /// An `Option<String>` containing the SHA-256 hash if available, otherwise `None`.
    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        self.file.sha256()
    }

    /// Returns the base address (image base) of the PE file.
    ///
    /// # Returns
    /// The image base address as a `u64`.
    #[allow(dead_code)]
    pub fn imagebase(&self) -> u64 {
        self.pe.optional_header().imagebase()
    }

    /// Returns a set of exported function addresses in the PE file.
    ///
    /// # Returns
    /// A `BTreeSet` of exported function addresses.
    #[allow(dead_code)]
    pub fn exports(&self) -> BTreeSet<u64> {
        let mut addresses = BTreeSet::<u64>::new();
        let export = match self.pe.export(){
            Some(export) => export,
            None => {
                return addresses;
            }
        };
        for entry in export.entries(){
            let address = entry.address() as u64 + self.imagebase();
            addresses.insert(address);
        }
        return addresses;
    }
}
