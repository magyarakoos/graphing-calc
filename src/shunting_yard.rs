use super::parser::FunctionBox::*;
use super::parser::Token;
use super::parser::Token::*;

pub fn infix_to_postfix<'a>(tokens: Vec<Token<'a>>) -> Option<Vec<Token<'a>>> {
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
                    match operator_stack.last()? {
                        Function(prev_fun) => {
                            let prev_fun = *prev_fun;
                            if prev_fun.precedence < fun.precedence {
                                break;
                            }
                            result.push(operator_stack.pop()?);
                        }
                        OpenParen => {
                            break;
                        }
                        _ => None?,
                    }
                }
                operator_stack.push(token);
            }
            OpenParen => operator_stack.push(token),
            CloseParen => loop {
                match operator_stack.last()? {
                    Function(_) => result.push(operator_stack.pop()?),
                    OpenParen => {
                        operator_stack.pop();
                        break;
                    }
                    _ => None?,
                }
            },
        }
    }

    while !operator_stack.is_empty() {
        match operator_stack.last()? {
            Function(_) => result.push(operator_stack.pop()?),
            _ => None?,
        }
    }

    Some(result)
}

pub fn evaluate_postfix<'a>(tokens: &Vec<Token<'a>>, x: f32) -> Option<f32> {
    let mut stack: Vec<f32> = Vec::new();
    for token in tokens {
        match token {
            Number(num) => stack.push(*num),
            X => stack.push(x),
            Function(fun) => match &fun.func {
                Unary(f) => {
                    let a = stack.pop()?;
                    stack.push(f(a));
                }
                Binary(f) => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(f(a, b));
                }
            },
            _ => None?,
        }
    }
    if stack.len() != 1 {
        None
    } else {
        Some(*stack.last()?)
    }
}
