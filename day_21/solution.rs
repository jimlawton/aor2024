use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let file_path = std::env::temp_dir().join(format!(
            "{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros()
        ));
        let file = File::create_new(&file_path)?;
        Ok(Self {
            file_path: file_path,
            file: file,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
        let mut file = match OpenOptions::new().write(true).open(&self.file_path) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        file.write(data)?;
        Ok(())
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = remove_file(&self.file_path);
    }
}

