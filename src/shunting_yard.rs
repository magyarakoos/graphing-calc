use crate::parser::FunctionBox::*;
use crate::parser::Token;
use crate::parser::Token::*;
use anyhow::Result;
use anyhow::anyhow;
use itertools::Itertools;

pub fn infix_to_postfix<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<Token<'a>>> {
    let mut result: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Number(_) | X => result.push(token),
            Function(fun) => {
                loop {
                    if operator_stack.is_empty() {
                        break;
                    }
                    match operator_stack.last().unwrap() {
                        Function(prev_fun) => {
                            let prev_fun = *prev_fun;
                            if prev_fun.precedence < fun.precedence {
                                break;
                            }
                            result.push(operator_stack.pop().unwrap());
                        }
                        OpenParen => {
                            break;
                        }
                        _ => unreachable!(),
                    }
                }
                operator_stack.push(token);
            }
            OpenParen => operator_stack.push(token),
            CloseParen => loop {
                if operator_stack.is_empty() {
                    return Err(anyhow!(
                        "Mismatched parentheses, no opening paren found for closing paren"
                    ));
                }
                match operator_stack.last().unwrap() {
                    Function(_) => result.push(operator_stack.pop().unwrap()),
                    OpenParen => {
                        operator_stack.pop();
                        break;
                    }
                    _ => unreachable!(),
                }
            },
        }
    }

    while !operator_stack.is_empty() {
        match operator_stack.last().unwrap() {
            Function(_) => result.push(operator_stack.pop().unwrap()),
            OpenParen => (), // ignore unclosed parens
            _ => unreachable!(),
        }
    }

    Ok(result)
}

pub fn evaluate_postfix<'a>(tokens: &Vec<Token<'a>>, x: f64) -> Result<f64> {
    let mut stack: Vec<f64> = Vec::new();
    for token in tokens {
        match token {
            Number(num) => stack.push(*num),
            X => stack.push(x),
            Function(fun) => match &fun.func {
                Unary(f) => {
                    if stack.is_empty() {
                        return Err(anyhow!(format!(
                            "Function {} expects 1 argument, but none was provided",
                            fun.name
                        )));
                    }
                    let a = stack.pop().unwrap();
                    stack.push(f(a));
                }
                Binary(f) => {
                    // this is negation
                    if stack.len() == 1 && fun.name == "-" {
                        let a = stack.pop().unwrap();
                        stack.push(f(0.0, a));
                    }

                    if stack.len() < 2 {
                        return Err(anyhow!(format!(
                            "Function {} expects 2 arguments, but {} was provided",
                            fun.name,
                            stack.len()
                        )));
                    }
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(f(a, b));
                }
            },
            _ => unreachable!(),
        }
    }
    if stack.is_empty() {
        Err(anyhow!("The formula didn't produce a value"))
    } else if stack.len() != 1 {
        Err(anyhow!(format!(
            "Invalid formula, {} items were left on the stack: [{}]",
            stack.len(),
            stack.iter().format(", ")
        )))
    } else {
        Ok(*stack.last().unwrap())
    }
}
