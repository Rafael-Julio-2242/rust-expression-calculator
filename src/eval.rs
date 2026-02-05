use crate::tree::Node;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {

 #[error("Operation cannot be a leaf!")]
 OperationCannotBeLeaf(String)

}

pub fn exec(node: Node) -> Result<f64, EvalError> {

 if node.left.is_none() && node.right.is_none() {

  let parse_result = node.value.parse::<f64>();

  if parse_result.is_err() {
   return Err(EvalError::OperationCannotBeLeaf(node.value))
  }

 } 

 Ok(0.0)
}

