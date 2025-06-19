use super::{LocationSearcher,unwrap_file_location};
use std::collections::VecDeque;
use std::ffi::OsString;

pub struct ConfigurationElementSearchParams {

    pub locations: VecDeque<Box<dyn LocationSearcher>>,
    pub file_name: OsString
}

pub fn get_configuration_element_from_params<I: serde::de::DeserializeOwned, P: AsRef<ConfigurationElementSearchParams>>(params: P) 

{
    let params = params.as_ref();

    for item in params.locations.iter() {

        //match unwrap_file_location(params.file_name.to_os_string(), item,
    }

}
