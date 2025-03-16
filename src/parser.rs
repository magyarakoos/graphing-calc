use serde_json::Value;
use std::collections::HashMap;
use std::{iter::Peekable, str::Chars};

use super::config::read_json;

enum FunctionType {
    Prefix,
    Infix,
}

struct UnaryFunction {
    func: Box<dyn Fn(f32) -> f32>,
    name: String,
    r#type: FunctionType,
}

struct BinaryFunction {
    func: Box<dyn Fn(f32, f32) -> f32>,
    name: String,
    r#type: FunctionType,
}

enum Function {
    Unary(UnaryFunction),
    Binary(BinaryFunction),
}

enum Token {
    Number(f32),
    Function(Function),
    X,
    OpenParen,
    CloseParen,
}

fn get_operators() -> HashMap<String, Function> {
    let mut operators = HashMap::new();

    operators.insert(
        "+".to_string(),
        Function::Binary(BinaryFunction {
            func: Box::new(|x, y| x + y),
            name: "+".to_string(),
            r#type: FunctionType::Infix,
        }),
    );
    operators.insert(
        "-".to_string(),
        Function::Binary(BinaryFunction {
            func: Box::new(|x, y| x - y),
            name: "-".to_string(),
            r#type: FunctionType::Infix,
        }),
    );
    operators.insert(
        "*".to_string(),
        Function::Binary(BinaryFunction {
            func: Box::new(|x, y| x * y),
            name: "*".to_string(),
            r#type: FunctionType::Infix,
        }),
    );
    operators.insert(
        "/".to_string(),
        Function::Binary(BinaryFunction {
            func: Box::new(|x, y| x / y),
            name: "/".to_string(),
            r#type: FunctionType::Infix,
        }),
    );
    operators.insert(
        "^".to_string(),
        Function::Binary(BinaryFunction {
            func: Box::new(|x, y| x.powf(y)),
            name: "^".to_string(),
            r#type: FunctionType::Infix,
        }),
    );

    operators.insert(
        "exp".to_string(),
        Function::Unary(UnaryFunction {
            func: Box::new(|x| x.exp()),
            name: "exp".to_string(),
            r#type: FunctionType::Prefix,
        }),
    );
    operators.insert(
        "sin".to_string(),
        Function::Unary(UnaryFunction {
            func: Box::new(|x| x.sin()),
            name: "sin".to_string(),
            r#type: FunctionType::Prefix,
        }),
    );
    operators.insert(
        "cos".to_string(),
        Function::Unary(UnaryFunction {
            func: Box::new(|x| x.cos()),
            name: "cos".to_string(),
            r#type: FunctionType::Prefix,
        }),
    );
    operators.insert(
        "tan".to_string(),
        Function::Unary(UnaryFunction {
            func: Box::new(|x| x.tan()),
            name: "tan".to_string(),
            r#type: FunctionType::Prefix,
        }),
    );
    operators.insert(
        "sqrt".to_string(),
        Function::Unary(UnaryFunction {
            func: Box::new(|x| x.sqrt()),
            name: "sqrt".to_string(),
            r#type: FunctionType::Prefix,
        }),
    );
    operators.insert(
        "ln".to_string(),
        Function::Unary(UnaryFunction {
            func: Box::new(|x| x.ln()),
            name: "ln".to_string(),
            r#type: FunctionType::Prefix,
        }),
    );

    operators
}

pub struct Parser {
    pub constants: Value,
    operators: HashMap<String, Function>,
}

impl Parser {
    pub fn new(constants_path: &str) -> Self {
        let constants = read_json(constants_path)
            .map_err(|e| {
                panic!("Problem parsing the JSON: {e:?}");
            })
            .unwrap();

        Self {
            constants,
            operators: get_operators(),
        }
    }

    fn read_while<F>(it: &mut Peekable<Chars<'_>>, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut str = String::new();
        while it.peek().is_some() && predicate(*it.peek().unwrap()) {
            str.push(it.next().unwrap());
        }
        str
    }

    fn read_number(it: &mut Peekable<Chars<'_>>) -> Option<Token> {
        Some(Token::Number(
            Self::read_while(it, Self::is_digit).parse().ok()?,
        ))
    }

    fn read_function(&self, it: &mut Peekable<Chars<'_>>) -> Option<Token> {
        let str = Self::read_while(it, Self::is_function);

        if str == "x" {
            Some(Token::X)
        } else if Self::is_constant(self, &str) {
            Some(Token::Number(self.constants[str].as_f64()? as f32))
        } else if Self::is_operator(self, &str) {
            let ch = *it.peek()?;
            if ch != '(' {
                None
            } else {
                let func = self.operators.get(&str)?.clone();
                match func {
                    Function::Unary(unary_func) => {
                        Some(Token::Function(Function::Unary(*unary_func)))
                    }
                    Function::Binary(binary_func) => {
                        Some(Token::Function(Function::Binary(*binary_func)))
                    }
                }
            }
        } else {
            None
        }
    }

    fn is_constant(&self, str: &str) -> bool {
        !self.constants[str].is_null()
    }

    fn is_operator(&self, str: &str) -> bool {
        self.operators.contains_key(str)
    }

    fn is_whitespace(c: char) -> bool {
        ", \r\n\t".contains(c)
    }

    fn is_digit(c: char) -> bool {
        "0123456789.".contains(c)
    }

    fn is_alpha(c: char) -> bool {
        "abcdefghijklmnopqrstuvwxyz".contains(c)
    }

    fn is_primitive_function(c: char) -> bool {
        "+-*/^".contains(c)
    }

    fn is_function(c: char) -> bool {
        Self::is_alpha(c) || Self::is_primitive_function(c)
    }

    fn read_next(&self, it: &mut Peekable<Chars<'_>>) -> Option<Token> {
        Self::read_while(it, Self::is_whitespace);
        let ch = *it.peek()?;

        if Self::is_digit(ch) {
            Self::read_number(it)
        } else if Self::is_function(ch) {
            Self::read_function(&self, it)
        } else if ch == '(' {
            Some(Token::OpenParen)
        } else if ch == ')' {
            Some(Token::CloseParen)
        } else {
            None
        }
    }
}
