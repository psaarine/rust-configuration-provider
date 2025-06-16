pub trait Deserializer<T: serde::de::DeserializeOwned> {

    fn deserialize_from_reader<I: std::io::Read>(content:I) -> Result<T, String>;
}

pub struct JsonSerializer;

impl<T: serde::de::DeserializeOwned>  Deserializer<T> for JsonSerializer {

    fn deserialize_from_reader<I: std::io::Read>(content:I) -> Result<T, String> {

        return Ok(serde_json::from_reader(content)
                .map_err(|err| err.to_string())?
                  );
    }
}


