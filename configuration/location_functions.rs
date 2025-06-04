use std::env::current_dir;
use std::ffi::OsStr;
use std::path::PathBuf;

pub fn get_current_directory_location<T: AsRef<OsStr>>(file_name: T) -> Result<PathBuf, Box<dyn std::error::Error>> {

    let current_dir = current_dir();

    if let Err(e) = current_dir {

        return Err(Box::new(e));
    }

    let mut current_dir = current_dir.unwrap();
    current_dir.set_file_name(file_name);
    return Ok(current_dir);
}
