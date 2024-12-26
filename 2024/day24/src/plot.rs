use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Dot, Config};
use std::collections::HashMap;
use std::process::Command;
use std::fmt;
use petgraph::visit::EdgeRef;

struct LabeledGraph<'a>(pub &'a DiGraph<String, String>);

impl<'a> fmt::Display for LabeledGraph<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "digraph {{\n")?;
        write!(f, "    node [shape=box, style=filled];\n")?;
        
        for node_idx in self.0.node_indices() {
            let node_name = &self.0[node_idx];
            let mut fillcolor = if node_name.starts_with('z') {
                "lightcoral" 
            } else if node_name.starts_with('y') || node_name.starts_with('x') {
                "lightgreen"
            } else {
                "lightblue"
            };

            for edge in self.0.edges_directed(node_idx, petgraph::Direction::Incoming) {
                let source_name = &self.0[edge.source()];
                if source_name.starts_with('x') || source_name.starts_with('y') {
                    fillcolor = "yellow"; 
                    break;
                }
            }

            write!(f, "    {} [label=\"{}\", fillcolor={}];\n", node_idx.index(), node_name, fillcolor)?;
        }
        
        for edge in self.0.edge_references() {
            write!(f, "    {} -> {};\n",
                edge.source().index(),
                edge.target().index()
            )?;
        }
        write!(f, "}}")
    }
}

fn main() -> std::io::Result<()> {
    let mut graph = DiGraph::new();
    let mut node_indices = HashMap::new();

    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("->") {
            let parts: Vec<&str> = line.split("->").collect();
            let inputs = parts[0].trim();
            let output = parts[1].trim();

            if !node_indices.contains_key(output) {
                let idx = graph.add_node(output.to_string());
                node_indices.insert(output.to_string(), idx);
            }

            for input in inputs.split_whitespace() {
                if input != "AND" && input != "OR" && input != "XOR" {
                    if !node_indices.contains_key(input) {
                        let idx = graph.add_node(input.to_string());
                        node_indices.insert(input.to_string(), idx);
                    }
                }
            }
        }
    }

    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("->") {
            let parts: Vec<&str> = line.split("->").collect();
            let inputs = parts[0].trim();
            let output = parts[1].trim();

            let output_idx = node_indices[output];
            let input_parts: Vec<&str> = inputs.split_whitespace().collect();

            if input_parts.len() == 3 {
                let input1 = input_parts[0];
                let input2 = input_parts[2];
                let op = input_parts[1];

                let input1_idx = node_indices[input1];
                let input2_idx = node_indices[input2];

                graph.add_edge(input1_idx, output_idx, "".to_string());
                graph.add_edge(input2_idx, output_idx, "".to_string());

                let new_label = format!("{} {}", output, op);
                graph[output_idx] = new_label;
            }
        }
    }

    let dot = format!("{}", LabeledGraph(&graph));

    std::fs::write("circuit.dot", dot)?;

    // Generate PNG using graphviz (requires graphviz to be installed)
    Command::new("dot")
        .args(["-Tpng", "circuit.dot", "-o", "circuit.png"])
        .output()?;

    println!("Circuit visualization has been saved to circuit.png");
    Ok(())
}