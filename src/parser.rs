use serde_json::Value;
use std::collections::HashMap;
use std::{iter::Peekable, str::Chars};

use super::config::read_json;
use super::shunting_yard::evaluate_postfix;
use super::shunting_yard::infix_to_postfix;

pub enum FunctionBox {
    Unary(Box<dyn Fn(f32) -> f32>),
    Binary(Box<dyn Fn(f32, f32) -> f32>),
}

pub struct Function {
    pub func: FunctionBox,
    pub name: String,
    pub precedence: i32,
}

pub enum Token<'a> {
    Number(f32),
    Function(&'a Function),
    X,
    OpenParen,
    CloseParen,
}

fn get_operators() -> HashMap<String, Function> {
    let mut operators = HashMap::new();

    operators.insert(
        "+".to_string(),
        Function {
            func: FunctionBox::Binary(Box::new(|x, y| x + y)),
            name: "+".to_string(),
            precedence: 1,
        },
    );
    operators.insert(
        "-".to_string(),
        Function {
            func: FunctionBox::Binary(Box::new(|x, y| x - y)),
            name: "-".to_string(),
            precedence: 1,
        },
    );
    operators.insert(
        "*".to_string(),
        Function {
            func: FunctionBox::Binary(Box::new(|x, y| x * y)),
            name: "*".to_string(),
            precedence: 2,
        },
    );
    operators.insert(
        "/".to_string(),
        Function {
            func: FunctionBox::Binary(Box::new(|x, y| x / y)),
            name: "/".to_string(),
            precedence: 2,
        },
    );
    operators.insert(
        "^".to_string(),
        Function {
            func: FunctionBox::Binary(Box::new(|x, y| x.powf(y))),
            name: "^".to_string(),
            precedence: 3,
        },
    );
    operators.insert(
        "exp".to_string(),
        Function {
            func: FunctionBox::Unary(Box::new(|x| x.exp())),
            name: "exp".to_string(),
            precedence: 4,
        },
    );
    operators.insert(
        "sin".to_string(),
        Function {
            func: FunctionBox::Unary(Box::new(|x| x.sin())),
            name: "sin".to_string(),
            precedence: 4,
        },
    );
    operators.insert(
        "cos".to_string(),
        Function {
            func: FunctionBox::Unary(Box::new(|x| x.cos())),
            name: "cos".to_string(),
            precedence: 4,
        },
    );
    operators.insert(
        "tan".to_string(),
        Function {
            func: FunctionBox::Unary(Box::new(|x| x.tan())),
            name: "tan".to_string(),
            precedence: 4,
        },
    );
    operators.insert(
        "sqrt".to_string(),
        Function {
            func: FunctionBox::Unary(Box::new(|x| x.sqrt())),
            name: "sqrt".to_string(),
            precedence: 4,
        },
    );
    operators.insert(
        "ln".to_string(),
        Function {
            func: FunctionBox::Unary(Box::new(|x| x.ln())),
            name: "ln".to_string(),
            precedence: 4,
        },
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

    fn read_number<'a>(it: &mut Peekable<Chars<'a>>) -> Option<Token<'a>> {
        Some(Token::Number(
            Self::read_while(it, Self::is_digit).parse().ok()?,
        ))
    }

    fn read_function<F>(&self, it: &mut Peekable<Chars<'_>>, predicate: F) -> Option<Token>
    where
        F: Fn(char) -> bool,
    {
        let str = Self::read_while(it, predicate);

        if str == "x" {
            Some(Token::X)
        } else if self.is_constant(&str) {
            Some(Token::Number(self.constants[str].as_f64()? as f32))
        } else if self.is_operator(&str) {
            let ch = *it.peek()?;
            if !Self::is_primitive_function(str.chars().nth(0)?) && ch != '(' {
                None
            } else {
                Some(Token::Function(self.operators.get(&str)?))
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

    fn read_next<'a>(&'a self, it: &mut Peekable<Chars<'a>>) -> Option<Token<'a>> {
        Self::read_while(it, Self::is_whitespace);
        let ch = *it.peek()?;

        if Self::is_digit(ch) {
            Self::read_number(it)
        } else if Self::is_alpha(ch) {
            self.read_function(it, Self::is_alpha)
        } else if Self::is_primitive_function(ch) {
            self.read_function(it, Self::is_primitive_function)
        } else if ch == '(' {
            it.next();
            Some(Token::OpenParen)
        } else if ch == ')' {
            it.next();
            Some(Token::CloseParen)
        } else {
            None
        }
    }

    fn tokenize<'a>(&'a self, str: &'a str) -> Option<Vec<Token<'a>>> {
        let mut it = str.chars().peekable();
        let mut vec: Vec<Token> = Vec::new();
        loop {
            if let Some(token) = self.read_next(&mut it) {
                vec.push(token);
            } else {
                if it.peek().is_none() {
                    break;
                }
            }
        }
        infix_to_postfix(vec)
    }

    // parses a formula and generates a function for it
    pub fn parse<'a>(&'a self, str: &'a str) -> Option<Box<dyn Fn(f32) -> f32 + 'a>> {
        if let Some(tokens) = self.tokenize(str) {
            evaluate_postfix(&tokens, 0.0)?;
            Some(Box::new(move |x: f32| {
                evaluate_postfix(&tokens, x).unwrap()
            }))
        } else {
            None
        }
    }
}
