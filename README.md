# Ilmen DOT PARSER

A standard and basic library to read from file, manipulate and write DOT graph. 

This is still a project under construction and primiraly use by myself for fun in other personal projects.

The API is still under reflexion and may evolve rapidly and abruptly. 

As it's first use with petgraph you can use the `petgraph` feature to allow simplified conversion from DotGraph to petgraph::StableGraph.

Standard usage: 

```
let graph : DotGraph = DotGraph::graph_from_file("./graph.dot").unwrap();

let as_dot_content : String = graph.write("./graph.dot");  
```


Feel free to open issues. 