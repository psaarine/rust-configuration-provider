use std::path::PathBuf;
use std::io::ErrorKind;

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

pub fn map_directory_search_error_to_file_validation_failure(search_type: LocationSearchType, err_msg: String) -> FileValidationFailure {

    return FileValidationFailure {
        result: FileValidationResultType::CouldNotDetermineDirectory,
        message: err_msg,
        search_type: search_type,
        path:None
    };
}

pub fn map_std_io_error_to_file_validation_failure(error: std::io::Error, path: PathBuf, search_type: LocationSearchType) -> FileValidationFailure {

    return FileValidationFailure {
        result: map_io_error_to_file_validation_error(error.kind()),
        message: error.to_string(),
        search_type: search_type,
        path: Some(path),
    };
}

pub fn map_io_error_to_file_validation_error(error_kind: std::io::ErrorKind) -> FileValidationResultType {

    match error_kind {
        ErrorKind::NotFound => return FileValidationResultType::FileNotFound,
        ErrorKind::PermissionDenied => return FileValidationResultType::FileNotReadable,
        _ => return FileValidationResultType::UndefinedReadRelatedError
    }
    
}

pub fn map_data_deserialization_error_to_file_validation_failure(error_msg: String, path: PathBuf, search_type: LocationSearchType) -> FileValidationFailure {

    return FileValidationFailure {

        result: FileValidationResultType::FileContentMalformed,
        message:error_msg,
        search_type: search_type,
        path: Some(path),
    }
}
