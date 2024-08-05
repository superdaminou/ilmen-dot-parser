use super::parsing_error::ParsingError;


#[derive(PartialEq, Eq,Debug, Clone)]
pub struct Attribut{pub key: String,pub value: String}

impl ToString for Attribut {
    fn to_string(&self) -> String {
        format!("{}:{}", self.key, self.value)
    }
}

impl TryFrom<&str> for Attribut {
    type Error = ParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splitted = value.trim().split_once("=")
            .ok_or(ParsingError::DefaultError("Could not parse Attribute: ".to_string() + value))?;
        Ok(Self{key: splitted.0.to_string(), value:splitted.1.to_string()})
    }
}

pub fn new_from_array(array: &String) -> Result<Vec<Attribut>, ParsingError> {
    array.split(",").map(Attribut::try_from).collect::<Result<Vec<Attribut>,ParsingError>>()
}



#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_extract_attributes() {
        let combinations :Vec<(&str, Attribut)> = vec![
            ("ata=3", Attribut{ key:"ata".to_string(), value: "3".to_string()}),
            ("encore=\"encore\"", Attribut{ key:"encore".to_string(),value: "\"encore\"".to_string()}),
            ("2=label", Attribut{ key:"2".to_string(), value: "label".to_string()}),
        ];
            
        combinations.iter().for_each(|combinaisons|{
             assert_eq!(Attribut::try_from(combinaisons.0).unwrap(), combinaisons.1)
            });
    }
}

    

