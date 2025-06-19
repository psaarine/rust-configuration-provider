use super::errors::{FileValidationResultType, FileValidationFailure, LocationSearchType};
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub fn combine_directory_and_file_name(file_name: OsString, mut dir_path: PathBuf, 
                                       search_type: LocationSearchType) 
    -> Result<PathBuf, FileValidationFailure>{

        if dir_path.is_file() {

            return Err(create_malformed_path_entry_error(dir_path, search_type));
        }

        dir_path.set_file_name(file_name);

        return Ok(dir_path);
}

fn create_malformed_path_entry_error(dir_path: PathBuf, 
                                     search_type: LocationSearchType) -> FileValidationFailure {

    return FileValidationFailure {
        result:FileValidationResultType::NotADirectory,
        path: Some(dir_path),
        search_type: search_type,
        message: "path entry refers to file, not a directory".to_owned()
    };
}

pub fn extract_readable_content_from_configuration_location(path: PathBuf) -> std::io::Result<Vec<u8>> {

    let mut file = File::open(path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;

    return Ok(contents);
}
