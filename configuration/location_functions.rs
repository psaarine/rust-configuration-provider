use std::env::current_dir;
use std::path::PathBuf;
use super::LocationSearchType;

pub trait LocationSearcher {
    fn try_get(&self) -> Result<PathBuf, String>;
    fn search_type(&self) -> LocationSearchType;
}

pub struct CurrentDirectoryLocationSearcher;

impl LocationSearcher for CurrentDirectoryLocationSearcher {


    fn try_get(&self) -> Result<PathBuf, String> {

    return Ok(current_dir()
        .map_err(|error| error.to_string())?);
    }

    fn search_type(&self) -> LocationSearchType {

        return LocationSearchType::CurrentDirectory;
    }

}
