use std::path::PathBuf;
use std::ffi::OsStr;

pub struct ConfigurationElementProvider {
    location: PathBuf
}

impl ConfigurationElementProvider {

    pub fn new<T: AsRef<OsStr>>(element_name: T) -> super::ConfigurationElementProviderBuilder {
        return super::ConfigurationElementProviderBuilder::new(element_name);
    }

    pub fn try_get_json<T>(&self) -> Result<T,Box< dyn std::error::Error>> {
        unimplemented!();
    }
}
