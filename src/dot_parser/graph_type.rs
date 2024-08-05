use super::parsing_error::ParsingError;


#[derive(PartialEq, Eq, Clone, Copy)]
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
