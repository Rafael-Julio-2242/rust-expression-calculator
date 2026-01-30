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

pub fn exec() {}

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
