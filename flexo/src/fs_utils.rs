use std::fs;
use std::path::Path;
use std::io;

pub fn create_dir_unless_exists(directory: &Path) -> io::Result<()> {
    fs::create_dir_all(directory)
}
