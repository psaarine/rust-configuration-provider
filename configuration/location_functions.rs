use std::env::current_dir;
use std::ffi::OsStr;
use std::path::PathBuf;

/*
pub fn preformat_configuration_directory_with_file_name<T: AsRef<OsStr>>(file_name: T, mut base:PathBuf) -> PathBuf {

    let file_name = file_name.as_ref();
    let existing_file_name = base.file_name();

    if existing_file_name == None || existing_file_name.unwrap() != file_name {

        base.set_file_name(file_name);
        return base;

    } else {

        return base;
    }

}
*/

pub fn get_current_directory_location<T: AsRef<OsStr>>(file_name: T) -> Result<PathBuf, Box<dyn std::error::Error>> {

    let current_dir = current_dir();

    if let Err(e) = current_dir {

        return Err(Box::new(e));
    }

    let mut current_dir = current_dir.unwrap();
    current_dir.set_file_name(file_name);
    return Ok(current_dir);
}
