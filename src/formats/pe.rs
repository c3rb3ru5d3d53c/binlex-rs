use lief::generic::Section;
use lief::Binary;
use lief::pe::section::Characteristics;
use std::io::{Cursor, Error, ErrorKind};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::path::PathBuf;
use lief::pe::headers::MachineType;
use crate::Architecture;
use crate::formats::File;
use std::collections::BTreeMap;
use lief::pe::debug::Entries;
use crate::types::MemoryMappedFile;
use crate::Config;
use lief::pe::data_directory::Type as DATA_DIRECTORY;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataToken {
    Module = 0,
    TypeRef = 1,
    TypeDef = 2,
    FieldPtr = 3,
    Field = 4,
    MethodPtr = 5,
    MethodDef = 6,
    ParamPtr = 7,
    Param = 8,
    InterfaceImpl = 9,
    MemberRef = 10,
    Constant = 11,
    CustomAttribute = 12,
    FieldMarshal = 13,
    DeclSecurity = 14,
    ClassLayout = 15,
    FieldLayout = 16,
    StandAloneSig = 17,
    EventMap = 18,
    EventPtr = 19,
    Event = 20,
    PropertyMap = 21,
    PropertyPtr = 22,
    Property = 23,
    MethodSemantics = 24,
    MethodImpl = 25,
    ModuleRef = 26,
    TypeSpec = 27,
    ImplMap = 28,
    FieldRva = 29,
    EncLog = 30,
    EncMap = 31,
    Assembly = 32,
    AssemblyProcessor = 33,
    AssemblyOs = 34,
    AssemblyRef = 35,
    AssemblyRefProcessor = 36,
    AssemblyRefOs = 37,
    File = 38,
    ExportedType = 39,
    ManifestResource = 40,
    NestedClass = 41,
    GenericParam = 42,
    MethodSpec = 43,
    GenericParamConstraint = 44,
    Document = 48,
    MethodDebugInformation = 49,
    LocalScope = 50,
    LocalVariable = 51,
    LocalConstant = 52,
    ImportScope = 53,
    StateMachineMethod = 54,
    CustomDebugInformation = 55,
}

#[repr(C)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

#[repr(C)]
pub union ImageCor20Header0 {
    pub entry_point_token: u32,
    pub entry_point_rva: u32,
}

#[repr(C)]
pub struct ImageCor20Header {
    pub cb: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub meta_data: ImageDataDirectory,
    pub flags: u32,
    pub anonymous: ImageCor20Header0,
    pub resources: ImageDataDirectory,
    pub strong_name_signature: ImageDataDirectory,
    pub code_manager_table: ImageDataDirectory,
    pub vtable_fixups: ImageDataDirectory,
    pub export_address_table_jumps: ImageDataDirectory,
    pub managed_native_header: ImageDataDirectory,
}

impl ImageCor20Header {
    pub fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        if bytes.len() != mem::size_of::<Self>() {
            return None;
        }
        if bytes.as_ptr().align_offset(mem::align_of::<Self>()) != 0 {
            return None;
        }
        Some(unsafe { &*(bytes.as_ptr() as *const Self) })
    }
}

#[repr(C)]
pub struct Cor20StorageSignature {
    pub signature: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub extra_data: u32,
    pub version_string_size: u32,
    pub version_string: u32,
}

impl Cor20StorageSignature {
    pub fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        if bytes.len() != mem::size_of::<Self>() {
            return None;
        }
        if bytes.as_ptr().align_offset(mem::align_of::<Self>()) != 0 {
            return None;
        }
        Some(unsafe { &*(bytes.as_ptr() as *const Self) })
    }
}

#[repr(C)]
pub struct Cor20StorageHeader {
    pub flags: u8,
    pub pad: u8,
    pub number_of_streams: u16,
}

impl Cor20StorageHeader {
    pub fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        if bytes.len() != mem::size_of::<Self>() {
            return None;
        }
        if bytes.as_ptr().align_offset(mem::align_of::<Self>()) != 0 {
            return None;
        }
        Some(unsafe { &*(bytes.as_ptr() as *const Self) })
    }
}

#[repr(C)]
pub struct Cor20StreamHeader {
    pub offset: u32,
    pub size: u32,
}

impl Cor20StreamHeader {
    pub fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        if bytes.len() < mem::size_of::<Cor20StreamHeader>() {
            return None;
        }
        Some(unsafe { &*(bytes.as_ptr() as *const Cor20StreamHeader) })
    }

    pub fn name(&self) -> &[u8] {
        let header_size = mem::size_of::<Cor20StreamHeader>();
        let base_ptr = self as *const Self as *const u8;

        unsafe {
            let name_ptr = base_ptr.add(header_size);

            let mut len = 0;
            while *name_ptr.add(len) != 0 {
                len += 1;
            }

            let padded_len = (len + 4) & !3;

            std::slice::from_raw_parts(name_ptr, padded_len)
        }
    }

    pub fn header_size(&self) -> usize {
        let header_size = mem::size_of::<Cor20StreamHeader>();
        header_size + self.name().len()
    }
}

#[repr(C)]
pub struct Cor20MetadataTable {
        pub reserved: u32,
        pub major_version: u8,
        pub minor_version: u8,
        pub heap_sizes: u8,
        pub rid: u8,
        pub mask_valid: u64,
        pub mask_sorted: u64,
}

impl Cor20MetadataTable {
    pub fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        if bytes.len() != mem::size_of::<Self>() {
            return None;
        }
        if bytes.as_ptr().align_offset(mem::align_of::<Self>()) != 0 {
            return None;
        }
        Some(unsafe { &*(bytes.as_ptr() as *const Self) })
    }
}

#[repr(C)]
pub struct ModuleEntry {
    pub generation: u16,
    pub name: StringHeapIndex,
    pub mv_id: GuidHeapIndex,
    pub enc_id: GuidHeapIndex,
    pub enc_base_id: GuidHeapIndex,
}

impl ModuleEntry {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        if bytes.len() < 2 { return None; }
        let generation = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        let mut offset: usize = mem::size_of::<u16>();
        let name = StringHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += name.size();
        let mv_id = GuidHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += mv_id.size();
        let enc_id = GuidHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += enc_id.size();
        let enc_base_id = GuidHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        Some(Self {
            generation,
            name,
            mv_id,
            enc_id,
            enc_base_id,
        })
    }

    pub fn size(&self) -> usize {
        let mut size: usize = mem::size_of::<u16>();
        size += self.name.size();
        size += self.mv_id.size();
        size += self.enc_id.size();
        size += self.enc_base_id.size();
        size
    }
}

#[repr(C)]
pub struct TypeRefEntry {
    pub resolution_scope: ResolutionScopeIndex,
    pub name: StringHeapIndex,
    pub namespace: StringHeapIndex,
}

impl TypeRefEntry {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        let mut offset: usize = 0;
        let resolution_scope = ResolutionScopeIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += resolution_scope.size();
        let name = StringHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += name.size();
        let namespace = StringHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        Some(Self {
            resolution_scope,
            name,
            namespace,
        })
    }

    pub fn size(&self) -> usize {
        let mut size = self.resolution_scope.size();
        size += self.name.size();
        size += self.namespace.size();
        size
    }
}

#[repr(C)]
pub struct TypeDefEntry {
    pub flags: u32,
    pub name: StringHeapIndex,
    pub namespace: StringHeapIndex,
    pub extends: TypeDefOrRefIndex,
    pub field_list: SimpleTableIndex,
    pub method_list: SimpleTableIndex,
}

impl TypeDefEntry {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        if bytes.len() < 4 { return None; }
        let flags = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let mut offset: usize = mem::size_of::<u32>();
        let name = StringHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += name.size();
        let namespace = StringHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += namespace.size();
        let extends = TypeDefOrRefIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += extends.size();
        let field_list = SimpleTableIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += field_list.size();
        let method_list = SimpleTableIndex::from_bytes(&bytes[offset..], heap_size)?;
        Some(Self {
            flags,
            name,
            namespace,
            extends,
            field_list,
            method_list,
        })
    }

    pub fn size(&self) -> usize {
        let mut size: usize = mem::size_of::<u32>();
        size += self.name.size();
        size += self.namespace.size();
        size += self.extends.size();
        size += self.field_list.size();
        size += self.method_list.size();
        size
    }
}

#[repr(C)]
pub struct FieldEntry {
    pub flags: u16,
    pub name: StringHeapIndex,
    pub signature: BlobHeapIndex,
}

impl FieldEntry {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        if bytes.len() < 2 { return None; }
        let flags = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        let mut offset: usize = mem::size_of::<u16>();
        let name: StringHeapIndex = StringHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += name.size();
        let signature = BlobHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        Some(Self {
            flags,
            name,
            signature,
        })
    }

    pub fn size(&self) -> usize {
        let mut size: usize = mem::size_of::<u16>();
        size += self.name.size();
        size += self.signature.size();
        size
    }
}

#[repr(C)]
pub struct MethodDefEntry {
    pub rva: u32,
    pub impl_flags: u16,
    pub flags: u16,
    pub name: StringHeapIndex,
    pub signature: BlobHeapIndex,
    pub param_list: SimpleTableIndex,
}

impl MethodDefEntry {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        let rva = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let impl_flags = u16::from_le_bytes(bytes[4..6].try_into().unwrap());
        let flags = u16::from_le_bytes(bytes[6..8].try_into().unwrap());
        let mut offset: usize = 8;
        let name = StringHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += name.size();
        let signature = BlobHeapIndex::from_bytes(&bytes[offset..], heap_size)?;
        offset += signature.size();
        let param_list = SimpleTableIndex::from_bytes(&bytes[offset..], heap_size)?;
        Some(Self{
            rva,
            impl_flags,
            flags,
            name,
            signature,
            param_list,
        })
    }

    pub fn size(&self) -> usize {
        let mut size: usize = 8;
        size += self.name.size();
        size += self.signature.size();
        size += self.param_list.size();
        size
    }
}

#[repr(C)]
pub struct SimpleTableIndex {
    pub offset: u32,
    pub size: u32,
}

impl SimpleTableIndex {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        let size = if heap_size & 1 != 0 { 4 } else { 2 };

        let offset = match size {
            2 if bytes.len() >= 2 => u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as u32,
            4 if bytes.len() >= 4 => u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            _ => return None,
        };

        Some(Self {
            offset,
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}


#[derive(Debug)]
pub struct StringHeapIndex {
    pub offset: u32,
    pub size: u32,
}

impl StringHeapIndex {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        let size = if heap_size & 1 != 0 { 4 } else { 2 };

        let offset = match size {
            2 if bytes.len() >= 2 => u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as u32,
            4 if bytes.len() >= 4 => u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            _ => return None,
        };

        Some(Self {
            offset,
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

#[derive(Debug)]
pub struct GuidHeapIndex {
    pub offset: u32,
    pub size: u32,
}

impl GuidHeapIndex {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        let size = if heap_size & 2 != 0 { 4 } else { 2 };

        let offset = match size {
            2 if bytes.len() >= 2 => u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as u32,
            4 if bytes.len() >= 4 => u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            _ => return None,
        };

        Some(Self {
            offset,
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

#[repr(C)]
pub struct ResolutionScopeIndex {
    pub offset: u32,
    pub size: u32,
}

impl ResolutionScopeIndex {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        let size = if heap_size & 2 != 0 { 4 } else { 2 };

        let offset = match size {
            2 if bytes.len() >= 2 => u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as u32,
            4 if bytes.len() >= 4 => u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            _ => return None,
        };

        Some(Self {
            offset,
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TypeDefOrRefIndex {
    pub offset: u32,
    pub size: u32,
}

impl TypeDefOrRefIndex {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        let size = if heap_size & 2 != 0 { 4 } else { 2 };

        let offset = match size {
            2 if bytes.len() >= 2 => u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as u32,
            4 if bytes.len() >= 4 => u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            _ => return None,
        };

        Some(Self {
            offset,
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

#[repr(C)]
pub struct BlobHeapIndex {
    pub offset: u32,
    pub size: u32,
}

impl BlobHeapIndex {
    pub fn from_bytes(bytes: &[u8], heap_size: u8) -> Option<Self> {
        let size = if heap_size & 2 != 0 { 4 } else { 2 };

        let offset = match size {
            2 if bytes.len() >= 2 => u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as u32,
            4 if bytes.len() >= 4 => u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            _ => return None,
        };

        Some(Self {
            offset,
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

pub enum Entry {
    Module(ModuleEntry),
    TypeRef(TypeRefEntry),
    TypeDef(TypeDefEntry),
    Field(FieldEntry),
    MethodDef(MethodDefEntry),
}

#[repr(C)]
pub struct TinyHeader {
    code_size: u8,
}

impl TinyHeader {
    pub fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        if bytes.len() != mem::size_of::<Self>() {
            return None;
        }
        if bytes.as_ptr().align_offset(mem::align_of::<Self>()) != 0 {
            return None;
        }
        Some(unsafe { &*(bytes.as_ptr() as *const Self) })
    }

    pub fn size(&self) -> usize {
        1
    }
}

pub enum MethodHeader {
    Tiny(TinyHeader),
    Fat(FatHeader),
}

impl MethodHeader {
    pub fn size(&self) -> Option<usize> {
        match self {
            Self::Tiny(header) => Some(header.size()),
            Self::Fat(header) => Some(header.size()),
        }
    }
}

#[repr(C)]
pub struct FatHeader {
    pub flags: u16,
    pub size: u16,
    pub max_stack: u16,
    pub code_size: u32,
    pub local_var_sig_token: u32,
}

impl FatHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 12 {
            return Err(Error::new(ErrorKind::InvalidData, "not enough bytes for FatHeader"));
        }
        Ok(Self {
            flags: u16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            size: u16::from_le_bytes(bytes[2..4].try_into().unwrap()),
            max_stack: u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
            code_size: u32::from_le_bytes(bytes[6..10].try_into().unwrap()),
            local_var_sig_token: u32::from_le_bytes(bytes[10..12].try_into().unwrap()),
        })
    }

    pub fn size(&self) -> usize {
        14
    }
}

/// Represents a PE (Portable Executable) file, encapsulating the `lief::pe::Binary` and associated metadata.
pub struct PE {
    pub pe: lief::pe::Binary,
    pub file: File,
    pub config: Config,
}

impl PE {
    /// Creates a new `PE` instance by reading a PE file from the provided path.
    ///
    /// # Parameters
    /// - `path`: The file path to the PE file to be loaded.
    ///
    /// # Returns
    /// A `Result` containing the `PE` object on success or an `Error` on failure.
    pub fn new(path: String, config: Config) -> Result<Self, Error> {
        let mut file = File::new(path.clone(), config.clone())?;
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
                config: config,
            });
        }
        return Err(Error::new(ErrorKind::InvalidInput, "invalid pe file"));
    }

    /// Converts a relative virtual address to a file offset
    ///
    /// # Returns
    /// The file offset as a `Option<u64>`.
    pub fn relative_virtual_address_to_file_offset(&self, rva: u64) -> Option<u64> {
        for section in self.pe.sections() {
            let section_start_rva = section.virtual_address() as u64;
            let section_end_rva = section_start_rva + section.virtual_size() as u64;
            if rva >= section_start_rva && rva < section_end_rva {
                let section_offset = rva - section_start_rva;
                let file_offset = section.pointerto_raw_data() as u64 + section_offset;
                return Some(file_offset);
            }
        }
        None
    }

    fn parse_image_cor20_header(&self) -> Option<(u64, &ImageCor20Header)> {
        if !self.is_dotnet() { return None; }
        if let Some(clr_runtime_header) = self.pe.data_directory_by_type(DATA_DIRECTORY::CLR_RUNTIME_HEADER) {
            if let Some(start) = self.relative_virtual_address_to_file_offset(clr_runtime_header.rva() as u64) {
                let end = start + clr_runtime_header.size() as u64;
                let data = &self.file.data[start as usize..end as usize];
                let header = ImageCor20Header::from_bytes(&data)?;
                return Some((start, header));
            }
        }
        None
    }

    pub fn image_cor20_header(&self) -> Option<&ImageCor20Header> {
        Some(self.parse_image_cor20_header()?.1)
    }

    fn parse_cor20_storage_signature_header(&self) -> Option<(u64, &Cor20StorageSignature)> {
        if !self.is_dotnet() { return None; }
        let (_, image_cor20_header) = self.parse_image_cor20_header()?;
        let rva = image_cor20_header.meta_data.virtual_address as u64;
        let start = self.relative_virtual_address_to_file_offset(rva)? as usize;
        let end = start + mem::size_of::<Cor20StorageSignature>() as usize;
        let data = &self.file.data[start..end];
        let header = Cor20StorageSignature::from_bytes(&data)?;
        Some((start as u64, header))
    }

    pub fn cor20_storage_signature_header(&self) -> Option<&Cor20StorageSignature> {
        Some(self.parse_cor20_storage_signature_header()?.1)
    }

    fn parse_cor20_storage_header(&self) -> Option<(u64, &Cor20StorageHeader)> {
        if !self.is_dotnet() { return None; };
        let (mut start, cor20_storage_signaure_header) = self.parse_cor20_storage_signature_header()?;
        start += mem::size_of::<Cor20StorageSignature>() as u64;
        start += cor20_storage_signaure_header.version_string_size as u64;
        start -= mem::size_of::<u32>() as u64;
        let end = start as usize + mem::size_of::<Cor20StorageHeader>() as usize;
        let data = &self.file.data[start as usize..end];
        let header = Cor20StorageHeader::from_bytes(data)?;
        Some((start, header))
    }

    pub fn cor20_storage_header(&self) -> Option<&Cor20StorageHeader> {
        Some(self.parse_cor20_storage_header()?.1)
    }

    fn parse_cor20_stream_headers(&self) -> Option<BTreeMap<u64, &Cor20StreamHeader>> {
        if !self.is_dotnet() { return None; }
        let (cor20_storage_header_offset, cor20_storage_header) = self.parse_cor20_storage_header()?;
        let mut offset = cor20_storage_header_offset as usize + mem::size_of::<Cor20StorageHeader>();
        let mut result = BTreeMap::<u64, &Cor20StreamHeader>::new();
        for _ in 0.. cor20_storage_header.number_of_streams {
            let data = &self.file.data[offset..offset + mem::size_of::<Cor20StreamHeader>()];
            let header = Cor20StreamHeader::from_bytes(data)?;
            result.insert(offset as u64, header);
            offset += header.header_size();
        }
        if result.len() <= 0 {
            return None;
        }
        Some(result)
    }

    pub fn cor20_stream_headers(&self) -> Vec<&Cor20StreamHeader> {
        let mut result = Vec::<&Cor20StreamHeader>::new();
        let headers = self.parse_cor20_stream_headers();
        if headers.is_none() { return result; }
        for (_, header) in headers.unwrap() {
            result.push(header);
        }
        result
    }

    fn parse_cor20_metadata_table(&self) -> Option<(u64, &Cor20MetadataTable)> {
        if !self.is_dotnet() { return None; }
        let (mut start, _) = self.parse_cor20_storage_signature_header()?;
        for (_, header) in self.parse_cor20_stream_headers()? {
            if header.name() == vec![0x23, 0x7e, 0x00, 0x00] {
                start += header.offset as u64;
            }
        }
        let data = &self.file.data[start as usize..start as usize + mem::size_of::<Cor20MetadataTable>()];
        Some((start, Cor20MetadataTable::from_bytes(data)?))
    }

    pub fn cor20_metadata_table(&self) -> Option<&Cor20MetadataTable> {
        Some(self.parse_cor20_metadata_table()?.1)
    }

    pub fn cor20_metadata_table_entries(&self) -> Option<Vec<Entry>> {
        if !self.is_dotnet() { return None; }

        let (cor20_metadata_table_offset, cor20_metadata_table) = self.parse_cor20_metadata_table()?;

        let mut offset: usize = cor20_metadata_table_offset as usize
            + mem::size_of::<Cor20MetadataTable>()
            + cor20_metadata_table.mask_valid.count_ones() as usize * 4;

        let mut valid_index: usize = 0;

        let mut entries = Vec::<Entry>::new();

        for i in 0..64 as usize {

            let entry_offset = cor20_metadata_table_offset as usize
                + mem::size_of::<Cor20MetadataTable>()
                + (valid_index * 4);

            if entry_offset + 4 > self.file.data.len() {
                return None;
            }

            let entry_count = u32::from_le_bytes(
                self.file.data[entry_offset..entry_offset + 4].try_into().unwrap(),
            ) as usize;

            match i {
                x if x == MetadataToken::Module as usize => {
                    for _ in 0..entry_count {
                        let entry = ModuleEntry::from_bytes(
                            &self.file.data[offset..],
                            cor20_metadata_table.heap_sizes)?;
                        offset += entry.size();
                        entries.push(Entry::Module(entry));
                    }
                    valid_index += 1;
                }
                x if x == MetadataToken::TypeRef as usize => {
                    for _ in 0..entry_count {
                        let entry = TypeRefEntry::from_bytes(
                            &self.file.data[offset..],
                            cor20_metadata_table.heap_sizes)?;
                        offset += entry.size();
                        entries.push(Entry::TypeRef(entry));
                    }
                    valid_index += 1;
                }
                x if x == MetadataToken::TypeDef as usize => {
                    for _ in 0..entry_count {
                        let entry = TypeDefEntry::from_bytes(
                            &self.file.data[offset..],
                            cor20_metadata_table.heap_sizes,
                        )?;
                        offset += entry.size();
                        entries.push(Entry::TypeDef(entry));
                    }
                    valid_index += 1;
                }
                x if x == MetadataToken::Field as usize => {
                    for _ in 0..entry_count {
                        let entry = FieldEntry::from_bytes(
                            &self.file.data[offset..],
                            cor20_metadata_table.heap_sizes,
                        )?;
                        offset += entry.size();
                        entries.push(Entry::Field(entry));
                    }
                    valid_index += 1;
                }
                x if x == MetadataToken::MethodDef as usize => {
                    for _ in 0..entry_count {
                        let entry = MethodDefEntry::from_bytes(
                            &self.file.data[offset..],
                            cor20_metadata_table.heap_sizes)?;
                        offset += entry.size();
                        entries.push(Entry::MethodDef(entry));
                    }
                }
                _ => {}
            }
        }

        Some(entries)
    }

    pub fn virtual_address_to_relative_virtual_address(&self, address: u64) -> u64{
        address - self.imagebase()
    }

    pub fn virtual_address_to_file_offset(&self, address: u64) -> Option<u64> {
        let rva = self.virtual_address_to_relative_virtual_address(address);
        self.relative_virtual_address_to_file_offset(rva)
    }

    pub fn cor20_method_header(&self, address: u64) -> Result<MethodHeader, Error> {

        let offset = self.virtual_address_to_file_offset(address);

        if offset.is_none() { return Err(Error::new(ErrorKind::InvalidInput, "invalid virtual address")); }

        let bytes = &self.file.data[offset.unwrap() as usize..offset.unwrap() as usize + 12];

        if bytes[0] & 0b11 == 0b10 {
            let code_size = bytes[0] >> 2;
            let tiny_header = TinyHeader { code_size };
            return Ok(MethodHeader::Tiny(tiny_header));
        }
        if bytes[0] & 0b11 == 0b11 {
            let fat_header = FatHeader::from_bytes(bytes)?;
            return Ok(MethodHeader::Fat(fat_header));
        }
        return Err(Error::new(ErrorKind::InvalidData, "invalid method header"));
    }

    /// Checks if the PE file is a .NET assembly.
    ///
    /// This function inspects the imports of the PE file to identify whether it is a .NET application.
    /// It does so by looking for the presence of specific .NET-related DLLs (`mscorelib.dll` and `mscoree.dll`)
    /// in the import table and confirming the existence of a CLR runtime header.
    ///
    /// # Returns
    ///
    /// - `true` if the PE file is a .NET assembly.
    /// - `false` otherwise.
    #[allow(dead_code)]
    pub fn is_dotnet(&self) -> bool {
        self.pe.imports().any(|import| {
            matches!(import.name().to_lowercase().as_str(), "mscorelib.dll" | "mscoree.dll")
                && self.pe.data_directory_by_type(DATA_DIRECTORY::CLR_RUNTIME_HEADER).is_some()
        })
    }

    /// Creates a new `PE` instance from a byte vector containing PE file data.
    ///
    /// # Parameters
    /// - `bytes`: A vector of bytes representing the PE file data.
    ///
    /// # Returns
    /// A `Result` containing the `PE` object on success or an `Error` on failure.
    #[allow(dead_code)]
    pub fn from_bytes(bytes: Vec<u8>, config: Config) -> Result<Self, Error> {
        let file = File::from_bytes(bytes, config.clone());
        let mut cursor = Cursor::new(&file.data);
        if let Some(Binary::PE(pe)) = Binary::from(&mut cursor) {
            return Ok(Self{
                pe: pe,
                file: file,
                config: config,
            })
        }
        return Err(Error::new(ErrorKind::InvalidInput, "invalid pe file"));
    }

    /// Returns the architecture of the PE file based on its machine type.
    ///
    /// # Returns
    /// The `BinaryArchitecture` enum value corresponding to the PE machine type (e.g., AMD64, I386, CIL or UNKNOWN).
    #[allow(dead_code)]
    pub fn architecture(&self) -> Architecture {
        match self.pe.header().machine() {
            MachineType::I386 if self.is_dotnet() => Architecture::CIL,
            MachineType::I386 => Architecture::I386,
            MachineType::AMD64 => Architecture::AMD64,
            _ => Architecture::UNKNOWN,
        }
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
    pub fn entrypoints(&self) -> BTreeSet<u64> {
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

    /// Caches the PE file contents and returns a `MemoryMappedFile` object.
    ///
    /// # Parameters
    /// - `path`: The base path to store the memory mapped file.
    /// - `cache`: Whether to cache the file or not.
    ///
    /// # Returns
    /// A `Result` containing the `MemoryMappedFile` object on success or an `Error` on failure.
    pub fn image(&self) -> Result<MemoryMappedFile, Error> {
        let pathbuf = PathBuf::from(self.config.mmap.directory.clone())
            .join(self.file.sha256_no_config().unwrap());
        let mut tempmap = match MemoryMappedFile::new(pathbuf, true, self.config.mmap.cache.enabled) {
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
            if section_virtual_adddress > tempmap.size().unwrap() as u64 {
                let padding_length = section_virtual_adddress - tempmap.size().unwrap() as u64;
                tempmap.write_padding(padding_length as usize)?;
            }
            let pointerto_raw_data = section.pointerto_raw_data() as usize;
            let sizeof_raw_data = section.sizeof_raw_data() as usize;
            tempmap.write(&self.file.data[pointerto_raw_data..pointerto_raw_data + sizeof_raw_data])?;
        }
        Ok(tempmap)
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
