use super::parser::Token;

fn infix_to_postfix<'a>(tokens: &Vec<Token<'a>>) -> Vec<Token<'a>> {
    let mut result: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    result
}
