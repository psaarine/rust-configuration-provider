use std::ffi::OsString;
use serde::de::DeserializeOwned;
use super::{LocationSearcher, 
    combine_directory_and_file_name, 
    extract_readable_content_from_configuration_location};
use super::errors::*;

type FileMapper<T> = fn(Vec<u8>) -> Result<T, String>;

pub fn unwrap_file_location<T: DeserializeOwned>(file_name: OsString, 
                            file_searcher: &impl LocationSearcher,
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

