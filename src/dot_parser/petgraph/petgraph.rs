use petgraph::{csr::DefaultIx, prelude::StableGraph, Directed};

use crate::{dot_parser::graph_type::GraphType, DotGraph, Edge, Node, Attributs};

impl<T: Sized,U: Sized> From<&StableGraph<T,U,Directed,DefaultIx>> for DotGraph 
where 
T: Into<Node> + Clone, 
U: Into<Edge> + Clone
{
    fn from(value: &StableGraph<T,U,Directed,DefaultIx>) -> Self {
        let edges = value.edge_weights().map(|n|n.clone().into()).collect::<Vec<Edge>>();
        let nodes = value.node_weights().map(|n| n.clone().into()).collect::<Vec<Node>>();
        DotGraph::new(GraphType::Digraph, nodes, edges, Vec::default(), Attributs::default(),"Graph".to_string())
    }
}