pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;

    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs.extend(
                attrs
                    .iter()
                    .map(|(name, value)| (name.to_string(), value.to_string())),
            );
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| name == node.name)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use super::node::Node;
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub n1: Node,
                pub n2: Node,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(n1: &str, n2: &str) -> Self {
                    Self {
                        n1: Node::new(n1),
                        n2: Node::new(n2),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(
                        attrs
                            .iter()
                            .map(|(name, value)| (name.to_string(), value.to_string())),
                    );
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(String::as_str)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(
                        attrs
                            .iter()
                            .map(|(name, value)| (name.to_string(), value.to_string())),
                    );
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(String::as_str)
                }
            }
        }
    }
}
