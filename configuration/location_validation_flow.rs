use std::path::PathBuf;
use std::ffi::OsString;
use std::io::Read;
use serde::de::DeserializeOwned;
use super::{LocationSearcher, 
    combine_directory_and_file_name, 
    extract_readable_content_from_configuration_location};
use std::io::ErrorKind;

type FileMapper<T> = fn(Vec<u8>) -> Result<T, String>;

pub enum FileValidationResultType {
    FileNotFound,
    FileNotReadable,
    FileContentMalformed,
    UndefinedReadRelatedError,
    NotADirectory,
    CouldNotDetermineDirectory,
}

pub struct FileValidationFailure {
    pub result: FileValidationResultType,
    pub message: String,
    pub search_type: LocationSearchType,
    pub path: Option<PathBuf>,
}

pub struct FileValidationSuccess<T> {

    pub item: T,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum LocationSearchType {
    CurrentDirectory
}

pub fn unwrap_file_location<T: DeserializeOwned, S: LocationSearcher>(file_name: OsString, 
                            file_searcher: S,
                            file_mapper: FileMapper<T>) -> Result<FileValidationSuccess<T>, FileValidationFailure> {

    let map_potential_directory_failure = |error:String| map_directory_search_error_to_file_validation_failure(file_searcher.search_type(), error);

    let path = file_searcher
        .try_get()
        .map_err(map_potential_directory_failure)?;

    let file_location = combine_directory_and_file_name(file_name, path, file_searcher.search_type())?;

    let map_potential_file_io_error = |error:std::io::Error| map_std_io_error_to_file_validation_failure(error, file_location.clone(), file_searcher.search_type());

    let file_location_as_byte_vector = extract_readable_content_from_configuration_location(file_location.clone())
        .map_err(map_potential_file_io_error)?;

    let map_potential_deserialization_error = |error:String| map_data_deserialization_error_to_file_validation_failure(error, file_location.clone(), file_searcher.search_type());
    let map_succesfull_file_deserialization_to_validation_success = |item: T| return FileValidationSuccess { path: file_location.clone(), item: item };

    return Ok(file_mapper(file_location_as_byte_vector)
              .map_err(map_potential_deserialization_error)
              .map(map_succesfull_file_deserialization_to_validation_success)?);

}

fn map_directory_search_error_to_file_validation_failure(search_type: LocationSearchType, err_msg: String) -> FileValidationFailure {

    return FileValidationFailure {
        result: FileValidationResultType::CouldNotDetermineDirectory,
        message: err_msg,
        search_type: search_type,
        path:None
    };
}

fn map_std_io_error_to_file_validation_failure(error: std::io::Error, path: PathBuf, search_type: LocationSearchType) -> FileValidationFailure {

    return FileValidationFailure {
        result: map_io_error_to_file_validation_error(error.kind()),
        message: error.to_string(),
        search_type: search_type,
        path: Some(path),
    };
}

fn map_io_error_to_file_validation_error(error_kind: std::io::ErrorKind) -> FileValidationResultType {

    match error_kind {
        ErrorKind::NotFound => return FileValidationResultType::FileNotFound,
        ErrorKind::PermissionDenied => return FileValidationResultType::FileNotReadable,
        _ => return FileValidationResultType::UndefinedReadRelatedError
    }
    
}

fn map_data_deserialization_error_to_file_validation_failure(error_msg: String, path: PathBuf, search_type: LocationSearchType) -> FileValidationFailure {

    return FileValidationFailure {

        result: FileValidationResultType::FileContentMalformed,
        message:error_msg,
        search_type: search_type,
        path: Some(path),
    }
}
