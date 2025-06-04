use std::vec::Vec;
use std::path::PathBuf;
use std::ffi::{OsStr, OsString};
use super::location_functions;
use super::file_result::{LocationFailure, LocationSearchType, LocationSuccess, validate_path_entry};
use serde::de::DeserializeOwned;

pub struct ConfigurationElementBuilder<T: DeserializeOwned> {
    file_name: OsString,
    failures: Vec<LocationFailure>,
    item: Option<ConfigurationElementBuilderSuccess<T>>
}

pub struct ConfigurationElementBuilderSuccess<T> {

    path: PathBuf,
    element: T

}

pub struct ConfigurationElementBuilderFailure {

    locations:Vec<LocationFailure>
}

impl<I: DeserializeOwned> ConfigurationElementBuilder<I> {

    pub fn of_current_dir(mut self) -> Self {

        let current_directory_with_configuration_file_name = location_functions::get_current_directory_location(&self.file_name);

        if let Err(e) = current_directory_with_configuration_file_name {

            self.failures.push(LocationFailure::DeterminingCurrentDirectoryFailed(e.to_string()));

            return self;
        };

        let current_directory_with_configuration_file_name = current_directory_with_configuration_file_name.unwrap();

        let path_entry = validate_path_entry(current_directory_with_configuration_file_name, LocationSearchType::CurrentDirectory);
        if let Err(error) = path_entry {

                self.failures.push(error);
                return self;
        }

        let path_entry_success = path_entry.unwrap();
        
        let item: serde_json::Result<I> = serde_json::from_str(&path_entry_success.content);

        match item {

            Ok(item) => {

                self.item = Some(ConfigurationElementBuilderSuccess { path: path_entry_success.path, element: item });

            },
            Err(e) => {

                self.failures.push(map_serialization_failure_to_common_failure(path_entry_success, LocationSearchType::CurrentDirectory, e));

            }
        }

        return self;
    }

    pub fn new<T: AsRef<OsStr>>(file_name: T) -> Self {

        return Self { 
            file_name: file_name.as_ref().to_os_string(),
            failures: Vec::new(),
            item: None,
        };
    }

    pub fn build_json(self) -> Result<ConfigurationElementBuilderSuccess<I>, ConfigurationElementBuilderFailure>{

        match self.item {
            Some(resp) => {
            return Ok(resp);
            },
            None => {

                return Err(ConfigurationElementBuilderFailure { locations: self.failures });
            }

        }
    }
}

fn map_serialization_failure_to_common_failure(success: LocationSuccess, 
                                               location_search_type: LocationSearchType, 
                                               serialization_failure: serde_json::Error) -> LocationFailure {

    return LocationFailure {
    failure_type: location_search_type,
    path: Some(success.path),
    message: serialization_failure.to_string()
    }
}
