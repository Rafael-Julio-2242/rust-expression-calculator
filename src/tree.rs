use thiserror::Error;

#[derive(Error, Debug)]
pub enum MountTreeError {
    #[error("'{0}' is not a valid operator")]
    InvalidOperator(String),

    #[error("Multiple Root Nodes!")]
    MultipleRootNodes,
}

#[derive(Clone)]
pub struct Node {
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Clone)]
pub struct NodeStack {
    nodes: Vec<Node>,
}

impl<'t> NodeStack {
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
}

pub fn mount_tree(postfix_expression: Vec<String>) -> Result<Node, MountTreeError> {
    let mut node_stack = NodeStack { nodes: Vec::new() };

    let operators_range = "+-/*";

    for value in postfix_expression {
        if operators_range.contains(&value) {
            let right = node_stack.pop().expect("Error poping right value!");
            let left = node_stack.pop().expect("Error poping left value!");

            let operator_node = Node {
                value: value,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };

            node_stack.push(operator_node);
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

        node_stack.push(number_node);
    }

    if node_stack.len() > 1 {
        return Err(MountTreeError::MultipleRootNodes);
    }

    let root_node = node_stack.pop().expect("Error poping root result node!");

    Ok(root_node)
}
