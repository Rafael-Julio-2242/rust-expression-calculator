use thiserror::Error;

#[derive(Error, Debug)]
enum PrecedenceError {
    #[error("Invalid operator informed: {0}")]
    InvalidOperatior(String),
}

struct PrecedenceResult {
    precedence: i32,
    str_value: String,
}

pub fn exec(expression: String) -> Vec<String> {
    let mut operator_stack: Vec<String> = Vec::<String>::new();
    let mut output_stack: Vec<String> = Vec::<String>::new();

    let numeric_range = String::from("123456789");
    let mut current_number: String = String::from("");

    for char in expression.chars() {
        let char_str = char.to_string();

        if numeric_range.contains(&char_str) {
            current_number.push_str(&char_str);
            // Preciso validar se ele é o ultimo caracter da string
            let last_char = expression
                .chars()
                .last()
                .expect("Error on getting the last value of expression!")
                .to_string();

            if last_char.eq(&char_str) {
                output_stack.push(current_number.clone());
                current_number = String::from("");
            }
            continue;
        } else {
            output_stack.push(current_number.clone());
            current_number = String::from("");
        }

        let precedence_result = get_precedence_info(&char_str)
            .expect(&format!("Error on getting precedence info for {char_str}"));

        if precedence_result.precedence == -2 {
            loop {
                let current_top = operator_stack[operator_stack.len() - 1].clone();

                if current_top.eq(&"(") {
                    operator_stack.pop();
                    break;
                }

                operator_stack.pop();
                output_stack.push(current_top);
            }

            continue;
        }

        if operator_stack.len() == 0 {
            operator_stack.push(char_str);
            continue;
        }

        if precedence_result.precedence == -1 {
            operator_stack.push(char_str);
            continue;
        }

        let last_operator_stack_index = operator_stack.len() - 1;
        let last_operator_stack_value = operator_stack[last_operator_stack_index].clone();

        let last_value_precedence_result = get_precedence_info(&last_operator_stack_value)
            .expect(&format!("Error on getting precedence info (last operator stack) for {last_operator_stack_value}"));

        if last_value_precedence_result.precedence == -1 {
            // Topo da pilha é um parêntese
            operator_stack.push(char_str);
            continue;
        }

        // Validação de precedência para notação posfixa
        let current_precedence = precedence_result.precedence;
        let last_precedence = last_value_precedence_result.precedence;

        let current_associativity = precedence_result.str_value;
        let last_associativity = last_value_precedence_result.str_value;

        if current_precedence > last_precedence
            || (current_precedence == last_precedence
                && current_associativity.eq(&last_associativity)
                && current_associativity.eq("right"))
        {
            operator_stack.push(char_str);
            continue;
        }

        if current_precedence < last_precedence
            || (current_precedence == last_precedence
                && current_associativity.eq(&last_associativity)
                && current_associativity.eq("left"))
        {
            let last_value = operator_stack.pop()
                .expect("Error Poping Operator Stack!");

            output_stack.push(last_value);
            

            while operator_stack.len() > 0 {
                let current_top = operator_stack[operator_stack.len() - 1].clone();

                let current_top_precedence_result = get_precedence_info(&current_top)
                    .expect("Error getting current_top precedence Result!");

                let current_top_precedence = current_top_precedence_result.precedence;
                let current_top_associativity = current_top_precedence_result.str_value;

                if current_top_precedence > current_precedence
                    || (current_top_precedence == current_precedence
                        && current_top_associativity.eq(&current_associativity)
                        && current_top_associativity.eq("right")) 
                    {

                    operator_stack.pop();
                    output_stack.push(current_top);

                } else {
                    break;
                }
            }
            operator_stack.push(char_str);
            continue;
        }

    }

    operator_stack
        .iter()
        .filter(|value: &&String| -> bool { !value.trim().is_empty() })
        .map(|value: &String| -> String { value.to_string() })
        .collect::<Vec<String>>()
        .clone_into(&mut operator_stack);

    if operator_stack.len() >= 1 {
        operator_stack.reverse();
        output_stack.append(&mut operator_stack);
    }

    output_stack
        .iter()
        .filter(|value: &&String| -> bool { !value.trim().is_empty() })
        .map(|value: &String| -> String { value.to_string() })
        .collect::<Vec<String>>()
        .clone_into(&mut operator_stack);

    
    output_stack
}

fn get_precedence_info(c: &str) -> Result<PrecedenceResult, PrecedenceError> {
    match c {
        "^" => {
            return Ok(PrecedenceResult {
                precedence: 4,
                str_value: String::from("right"),
            });
        }
        "*" => {
            return Ok(PrecedenceResult {
                precedence: 3,
                str_value: String::from("left"),
            });
        }
        "/" => {
            return Ok(PrecedenceResult {
                precedence: 3,
                str_value: String::from("left"),
            });
        }
        "+" => {
            return Ok(PrecedenceResult {
                precedence: 2,
                str_value: String::from("left"),
            });
        }
        "-" => {
            return Ok(PrecedenceResult {
                precedence: 2,
                str_value: String::from("left"),
            });
        }
        "(" => {
            return Ok(PrecedenceResult {
                precedence: -1,
                str_value: String::from("("),
            });
        }
        ")" => {
            return Ok(PrecedenceResult {
                precedence: -1,
                str_value: String::from(")"),
            });
        }
        _ => return Err(PrecedenceError::InvalidOperatior(String::from(c))),
    }
}
