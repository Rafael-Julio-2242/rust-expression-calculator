use thiserror::Error;

#[derive(Error, Debug)]
pub enum MountTreeError {
    #[error("'{0}' is not a valid operator")]
    InvalidOperator(String),

    #[error("Multiple Root Nodes!")]
    MultipleRootNodes,

    #[error("Nodes Tree is not empty")]
    NodesTreeNotEmpty,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub value: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Clone, Debug)]
pub struct NodeStack {
    pub nodes: Vec<Node>,
}

impl NodeStack {
    pub fn new() -> NodeStack {
        NodeStack { nodes: Vec::<Node>::new() }
    }

    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.nodes.pop()
    }

    pub fn peek(&self) -> Option<&Node> {
        self.nodes.last()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn mount_tree(&mut self, postfix_expression: Vec<String>) -> Result<Node, MountTreeError> {
        if !self.nodes.is_empty() {
            return Err(MountTreeError::NodesTreeNotEmpty);
        }

        let operators_range = "+-/*";

        for value in postfix_expression {
            if operators_range.contains(&value) {
                let right = self.nodes.pop().expect("Error poping right value!");
                let left = self.nodes.pop().expect("Error poping left value!");

                let operator_node = Node {
                    value: value,
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                };

                self.nodes.push(operator_node);

                continue;
            }

            // Preciso verificar se isso é um número
            let is_number = value.parse::<f64>().is_ok();

            if !is_number {
                return Err(MountTreeError::InvalidOperator(value));
            }

            let number_node = Node {
                value: value,
                left: None,
                right: None,
            };

            self.nodes.push(number_node);
        }

        if self.nodes.len() > 1 {
            return Err(MountTreeError::MultipleRootNodes);
        }

        let root_node = self.nodes.pop().expect("Error poping root result node!");

        Ok(root_node)
    }
}