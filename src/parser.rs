use anyhow::{Result, anyhow};
use itertools::Itertools;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Error;
use std::{iter::Peekable, str::Chars};

use crate::log;
use crate::shunting_yard::evaluate_postfix;
use crate::shunting_yard::infix_to_postfix;

pub enum FunctionBox {
    Unary(Box<dyn Fn(f64) -> f64 + Sync + Send>),
    Binary(Box<dyn Fn(f64, f64) -> f64 + Sync + Send>),
}

pub struct Function {
    pub func: FunctionBox,
    pub name: String,
    pub precedence: i32,
}

pub enum Token<'a> {
    Number(f64),
    Function(&'a Function),
    X,
    OpenParen,
    CloseParen,
    Comma,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Function(fun) => write!(f, "{}", fun.name),
            Self::X => write!(f, "x"),
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::Comma => write!(f, ","),
        }
    }
}

pub struct Parser {
    functions: HashMap<String, Function>,
    constants: HashMap<String, f64>,
}

impl Parser {
    pub fn new<'a>(functions_json: Value, constants_json: Value) -> Result<Self> {
        Ok(Parser {
            functions: Self::get_functions(functions_json)?,
            constants: Self::get_constants(constants_json)?,
        })
    }

    fn get_functions<'a>(json: Value) -> Result<HashMap<String, Function>> {
        let json = json.as_array().ok_or(anyhow!("Invalid Config format"))?;

        let mut functions = HashMap::new();

        #[derive(Deserialize)]
        struct FunctionData {
            name: String,
            r#type: String,
            precedence: i32,
        }

        for func_data in json {
            let func_data: FunctionData = serde_json::from_value(func_data.clone())?;

            let function = Function {
                func: match func_data.r#type.as_str() {
                    "unary" => {
                        let f: fn(f64) -> f64 = match func_data.name.as_str() {
                            "exp" => f64::exp,
                            "sin" => f64::sin,
                            "cos" => f64::cos,
                            "tan" => f64::tan,
                            "sqrt" => f64::sqrt,
                            "ln" => f64::ln,
                            _ => {
                                return Err(anyhow!(format!(
                                    "Function {} is not implemented",
                                    func_data.name
                                )));
                            }
                        };
                        FunctionBox::Unary(Box::new(f))
                    }
                    "binary" => {
                        let f: fn(f64, f64) -> f64 = match func_data.name.as_str() {
                            "+" => |x, y| x + y,
                            "-" => |x, y| x - y,
                            "*" => |x, y| x * y,
                            "/" => |x, y| x / y,
                            "^" => f64::powf,
                            "max" => f64::max,
                            _ => {
                                return Err(anyhow!(format!(
                                    "Function {} is not implemented",
                                    func_data.name
                                )));
                            }
                        };
                        FunctionBox::Binary(Box::new(f))
                    }
                    _ => {
                        return Err(anyhow!(format!(
                            "Unknown function type: {}",
                            func_data.r#type
                        )));
                    }
                },
                name: func_data.name.clone(),
                precedence: func_data.precedence,
            };

            functions.insert(func_data.name, function);
        }

        Ok(functions)
    }

    fn get_constants<'a>(json: Value) -> Result<HashMap<String, f64>> {
        let json = json.as_object().ok_or(anyhow!("Invalid Config format"))?;

        let mut constants = HashMap::new();

        for (name, value) in json {
            let num = value
                .as_f64()
                .ok_or(anyhow!(format!("{} is not a number: {}", name, value)))?;
            constants.insert(name.clone(), num);
        }

        Ok(constants)
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

    fn read_number<'a>(it: &mut Peekable<Chars<'a>>) -> Result<Token<'a>> {
        Ok(Token::Number(Self::read_while(it, Self::is_digit).parse()?))
    }

    fn is_digit(c: char) -> bool {
        "0123456789.".contains(c)
    }

    fn is_constant(&self, str: &str) -> bool {
        self.constants.contains_key(str)
    }

    fn is_function(&self, str: &str) -> bool {
        self.functions.contains_key(str)
    }

    fn is_primitive(c: char) -> bool {
        "+-*/^".contains(c)
    }

    fn is_whitespace(c: char) -> bool {
        " \r\n\t".contains(c)
    }

    fn is_alpha(c: char) -> bool {
        "abcdefghijklmnopqrstuvwxyz".contains(c)
    }

    fn read_function<'a>(&'a self, it: &mut Peekable<Chars<'_>>) -> Result<Token<'a>> {
        let ch = *it.peek().ok_or(anyhow!("EOF reached"))?;
        let str = if Self::is_primitive(ch) {
            it.next().unwrap().to_string()
        } else {
            Self::read_while(it, Self::is_alpha)
        };

        if str == "x" {
            Ok(Token::X)
        } else if self.is_constant(&str) {
            Ok(Token::Number(*self.constants.get(&str).unwrap()))
        } else if self.is_function(&str) {
            Ok(Token::Function(self.functions.get(&str).unwrap()))
        } else {
            Err(anyhow!(format!("Unknown expression: {}", str)))
        }
    }

    fn read_next<'a>(&'a self, it: &mut Peekable<Chars<'a>>) -> Result<Token<'a>> {
        Self::read_while(it, Self::is_whitespace);
        let ch = *it.peek().ok_or(anyhow!("Reached EOF"))?;

        if Self::is_digit(ch) {
            Self::read_number(it)
        } else if Self::is_alpha(ch) || Self::is_primitive(ch) {
            self.read_function(it)
        } else if ch == '(' {
            it.next();
            Ok(Token::OpenParen)
        } else if ch == ')' {
            it.next();
            Ok(Token::CloseParen)
        } else if ch == ',' {
            it.next();
            Ok(Token::Comma)
        } else {
            Err(anyhow!(format!("Unknown symbol: {}", ch)))
        }
    }

    fn insert_multiplications<'a>(&'a self, tokens: Vec<Token<'a>>) -> Result<Vec<Token<'a>>> {
        let mut result: Vec<Token> = Vec::new();
        for token in tokens {
            if !result.is_empty() {
                match (result.last().unwrap(), &token) {
                    (Token::Number(_) | Token::X, Token::Number(_) | Token::X) => {
                        result.push(Token::Function(
                            self.functions
                                .get("*")
                                .ok_or(anyhow!("Multiplication isn't defined"))?,
                        ))
                    }
                    _ => (),
                }
            }
            result.push(token);
        }
        Ok(result)
    }

    fn fix_negations<'a>(&'a self, tokens: Vec<Token<'a>>) -> Result<Vec<Token<'a>>> {
        let mut result: Vec<Token> = Vec::new();

        if tokens.is_empty() {
            return Ok(result);
        }

        // handle the first case separately
        if let Token::Function(f) = tokens.first().unwrap() {
            if f.name == "-" {
                result.push(Token::Number(0.0));
            }
        }

        for token in tokens {
            if !result.is_empty() {
                match (result.last().unwrap(), &token) {
                    (Token::Function(_) | Token::OpenParen | Token::Comma, Token::Function(f)) => {
                        if f.name == "-" {
                            result.push(Token::Number(0.0));
                        }
                    }
                    _ => (),
                }
            }
            result.push(token);
        }

        Ok(result)
    }

    fn tokenize_to_postfix<'a>(&'a self, str: &'a str) -> Result<Vec<Token<'a>>> {
        let mut it = str.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = self.read_next(&mut it);
            match token {
                Ok(token) => {
                    tokens.push(token);
                    println!("{}", tokens.last().unwrap());
                }
                Err(e) => {
                    println!("No token");
                    if it.peek().is_none() {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        // make implicit multiplications explicit
        let tokens = self.insert_multiplications(tokens)?;
        let tokens = self.fix_negations(tokens)?;
        log(&format!(
            "Done tokenizing, items: [{}]",
            tokens.iter().format(" ")
        ));
        infix_to_postfix(tokens)
    }

    pub fn parse<'a>(&'a self, str: &'a str) -> Result<Box<dyn Fn(f64) -> f64 + 'a>> {
        let tokens = self.tokenize_to_postfix(str)?;
        log(&format!(
            "Done conversion, items: [{}]",
            tokens.iter().format(" ")
        ));
        for t in &tokens {
            println!("{}", t);
        }
        evaluate_postfix(&tokens, 0.0)?;
        Ok(Box::new(move |x: f64| {
            evaluate_postfix(&tokens, x).unwrap()
        }))
    }
}
