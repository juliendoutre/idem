#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub label: String,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} [label=\"{}\"];", self.id, self.label)
    }
}

#[derive(Debug)]
pub struct Edge {
    pub source: String,
    pub destination: String,
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {};", self.source, self.destination)
    }
}

#[derive(Debug)]
pub struct Digraph {
    pub title: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Digraph {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl std::fmt::Display for Digraph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let nodes_str = self
            .nodes
            .iter()
            .map(|n| format!("\t{}", n))
            .collect::<Vec<String>>()
            .join("\n");

        let edges_str = self
            .edges
            .iter()
            .map(|e| format!("\t{}", e))
            .collect::<Vec<String>>()
            .join("\n");

        write!(
            f,
            "digraph \"{}\" {{\n{}\n{}\n}}",
            self.title, nodes_str, edges_str
        )
    }
}
