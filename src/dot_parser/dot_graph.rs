
use log::debug;


use super::{attribute::Attribut, edge::Edge, graph_type::GraphType, node::Node, parsing_error::ParsingError};

#[derive(PartialEq,Clone)]
pub struct DotGraph {
    family: GraphType, 
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    sous_graphes: Vec<DotGraph>,
    attributs: Vec<Attribut>,
    name: String
}

impl Default for DotGraph {
    fn default() -> Self {
        Self { family: GraphType::Graph, nodes: Default::default(), edges: Default::default(), sous_graphes: Default::default(), attributs: Default::default(), name: Default::default() }
    }
}


// Create A graph from a valid DOT content
impl TryFrom<&str> for DotGraph {
    type Error = ParsingError;
    fn try_from(content: &str) -> Result<Self, Self::Error> {
        let mut cleaned_content = content.lines()
            .map(clean_line)
            .filter(|l| !l.is_empty() && !l.starts_with("//"))
            .collect::<String>();

        Self::create_graph(&mut cleaned_content, None)
    }
}



impl DotGraph {

    pub fn nodes(&self) ->Vec<Node> {
        let mut nodes = self.nodes.clone();  
        
        nodes
        .extend(
            self.sous_graphes.iter().flat_map(|g| g.nodes.clone()));

        nodes
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut edges = self.edges.clone();  
        edges.extend(self.sous_graphes.iter().flat_map(|g| g.edges.clone()));
        edges
    }

    fn create_graph(content: &mut String, parent: Option<GraphType>)  -> Result<DotGraph, ParsingError>{
        debug!("creating graph from: {}", content);
    
        let head_and_body = content.split_once("{").ok_or(ParsingError::DefaultError("Pas de corps ?".to_string()))?;
        let head = head_and_body.0;
        
        let type_graph = get_type_graph(head, parent)?;
        let name = head.split_once(" ").map(|(_gtype,name)| name).unwrap_or("NoName").trim();
        
        let mut body = head_and_body.1.to_string();

        let sous_graphes = Self::extract_subgraphes(&mut body, type_graph)?; 
        body.pop(); // Popping last } for the cleanest body 

        let mut attributs = vec![];
        let mut nodes =vec![];
        let mut edges =vec![];
        let mut default_node_attribute = vec![];
        let mut default_edge_attribute = vec![];
        
        body
            .split(";")
            .map(clean_line)
            .filter(|l| !l.is_empty())
            .try_for_each(|line| {
                if line.contains("->") {
                    let edge = Edge::try_from((line, "->"))?;
                    edges.push(edge);
                    return Ok(());
                } 
    
                if line.contains("[") || !line.contains("=") {
                    let node = Node::try_from(&line.to_string())?;
                    if node.0 == "node" {
                        default_node_attribute=node.1
                    } else if node.0 == "edge" {
                        default_edge_attribute=node.1
                    } else {
                        nodes.push(node);   
                    }
                    return Ok(());
                }
    
                let att = Attribut::try_from(line)?;
                attributs.push(att);
                Ok::<(), ParsingError>(())
            })?;
    
            Ok(DotGraph {name: name.to_string(), family: type_graph, sous_graphes, nodes, edges, attributs })
    }
    
    fn extract_subgraphes(body: &mut String, parent: GraphType) -> Result<Vec<DotGraph>, ParsingError> {
        let mut sous_graphes_position = extract_subgraphes_position(body)?;
    
        let sous_graphes = sous_graphes_position
                .iter()
                .map(|(start,end)|Self::create_graph(&mut body[*start..*end+1].to_string(), Some(parent)))
                .collect::<Result<Vec<DotGraph>, ParsingError>>()?;
    
        sous_graphes_position.reverse();
        for i in sous_graphes_position {
            body.replace_range(i.0..i.1+1, "");
        }

        Ok(sous_graphes)
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    
}


// Get the graph type from the first chars of content
fn get_type_graph(content: &str, parent: Option<GraphType>) -> Result<GraphType, ParsingError> {
    if content.starts_with("digraph") {
        return Ok(GraphType::Digraph);
    }

    if content.starts_with("graph") {
        return Ok(GraphType::Graph);
    }

    if content.starts_with("subgraph") {
        return parent.ok_or(ParsingError::DefaultError("Should have a parent".to_string()));
    }

    Err(ParsingError::DefaultError("No graph type detected".to_string()))
} 

// Removing comments and trimming
fn clean_line(line: &str) -> &str {
    line.split_once("//").map(|a|a.0).unwrap_or(line).trim()
}

fn extract_subgraphes_position(inside_block: &str) -> Result<Vec<(usize, usize)>, ParsingError> {

    let mut remaining = inside_block.to_string();

    let mut sub_graphes_ranges = vec![];
    let mut stack = 0;
    while remaining.contains("subgraph"){
        let start = remaining.find("subgraph").unwrap();
        let end = next_block_range(&remaining)?.1;
        sub_graphes_ranges.push((start+stack, end+stack));
        stack =end+1;

        remaining = remaining.split_at(end+1).1.to_string();
    } 
    Ok(sub_graphes_ranges)
}

fn next_block_range(block: &str) -> Result<(usize, usize), ParsingError>{
    let mut stack = 0;
    let mut index = 0;

    let mut range : (Option<usize>, Option<usize>)= (None, None);
    let mut chars = block.chars();
    let mut next= chars.next();
    while next.is_some() {
        let char = next.unwrap();
        
        if char == '{' {
            stack+=1;
            if range.0.is_none() {
                range.0 = Some(index);
            }
        }

        if char == '}' {
            stack -= 1;
            if stack == 0 {
                return match range.0 {
                    Some(start) => Ok((start, index)),
                    None => Err(ParsingError::DefaultError("Parsing exception error: no starting brackets".to_string()))
                }
            }
            if stack < 0 {
                return Err(ParsingError::DefaultError("Too many }".to_string()));
            }
        } 
        index +=1;
        next = chars.next();
    }

    Err(ParsingError::DefaultError("Missing ending }".to_string()))
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    

    #[test]
    fn test_find_ending_pos_combinations() {
        let combinations :Vec<(&str, (usize,usize))> = vec![
            ("{test -> a;}", (0,11)),
            ("{{}}", (0,3)),
            ("{icitoutvabien}", (0,14)),
            ("{{{{}}}}", (0,7)),
            ("{{{{}}}}}", (0,7)),
            ("graph Test {A;subgraph{D;}A->C}", (11, 30))
            ];
            
        combinations.iter().for_each(|combinaisons| assert_eq!(next_block_range(combinaisons.0).unwrap(), combinaisons.1));
    }

    #[test]
    fn test_find_ending_pos_combinations_ko() {
        let combinations :Vec<(&str, &str)> = vec![
            ("}test{}", "Too many }"),
            ("{testt", "Missing ending }"),
            ("{test{}", "Missing ending }")
            ];
            
        combinations.iter().for_each(|combinaisons| assert_eq!(next_block_range(combinaisons.0).unwrap_err().to_string(), ParsingError::DefaultError(combinaisons.1.to_string()).to_string()));

    } 


    #[test]
    fn graph_try_from() {
        let input = "graph Test {A; B [label=test, encore=toto]; A -> B;subgraph{C;D;C->D;}B -> A [label=\"to B\"];value=type;subgraph{C;D;C->D;}A->C;}";

        let result = DotGraph::try_from(input).unwrap();
        assert_eq!(result.name, "Test".to_string());
        assert_eq!(result.nodes, 
            vec![
                Node("A".to_string(), vec![]),
                Node("B".to_string(), vec![
                    Attribut{ key:"label".to_string(), value: "test".to_string()},
                    Attribut{ key:"encore".to_string(),value: "toto".to_string()}])]);
        assert_eq!(result.edges, 
            vec![
                Edge::try_from(("A->B", "->")).unwrap(),
                Edge::try_from(("B->A[label=\"to B\"", "->")).unwrap(),
                Edge::try_from(("A->C", "->")).unwrap()]);
        assert_eq!(result.sous_graphes.len(), 2);
    }


    #[test]
    fn extract_subgraphes_position_ok() {
        let combinations :Vec<(&str,Vec<(usize, usize)>)> = vec![
            ("subgraph{tetsautres}",vec![(0,19)]),
            ("another what ?subgraph{tetsautres}", vec![(14,33)]),
            ("subgraph{} subgraph{}",vec![(0,9), (11,20)]),
            ("subgraph{E;} subgraph{H;}",vec![(0,11), (13,24)]),
            ("encore un test subgraph{tetsautres} et au subgraph{ } voila du boulout", vec![(15,34),(42,52)]),
            ("subgraph{C;D;C->D;}\r\n", vec![(0,18)]),
            ("no sub grhaph", vec![])
            ];

        combinations.iter()
            .for_each(|combinaisons| 
                {
                    let result = extract_subgraphes_position(combinaisons.0).unwrap();
                    assert_eq!(result, combinaisons.1);
                }
            );
    } 

    #[test]
    fn extract_subgraphes_ok() {
        let combinations :Vec<(&str,usize, &str)> = vec![
            ("subgraph{A->B}", 1, ""),
            ("another what ?subgraph{C->D}", 1, "another what ?"),
            ("subgraph{E;} subgraph{H;}",2, " "),
            ("encore un test subgraph {G->D;} et au subgraph {A;} voila du boulout", 2, "encore un test  et au  voila du boulout"),
            ("subgraph{X;Y;Z->E;}\r\n", 1, "\r\n" ),
            ];

        combinations.iter()
            .for_each(|combinaisons| 
                {
                    let mut content = combinaisons.0.to_string(); 
                    let result = DotGraph::extract_subgraphes(&mut content, GraphType::Digraph).unwrap();
                    assert_eq!(result.len(), combinaisons.1);
                    assert_eq!(content, combinaisons.2)
                }
            );
    } 
}