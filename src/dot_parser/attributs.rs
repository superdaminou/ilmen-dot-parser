use std::collections::HashMap;

use super::parsing_error::ParsingError;


#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Attributs(Option<HashMap<String,String>>);

impl Attributs {
    pub fn label(&self) -> Option<&String> {
        self.get("label")
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.as_ref()
            .map(|attributs| attributs.get(key))
            .unwrap_or_default()
    }

    pub fn attributs(&self) -> Option<HashMap<String, String>> {
        self.0.clone()
    }

}

impl From<HashMap<String,String>> for Attributs {
    fn from(value: HashMap<String,String>) -> Self {
        Attributs(Some(value))
    }
}

impl ToString for Attributs {
    fn to_string(&self) -> String {
        self.0.clone().map(
            |attributs| "[".to_string() + &attributs.iter().map(|(id, value)| id.clone()+"="+value).collect::<Vec<_>>().join(",") + "]")
        .unwrap_or_default()
    }
}

impl TryFrom<&String> for Attributs  {
    type Error = ParsingError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.split(",")
            .map(as_key_value)
            .collect::<Result<HashMap<String, String>,ParsingError>>()
            .map(Attributs::from)
    }
}

fn as_key_value(value: &str) -> Result<(String, String), ParsingError> {
    let splitted = value.trim().split_once("=")
        .ok_or(ParsingError::DefaultError("Could not parse Attribute: ".to_string() + value))?;
    Ok((splitted.0.to_string(),splitted.1.to_string()))
}