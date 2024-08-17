use crate::Attributs;
use super::parsing_error::ParsingError;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Node{
    pub identifier: String,
    pub attributes: Attributs
}

impl TryFrom<&String> for Node {
    type Error = ParsingError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {

        let split = value.split_once("[").unwrap_or((value, ""));
        let attr = match split.1.is_empty() {
            true => Attributs::default(),
            false => Attributs::try_from(&split.1.replace("]",""))?
        };
        
        Ok(Self{identifier: split.0.trim().to_string(), attributes: attr})
    }
}



impl Node {
    pub fn new(identifier: &str, attributes: Attributs) -> Self {
        Self{
            identifier: identifier.to_string(), 
            attributes
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut content = self.identifier.clone();

        let attributes_to_string = self.attributes.to_string();

        content = content + &attributes_to_string +  ";";
        content
    }
}

mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn try_from_ok() {
        
        let mut first_map = HashMap::new();
        first_map.insert("label".to_string(), "\"toto\"".to_string());
        let mut second_map = HashMap::new();
        second_map.insert("label".to_string(), "\"toto\"".to_string());
        second_map.insert("encore".to_string(), "2".to_string());
        let combinations :Vec<(&str,Node)> = vec![
            ("A", Node::new("A", Attributs::default())),
            ("A_long_name", Node::new("A_long_name", Attributs::default())),
            ("Bepourquoi[label=\"toto\"]", Node::new("Bepourquoi",Attributs::from(first_map))),
            ("Bepourquoi[label=\"toto\",encore=2]", Node::new("Bepourquoi", Attributs::from(second_map)))
            ];
            
    
        combinations.iter().for_each(|combinaisons| assert_eq!(Node::try_from(&combinaisons.0.to_string()).unwrap(), combinaisons.1));
    } 
}