
struct Node<'t> {
 value: String,
 left: Option<&'t Node<'t>>,
 right: Option<&'t Node<'t>>
}

struct NodeStack<'t> {
 nodes: Vec<Node<'t>>
}

impl<'t> NodeStack<'t> {
 fn push(&mut self, node: Node<'t>) {
  self.nodes.push(node);
 }

 fn pop(&mut self) -> Option<Node<'t>> {
  self.nodes.pop()
 }

 fn peek(&self) -> Option<&Node<'t>> {
  self.nodes.last()
 }

 fn len(&self) -> usize {
  self.nodes.len()
 }

 fn is_empty(&self) -> bool {
  self.nodes.is_empty()
 }
}

pub fn mount_tree(postfix_expression: Vec<String>) {
 
}