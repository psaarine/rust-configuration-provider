
pub fn deserialize_from_reader<I: std::io::Read, T: serde::de::DeserializeOwned>(content:I) -> Result<T, String> {

    return Ok(serde_json::from_reader(content)
            .map_err(|err| err.to_string())?
             );
}
