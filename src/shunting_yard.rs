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

pub fn exec(expression: String) {
    let mut operator_stack: Vec<String> = Vec::<String>::new();
    let mut output_stack: Vec<String> = Vec::<String>::new();

    let numeric_range = String::from("123456789");
    let mut current_numbers: String = String::from("");

    for char in expression.chars() {
        let char_str = char.to_string();

        if numeric_range.contains(&char_str) {
            current_numbers.push_str(&char_str);
            // Preciso validar se ele é o ultimo caracter da string
            let last_char = expression
                .chars()
                .last()
                .expect("Error on getting the last value of expression!")
                .to_string();

            if last_char.eq(&char_str) {
                output_stack.push(current_numbers.clone());
                current_numbers = String::from("");
            }
            continue;
        }

        let precedence_result = get_precedence_info(&char_str)
            .expect(&format!("Error on getting precedence info for {char_str}"));

        if precedence_result.precedence == -2 { // Fechamento de parenteses
            // TODO Desenvolver essa parte do algoritmo

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

        if last_value_precedence_result.precedence == -1 { // Topo da pilha é um parêntese
            operator_stack.push(char_str);
            continue;
        }

        // TODO Continuar construção

    }
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
