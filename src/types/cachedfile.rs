use memmap2::Mmap;
use std::fs::OpenOptions;
use std::io::{self, Error, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

/// A `CachedFile` struct that provides a cached file interface,
/// enabling file read/write operations with optional in-memory caching,
/// and automatic file cleanup on object drop.
pub struct CachedFile {
    /// Path to the file as a `String`.
    pub path: String,
    /// Handle to the file as an open file descriptor.
    pub handle: std::fs::File,
    /// Flag indicating whether the file is already cached (exists on disk).
    pub is_cached: bool,
    /// Flag to determine if the file should be cached. If `false`, the file will
    /// be deleted upon the object being dropped.
    pub cache: bool,
}

impl CachedFile {
    /// Creates a new `CachedFile` instance.
    ///
    /// This function opens a file at the specified path, with options to append and/or cache the file.
    /// If the file's parent directories do not exist, they are created.
    ///
    /// # Arguments
    ///
    /// * `path` - The `PathBuf` specifying the file's location.
    /// * `append` - If `true`, opens the file in append mode.
    /// * `cache` - If `true`, retains the file on disk after the `CachedFile` instance is dropped.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `CachedFile` on success, or an `io::Error` if file creation fails.
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

    /// Checks if the file is cached (exists on disk).
    ///
    /// # Returns
    ///
    /// A `bool` indicating if the file was already present when the object was created.
    pub fn is_cached(&self) -> bool {
        return self.is_cached;
    }

    /// Retrieves the file path as a `String`.
    ///
    /// # Returns
    ///
    /// A `String` containing the path of the file.
    #[allow(dead_code)]
    pub fn path(&self) -> String {
        return self.path.clone();
    }

    /// Writes data from a reader to the file.
    ///
    /// This method copies all data from the given reader into the file, flushing the data
    /// to ensure it is written to disk.
    ///
    /// # Arguments
    ///
    /// * `reader` - A generic `Read` trait object supplying data to be written to the file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the number of bytes written on success, or an `io::Error` on failure.
    pub fn write<R: Read>(&mut self, mut reader: R) -> Result<u64, Error> {
        let bytes_written = io::copy(&mut reader, &mut self.handle)?;
        self.handle.flush()?;
        Ok(bytes_written)
    }

    /// Retrieves the size of the file in bytes.
    ///
    /// # Returns
    ///
    /// A `u64` representing the file's current size in bytes.
    pub fn size(&mut self) -> u64 {
        let current_position = self.handle.seek(SeekFrom::Current(0)).unwrap();
        let file_size = self.handle.seek(SeekFrom::End(0)).unwrap();
        self.handle.seek(SeekFrom::Start(current_position)).unwrap();
        file_size
    }

    /// Maps the file into memory using `mmap`.
    ///
    /// This method uses the `memmap2` crate to map the file into memory,
    /// allowing for direct memory access to the file contents.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Mmap` object on success, or an `io::Error` if mapping fails.
    pub fn mmap(&self) -> Result<Mmap, Error> {
        unsafe { Mmap::map(&self.handle) }
    }
}

/// Automatically handles cleanup for the `CachedFile` when it goes out of scope.
///
/// If caching is disabled, this `Drop` implementation deletes the file from disk
/// when the `CachedFile` instance is dropped, provided there were no errors in file removal.
impl Drop for CachedFile {
    fn drop(&mut self) {
        if !self.cache {
            if let Err(error) = std::fs::remove_file(&self.path) {
                eprintln!("{}", error);
            }
        }
    }
}