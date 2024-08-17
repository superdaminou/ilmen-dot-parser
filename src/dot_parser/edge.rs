use crate::TypeRelation;

use super::{attributs::Attributs,  parsing_error::ParsingError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Edge{
    pub node_out: NodeId,
    pub node_in: NodeId,
    pub relation: TypeRelation,
    pub attributs: Attributs
}

type NodeId = String;


impl TryFrom<(&str, &str)> for Edge {
    type Error = ParsingError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let splitted = value.0
            .split_once(value.1)
            .ok_or(ParsingError::DefaultError("wtf".to_string()))?;

        let left_node= splitted.0.trim().to_string();
        let relation = TypeRelation::try_from(value.1)?;
        let right_node = splitted.1
            .split_once("[")
            .unwrap_or((splitted.1, "")).0
            .trim()
            .to_string();

        let attributs = match splitted.1.split_once("[") {
            Some((_, "")) => Attributs::default(),
            Some((_,b)) => Attributs::try_from(&b.replace("]",""))?,
            None => Attributs::default()
        };

        Ok(Self{node_out: left_node, node_in: right_node, relation, attributs})
    }
}

impl ToString for Edge {
    fn to_string(&self) -> String {
        self.node_out.clone() + " " + "->" + " " + &self.node_in + " " + &self.attributs.to_string() + ";"
    }
}

mod tests {
    use std::collections::HashMap;

    
    
    use super::*;
    
    #[test]
    fn try_from_ok() {
        let mut  map = HashMap::new();
        map.insert("toto".to_string(), "tutu".to_string());
        let combinations :Vec<(&str,Edge)> = vec![
            ("A->B", Edge{node_out: "A".to_string(), node_in: "B".to_string(), relation: TypeRelation::Oriente, attributs: Attributs::default()}),
            (" A -> B ", Edge{node_out: "A".to_string(), node_in: "B".to_string(), relation: TypeRelation::Oriente, attributs: Attributs::default()}),
            ("A->B[toto=tutu]", Edge{node_out: "A".to_string(), node_in: "B".to_string(), relation: TypeRelation::Oriente, attributs: Attributs::from(map)})
            ];
            

        combinations.iter().for_each(|combinaisons| assert_eq!(Edge::try_from((combinaisons.0, "->")).unwrap(), combinaisons.1));
    } 
}