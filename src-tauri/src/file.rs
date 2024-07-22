
use std::{fs::{File, OpenOptions}, io::{Read, Write}};
use std::path::{Path, PathBuf};

pub fn create_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn check_file_if_exists(path: impl AsRef<Path>) -> bool {
    Path::new(path.as_ref()).exists()
}

pub fn read_file_to_string(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn remove_file(path: &str) -> std::io::Result<()> {
    std::fs::remove_file(path)
}

pub fn write_to_file_append(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn write_to_file_overwrite(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}