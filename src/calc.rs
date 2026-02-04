use crate::shunting_yard;

use std::io:: {self, Write };

fn flush() {
 let result = io::stdout().flush();

 if result.is_err() {
  panic!("Failed to flush!!");
 }
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

  println!("Input: {input}");
  let output_stack = shunting_yard::exec(input.clone());
  println!("Output: {output_stack:?}");

  if input.trim() == "q" {
   break;
  }

 }

 println!("shutting down...");

}
