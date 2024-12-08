use std::fs::File;
use std::io::Write;

pub struct Dotfile {
    filename: String,
    nodes: Vec<DotNode>,
    edges: Vec<DotEdge>,
}
struct DotNode {
    idx: usize,
    label: String,
    color: String,
    font_color: String,
}

struct DotEdge {
    src_id: usize,
    dest_id: usize,
}

pub enum DotNodeColor {
    Red,
    Black,
    Green,
}


impl Dotfile {
    pub fn new(filename: &str) -> Self {
        Dotfile {
            filename: filename.to_string(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self, key: &str, node_color: DotNodeColor) -> usize {
        let current_len = self.nodes.len();
        let color = match node_color {
            DotNodeColor::Red => { "red" }
            DotNodeColor::Black => { "black" }
            DotNodeColor::Green => { "darkgreen"}
        };
        self.nodes.push(DotNode {
            idx: current_len,
            label: key.to_string(),
            color: color.to_string(),
            font_color: "white".to_string(),
        });
        current_len
    }

    pub fn add_edge(&mut self, key1: usize, key2: usize) {
        self.edges.push(DotEdge {
            src_id: key1,
            dest_id: key2,
        })
    }

    pub fn write_file(&self) {
        let mut dot_string = String::new();
        dot_string.push_str("graph {\n");

        for node in &self.nodes {
            dot_string.push_str(&format!("\t {} [label=\"{}\", color={}, style=filled, fontcolor={}];\n", node.idx, node.label, node.color, node.font_color))
        }
        for edge in &self.edges {
            dot_string.push_str(&format!("\t {} -- {};\n", edge.src_id, edge.dest_id))
        }
        dot_string.push_str("}\n");
        let mut dot_file = File::create(&self.filename).expect("Error while Creating file");
        dot_file.write_all(dot_string.as_bytes()).expect("W")
    }
}