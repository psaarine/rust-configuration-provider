use std::path::PathBuf;
use std::fs;

pub fn validate_path_entry(path: PathBuf, location_search_type_identifier: LocationSearchType) -> Result<LocationSuccess, LocationFailure> {

    let read_to_string_result = fs::read_to_string(&path);

    if let Err(e) = read_to_string_result {

        return Err(LocationFailure {

            failure_type: location_search_type_identifier,
            message: format!("Failed to read contents of a file: {}", e.to_string()),
            path: Some(path.to_path_buf())
        });
    }

    return Ok(LocationSuccess {

        content: read_to_string_result.unwrap(),
        path: path.to_path_buf(),
        search_type: location_search_type_identifier

    });
}

pub struct LocationSuccess {

    pub content: String,
    pub path: PathBuf,
    pub search_type: LocationSearchType

}

#[derive(Debug)]
pub struct LocationFailure {

    pub failure_type:LocationSearchType,
    pub message: String,
    pub path: Option<PathBuf>

}

#[derive(Debug)]
pub enum LocationSearchType {

    CurrentDirectory

}

impl LocationFailure {

    pub fn DeterminingCurrentDirectoryFailed(message: String) -> Self {

        return Self {

            failure_type: LocationSearchType::CurrentDirectory,
            message: message,
            path: None

        }
    }
}
