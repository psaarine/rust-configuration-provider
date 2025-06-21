use super::{LocationSearcher,unwrap_file_location};
use std::ops::Deref;
use std::collections::VecDeque;
use std::ffi::OsString;
use super::errors::{FileValidationSuccess, FileValidationFailure};

pub struct ConfigurationElementSearchParams {
    pub locations: Vec<Box<dyn LocationSearcher>>,
    pub file_name: OsString
}

pub fn get_configuration_element_from_params<I: serde::de::DeserializeOwned, P: AsRef<ConfigurationElementSearchParams>>(params: P) 
-> Result<FileValidationSuccess<I>, Vec<FileValidationFailure>>
{
    let params = params.as_ref();
    let mapper: fn(Vec<u8>) -> Result<I, String> = mock_mapper_signature;
    let mut errors: Vec<FileValidationFailure> = Vec::new();

    for item in params.locations.iter() {

        match unwrap_file_location(params.file_name.to_os_string(), item, mock_mapper_signature) {
            Ok(content) => return Ok(content),
            Err(e) => errors.push(e)
        }
    }

    return Err(errors);
}

fn mock_mapper_signature<T>(bytes: Vec<u8>) -> Result<T, String> {

    unimplemented!();
}
