use crate::eval;
use crate::shunting_yard;
use crate::tree;

use std::io:: {self, Write };

fn flush() {
 let result = io::stdout().flush();

 if result.is_err() {
  panic!("Failed to flush!!");
 }
}

fn treat_input(input: &mut String) {
 *input = input.trim().to_string();
 *input = input.replace(" ", "");
}

pub fn start() {
 
 println!("----------- CALCULATOR -----------");
 println!("q - to quit");

 loop {
  print!("Expression: ");
  
  flush();

  let mut input = String::new();
  let result = io::stdin().read_line(&mut input);
   
  if result.is_err() {
   println!("Failed to read line!");
   continue;
  }

  treat_input(&mut input);

  let output_stack = shunting_yard::exec(input.clone());

  let mut node_stack = tree::NodeStack::new();
  let result = node_stack.mount_tree(output_stack);

  if result.is_err() {
   println!("{result:?}");
   continue;
  }

  let root_node = result.unwrap();

  let calc_result = eval::exec(root_node);

  if calc_result.is_err() {
   println!("{calc_result:?}");
   continue;
  }
  
  let result_value = calc_result.unwrap();

  println!("Result: {result_value}");

  if input.trim() == "q" {
   break;
  }

 }

 println!("shutting down...");

}
