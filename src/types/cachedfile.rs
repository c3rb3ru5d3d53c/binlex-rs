use memmap2::Mmap;
use std::fs::OpenOptions;
use std::io::{self, Error, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
pub struct CachedFile {
    pub path: String,
    pub handle: std::fs::File,
    pub is_cached: bool,
    pub cache: bool,
}

impl CachedFile {
    pub fn new(path: PathBuf, append: bool, cache: bool) -> Result<Self, Error> {

        std::fs::create_dir_all(path.parent().unwrap())?;

        let is_cached = path.is_file();

        let handle = OpenOptions::new()
            .read(true)
            .write(true)
            .append(append)
            .create(true)
            .open(&path)?;

        Ok(Self {
            path: path.to_string_lossy().into_owned(),
            handle,
            is_cached,
            cache,
        })
    }

    pub fn is_cached(&self) -> bool {
        return self.is_cached;
    }

    #[allow(dead_code)]
    pub fn path(&self) -> String {
        return self.path.clone();
    }

    pub fn write<R: Read>(&mut self, mut reader: R) -> Result<u64, Error> {
        let bytes_written = io::copy(&mut reader, &mut self.handle)?;
        self.handle.flush()?;
        Ok(bytes_written)
    }

    pub fn size(&mut self) -> u64 {
        let current_position = self.handle.seek(SeekFrom::Current(0)).unwrap();
        let file_size = self.handle.seek(SeekFrom::End(0)).unwrap();
        self.handle.seek(SeekFrom::Start(current_position)).unwrap();
        file_size
    }

    pub fn mmap(&self) -> Result<Mmap, Error> {
        unsafe { Mmap::map(&self.handle) }
    }
}

impl Drop for CachedFile {
    fn drop(&mut self) {
        if !self.cache {
            if let Err(error) = std::fs::remove_file(&self.path) {
                eprintln!("{}", error);
            }
        }
    }
}