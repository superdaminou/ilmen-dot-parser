use super::parsing_error::ParsingError;


#[derive(PartialEq, Eq, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GraphType {
    Graph,
    Digraph
}

impl TryFrom<&str> for GraphType {
    type Error = ParsingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "digraph" => Ok(GraphType::Digraph),
            "graph" => Ok(GraphType::Graph),
            other => Err(ParsingError::DefaultError(other.to_string()))
        }
    }
}

impl GraphType {
    pub fn symbol(&self) -> String {
        match  self {
            GraphType::Graph => "--".to_string(),
            GraphType::Digraph => "->".to_string(),
        }
    }
}

impl ToString for GraphType {
    fn to_string(&self) -> String {
        match self {
            GraphType::Graph => "graph".to_string(),
            GraphType::Digraph => "digraph".to_string() ,
        }
    }
}
