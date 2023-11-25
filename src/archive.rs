use anyhow::{Context, Result};
use std::fs;
use std::time::UNIX_EPOCH;

type Archive = Vec<ArchivedFile>;

/// Reads an archive from a file and decodes it
/// 
/// # Arguments
/// 
/// `file_name` - Name of file to read
pub fn read_archive(file_name: String) -> Result<Archive> {
    let mut archive = Archive::new();
    let mut data = std::fs::read(file_name)?;

    ArchivedFile::from_archive(&mut data, &mut archive)?;
    return Ok(archive)
}

/// Takes a list of files, reads them, converts them into the format for the archive and returns a complete archive as a string
pub fn write_archive(files: Vec<String>) -> Result<String> {
    let mut archive_string = String::new();

    for file in files {
        archive_string.push_str(&ArchivedFile::new(file)?.write());
    }

    return Ok(archive_string);
}

/// Represents an encoded/decoded archived file
#[derive(Debug, Clone)]
pub struct ArchivedFile {
    pub name: String,
    pub size: u32,
    pub modification_timestamp: u32,
    pub contents: String,
}

impl ArchivedFile {
    /// Creates a new encoded file from a file on disk
    /// 
    /// # Arguments
    /// 
    /// `file_name` - File to encode
    pub fn new(mut file_name: String) -> Result<Self> {
        let data = fs::read(&file_name)
            .with_context(|| format!("Failed to read file '{}'", &file_name))?;
        let file_metadata = fs::metadata(&file_name)?;

        // Ensures that the length of the name is 100 characters by either padding it with null characters
        // or cutting off the nam
        if file_name.len() < 100 {
            let remaining_length = 100 - file_name.len();
            let null_chars = "\0".repeat(remaining_length);
            file_name.push_str(&null_chars);
        } else if file_name.len() > 100 {
            anyhow::bail!("File name cannot be longer than 100 characters")
        }

        // Get the timestamp for when the file was last modified
        let file_modification_date = file_metadata
            .modified()?
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let data = std::str::from_utf8(&data)?.to_string();

        Ok(Self {
            name: file_name,
            size: data.len() as u32,
            modification_timestamp: file_modification_date as u32,
            contents: data,
        })
    }

    /// Decodes a file from an archive adding it to an Archive passed into the function
    /// 
    /// # Arguments
    /// 
    /// `data` - Archive file data to decode
    /// 
    /// `archive` - A reference to an Archive that will have the output of the file
    pub fn from_archive(data: &mut Vec<u8>, archive: &mut Archive) -> Result<()> {
        if data.len() == 0 {
            return Ok(())
        }

        let new_data = data.clone();

        let name = &new_data[..100];
        let name = std::str::from_utf8(name)?.replace("\0", "");

        let size = &new_data[100..108];
        let size = u32::from_str_radix(std::str::from_utf8(size)?, 16)?;

        let modification_timestamp = &new_data[109..116];
        let modification_timestamp =
            u32::from_str_radix(std::str::from_utf8(modification_timestamp)?, 16)?;

        let contents = &new_data[116..116 + size as usize];

        data.drain(..116 + size as usize);

        let file = ArchivedFile{ name, size, modification_timestamp, contents: std::str::from_utf8(contents)?.to_string()};
        archive.push(file);
        
        ArchivedFile::from_archive(data, archive)
    }

    /// Encodes the archived file into it's final format to be written to a file
    pub fn write(&self) -> String {
        let mut archived_data = String::new();
        archived_data.push_str(&self.name);

        let file_size_hex = format!("{:08x}", self.size);

        archived_data.push_str(&file_size_hex);

        let file_modification_hex = format!("{:08x}", self.modification_timestamp);
        archived_data.push_str(&file_modification_hex);

        archived_data.push_str(&self.contents);
        return archived_data;
    }
}
