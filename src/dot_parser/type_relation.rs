use super::parsing_error::ParsingError;

#[derive(Eq, PartialEq, Debug,Clone)]
pub enum TypeRelation {
    Oriente,
    NonOriente
}



impl ToString for TypeRelation {
    fn to_string(&self) -> String {
        match self {
            TypeRelation::Oriente => "->".to_string(),
            TypeRelation::NonOriente => "--".to_string(),
        }
    }
}

impl TryFrom<&str> for TypeRelation {
    type Error= ParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "->" => Ok(TypeRelation::Oriente),
            "--" => Ok(TypeRelation::NonOriente),
            _ => Err(ParsingError::DefaultError("Could not reconginze relation".to_string()))
        }
    }
}