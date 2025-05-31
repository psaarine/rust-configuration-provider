use super::ConfigurationElementProvider;
use std::vec::Vec;
use std::path::PathBuf;
use std::ffi::{OsStr, OsString};
use super::location_functions;
use super::file_result::{LocationFailure, LocationSearchType};

pub struct ConfigurationElementProviderBuilder {
    locations: Vec<PathBuf>,
    file_name: OsString,
    failures: Vec<LocationFailure>
}

impl ConfigurationElementProviderBuilder {

    pub fn of_current_dir(mut self) -> Self {

        let current_directory_with_configuration_file_name = location_functions::get_current_directory_location(&self.file_name);

        if let Err(e) = current_directory_with_configuration_file_name {

            self.failures.push(LocationFailure { 
                failure_type: LocationSearchType::CurrentDirectory, 
                message: e.to_string(), 
                path: None });

            return self;
        };

        return self;
    }

    pub fn new<T: AsRef<OsStr>>(file_name: T) -> Self {

        let file_name = file_name.as_ref().to_os_string();
        return Self { locations:Vec::new(), 
            file_name: file_name,
            failures: Vec::new()
        };
    }

    pub fn build(self) -> ConfigurationElementProvider {

        /*
        for location in self.locations {

        }
        */

        unimplemented!();
    }
}
