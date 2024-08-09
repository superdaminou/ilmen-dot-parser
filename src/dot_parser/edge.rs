use super::{attribute::{ new_from_array, Attribut}, parsing_error::ParsingError};

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct Edge{
    pub node_out: NodeId,
    pub node_in: NodeId,
    pub relation: String,
    pub attributs: Vec<Attribut>
}

type NodeId = String;


impl TryFrom<(&str, &str)> for Edge {
    type Error = ParsingError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let splitted = value.0
            .split_once(value.1)
            .ok_or(ParsingError::DefaultError("wtf".to_string()))?;

        let left_node= splitted.0.trim().to_string();
        let relation = value.1.to_string();
        let right_node = splitted.1
            .split_once("[")
            .unwrap_or((splitted.1, "")).0
            .trim()
            .to_string();

        let attributs = match splitted.1.split_once("[") {
            Some((_, "")) => vec![],
            Some((_,b)) => new_from_array(&b.replace("]",""))?,
            None => vec![]
        };

        Ok(Self{node_out: left_node, node_in: right_node, relation, attributs})
    }
}

#[test]
fn try_from_ok() {
    let combinations :Vec<(&str,Edge)> = vec![
        ("A->B", Edge{node_out: "A".to_string(), node_in: "B".to_string(), relation: "->".to_string(), attributs: vec![]}),
        (" A -> B ", Edge{node_out: "A".to_string(), node_in: "B".to_string(), relation: "->".to_string(), attributs: vec![]}),
        ("A->B[toto=tutu]", Edge{node_out: "A".to_string(), node_in: "B".to_string(), relation: "->".to_string(), attributs: vec![Attribut::try_from("toto=tutu").unwrap()]})
        ];
        

    combinations.iter().for_each(|combinaisons| assert_eq!(Edge::try_from((combinaisons.0, "->")).unwrap(), combinaisons.1));
} 