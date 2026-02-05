use crate::tree::Node;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {

 #[error("Operation cannot be a leaf: {0}")]
 OperationCannotBeLeaf(String),

 #[error("One of the operations is Incomplete: {0}")]
 OneOfTheValuesIsEmpty(String),

 #[error("There was an error making calculations for: {0}")]
 CalculationError(String),

 #[error("Division by 0 is not allowed: {0} / {1}")]
 DivisionByZero(String, String)
}

pub fn exec(node: Node) -> Result<f64, EvalError> {

 if node.left.is_none() && node.right.is_none() {

  let parse_result = node.value.parse::<f64>();

  if parse_result.is_err() {
   return Err(EvalError::OperationCannotBeLeaf(node.value))
  }

  let number = parse_result.unwrap();

  return Ok(number);
 }

 if node.left.is_none() || node.right.is_none() {
  return Err(EvalError::OneOfTheValuesIsEmpty(node.value));
 }

 let left_node = node.left.unwrap().as_ref().clone();

 let left_result = exec(left_node.clone());

 if left_result.is_err() {
  return Err(EvalError::CalculationError(left_node.value));
 }

 let left_value = left_result.unwrap();

 let right_node = node.right.unwrap().as_ref().clone();

 let right_result = exec(right_node.clone());

 if right_result.is_err() {
  return Err(EvalError::CalculationError(right_node.value));
 }

 let right_value = right_result.unwrap();


 match node.value.as_str() {
  "+" => return Ok(left_value + right_value),
  "-" => return Ok(left_value - right_value),
  "*" => return Ok(left_value * right_value),
  "/" => {
   if right_value == 0.0 {
    return Err(EvalError::DivisionByZero(left_node.value, right_node.value));
   }
   return Ok(left_value / right_value);
  }
  _ => {
   let number_value_result = node.value.parse::<f64>();

   if number_value_result.is_err() {
    return Err(EvalError::CalculationError(node.value));
   }

   let number_value = number_value_result.unwrap();

   return Ok(number_value);
  }
 }
}

