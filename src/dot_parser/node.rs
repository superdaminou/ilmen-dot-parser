use super::{attribute::{new_from_array, Attribut}, parsing_error::ParsingError};


#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Node(pub String,pub Vec<Attribut>);

impl TryFrom<&String> for Node {
    type Error = ParsingError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {

        let split = value.split_once("[").unwrap_or((value, ""));
        let attr = match split.1.is_empty() {
            true => vec![],
            false => new_from_array(&split.1.replace("]",""))?
        };
        
        Ok(Self(split.0.trim().to_string(), attr))
    }
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self(name.to_string(), vec![])
    }
}



#[test]
fn try_from_ok() {
    let combinations :Vec<(&str,Node)> = vec![
        ("A", Node("A".to_string(), vec![])),
        ("A_long_name", Node("A_long_name".to_string(), vec![])),
        ("Bepourquoi[label=\"toto\"]", Node("Bepourquoi".to_string(), vec![Attribut::try_from("label=\"toto\"").unwrap()])),
        ("Bepourquoi[label=\"toto\",encore=2]", Node("Bepourquoi".to_string(), vec![Attribut::try_from("label=\"toto\"").unwrap(), Attribut::try_from("encore=2").unwrap()]))
        ];
        

    combinations.iter().for_each(|combinaisons| assert_eq!(Node::try_from(&combinaisons.0.to_string()).unwrap(), combinaisons.1));
} 