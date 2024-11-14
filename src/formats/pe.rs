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
pub struct PE {
    pub _pe: lief::pe::Binary,
    pub file: File,
}

impl PE {
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
                _pe: pe,
                file: file,
            });
        }
        return Err(Error::new(ErrorKind::InvalidInput, "invalid pe file"));
    }

    #[allow(dead_code)]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        let file = File::from_bytes(bytes);
        let mut cursor = Cursor::new(&file.data);
        if let Some(Binary::PE(pe)) = Binary::from(&mut cursor) {
            return Ok(Self{
                _pe: pe,
                file: file
            })
        }
        return Err(Error::new(ErrorKind::InvalidInput, "invalid pe file"));
    }

    #[allow(dead_code)]
    pub fn machine(&self) -> BinaryArchitecture {
        let machine = match self._pe.header().machine() {
            MachineType::AMD64 => BinaryArchitecture::AMD64,
            MachineType::I386 => BinaryArchitecture::I386,
            _ => BinaryArchitecture::UNKNOWN,
        };
        return machine;
    }

    #[allow(dead_code)]
    pub fn executable_address_ranges(&self) -> BTreeMap<u64, u64> {
        let mut result = BTreeMap::<u64, u64>::new();
        for section in self._pe.sections() {
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

    #[allow(dead_code)]
    pub fn pogos(&self) -> HashMap<u64, String> {
        let mut result = HashMap::<u64, String>::new();
        for entry in self._pe.debug() {
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

    pub fn tlscallbacks(&self) -> BTreeSet<u64> {
        self._pe.tls()
            .into_iter()
            .flat_map(|tls| tls.callbacks())
            .collect()
    }

    #[allow(dead_code)]
    pub fn functions(&self) -> BTreeSet<u64> {
        let mut addresses = BTreeSet::<u64>::new();
        addresses.insert(self.entrypoint());
        addresses.extend(self.exports());
        addresses.extend(self.tlscallbacks());
        addresses.extend(self.pogos().keys().cloned());
        return addresses;
    }

    #[allow(dead_code)]
    pub fn entrypoint(&self) -> u64 {
        self.imagebase() + self._pe.optional_header().addressof_entrypoint() as u64
    }

    #[allow(dead_code)]
    pub fn sizeofheaders(&self) -> u64 {
        self._pe.optional_header().sizeof_headers() as u64
    }

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

    #[allow(dead_code)]
    pub fn section_alignment(&self) -> u64 {
        self._pe.optional_header().section_alignment() as u64
    }

    #[allow(dead_code)]
    pub fn file_alignment(&self) -> u64 {
        self._pe.optional_header().file_alignment() as u64
    }

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
        for section in self._pe.sections() {
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

    #[allow(dead_code)]
    pub fn image(&self) -> Vec<u8> {
        let mut data = Vec::<u8>::new();
        data.extend_from_slice(&self.file.data[0..self.sizeofheaders() as usize]);
        for section in self._pe.sections() {
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

    #[allow(dead_code)]
    pub fn size(&self) -> u64 {
        self.file.size()
    }

    #[allow(dead_code)]
    pub fn tlsh(&self) -> Option<String> {
        self.file.tlsh()
    }

    #[allow(dead_code)]
    pub fn sha256(&self) -> Option<String> {
        self.file.sha256()
    }

    #[allow(dead_code)]
    pub fn imagebase(&self) -> u64 {
        self._pe.optional_header().imagebase()
    }

    #[allow(dead_code)]
    pub fn exports(&self) -> BTreeSet<u64> {
        let mut addresses = BTreeSet::<u64>::new();
        let export = match self._pe.export(){
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
