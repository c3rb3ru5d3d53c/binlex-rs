use memmap2::{Mmap, MmapMut};
use std::fs::OpenOptions;
use std::io::{self, Error, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;
#[cfg(windows)]
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE};

/// A `MemoryMappedFile` struct that provides a memory mapped file interface,
/// enabling file read/write operations with optional disk caching,
/// and automatic file cleanup on object drop.
pub struct MemoryMappedFile {
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

impl MemoryMappedFile {
    /// Creates a new `MemoryMappedFile` instance.
    ///
    /// This function opens a file at the specified path, with options to append and/or cache the file.
    /// If the file's parent directories do not exist, they are created.
    ///
    /// # Arguments
    ///
    /// * `path` - The `PathBuf` specifying the file's location.
    /// * `append` - If `true`, opens the file in append mode.
    /// * `cache` - If `true`, retains the file on disk after the `MemoryMappedFile` instance is dropped.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `MemoryMappedFile` on success, or an `io::Error` if file creation fails.
    pub fn new(path: PathBuf, append: bool, cache: bool) -> Result<Self, Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let is_cached = path.is_file();

        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true);
        if append {
            options.append(true);
        }

        #[cfg(windows)]
        options.share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE);

        let handle = options.open(&path)?;

        Ok(Self {
            path: path.to_string_lossy().into_owned(),
            handle,
            is_cached,
            cache,
        })
    }

    /// Creates a new `MemoryMappedFile` instance.
    ///
    /// This function opens a file at the specified path, in readonly mode.
    ///
    /// # Arguments
    ///
    /// * `path` - The `PathBuf` specifying the file's location.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `MemoryMappedFile` on success, or an `Error` if file creation fails.
    pub fn new_readonly(path: PathBuf) -> Result<Self, Error> {
        let mut options = OpenOptions::new();

        options
            .read(true)
            .write(false)
            .create(false)
            .append(false);

        #[cfg(windows)]
        options.share_mode(FILE_SHARE_READ);

        let handle = options.open(&path)?;

        Ok(Self {
            path: path.to_string_lossy().into_owned(),
            handle: handle,
            is_cached: false,
            cache: false,
        })
    }

    /// Checks if the file is cached (exists on disk).
    ///
    /// # Returns
    ///
    /// A `bool` indicating if the file was already present when the object was created.
    pub fn is_cached(&self) -> bool {
        self.is_cached
    }

    /// Retrieves the file path as a `String`.
    ///
    /// # Returns
    ///
    /// A `String` containing the path of the file.
    #[allow(dead_code)]
    pub fn path(&self) -> String {
        self.path.clone()
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
        // If in append mode, ensure the write pointer is at the end
        // OpenOptions with append=true should handle this, but double-check
        if self.handle.metadata()?.permissions().readonly() {
            return Err(Error::new(io::ErrorKind::Other, "File is read-only"));
        }

        let bytes_written = io::copy(&mut reader, &mut self.handle)?;
        self.handle.flush()?;
        Ok(bytes_written)
    }

    /// Adds symbolic padding (increases the file size without writing data) to the end of the file.
    ///
    /// This method sets the file length to the current size plus the specified padding length.
    /// The padding does not consume additional disk space as it is not physically written.
    ///
    /// # Arguments
    /// * `length` - The number of bytes to append as padding.
    ///
    /// # Returns
    /// A `Result` indicating success or an `io::Error` if the operation fails.
    pub fn write_padding(&mut self, length: usize) -> Result<(), Error> {
        // Get the current file size
        let current_size = self.handle.metadata()?.len();

        // Calculate the new size after padding
        let new_size = current_size + length as u64;

        // Resize the file to the new size
        self.handle.set_len(new_size)?;

        // No need to write zeros; this creates a sparse region
        // If the filesystem supports sparse files, this won't increase disk usage

        // Optionally, you can seek to the end if in append mode
        if self.handle.seek(SeekFrom::End(0))? != new_size {
            // This ensures that the next write will append correctly
            self.handle.seek(SeekFrom::Start(new_size))?;
        }

        Ok(())
    }

    /// Maps the file into memory as mutable using `mmap2`.
    #[allow(dead_code)]
    pub fn mmap_mut(&self) -> Result<MmapMut, Error> {
        unsafe { MmapMut::map_mut(&self.handle) }
    }

    /// Retrieves the size of the file in bytes.
    ///
    /// # Returns
    ///
    /// A `u64` representing the file's current size in bytes.
    pub fn size(&self) -> Result<u64, Error> {
        let file_size = self.handle.metadata()?.len();
        Ok(file_size)
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

/// Automatically handles cleanup for the `MemoryMappedFile` when it goes out of scope.
///
/// If caching is disabled, this `Drop` implementation deletes the file from disk
/// when the `MemoryMappedFile` instance is dropped, provided there were no errors in file removal.
impl Drop for MemoryMappedFile {
    fn drop(&mut self) {
        if !self.cache {
            if let Err(error) = std::fs::remove_file(&self.path) {
                eprintln!("Failed to remove file {}: {}", self.path, error);
            }
        }
    }
}
