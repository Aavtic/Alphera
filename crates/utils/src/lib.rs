use std::fs;
use std::path::PathBuf;

pub fn read_from_file(file_name: PathBuf) -> Result<String, std::io::Error> {
    fs::read_to_string(file_name)
}
