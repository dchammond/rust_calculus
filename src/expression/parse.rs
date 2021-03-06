extern crate regex;
use regex::Regex;

use expression;
use expression::enums;

type Expression = expression::Expression;

pub fn parse_input(input: &String,
                   numeric_regex: &Regex,
                   function_regex: &Regex)
                   -> (String, Result<expression::Expression, String>) {
    let (variable, expr) = string_to_expr(input, numeric_regex, function_regex);
    convert_to_postfix(input, variable, expr)
}

fn string_to_expr(input: &String,
                  numeric_regex: &Regex,
                  function_regex: &Regex)
                  -> (String, Expression) {
    let mut variable: String = String::new();
    let mut expr: Expression = Expression::new(Vec::new());
    let mut builder: String = String::new();
    for c in input.chars() {
        match c {
            '(' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    builder = String::new();
                }
                expr.push(enums::Token::Open);
            }
            ')' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    builder = String::new();
                }
                expr.push(enums::Token::Close);
            }
            '+' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Add));
            }
            '-' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    expr.push(enums::Token::Op(enums::Operator::Sub));
                    builder = String::new();
                } else if builder.len() == 0 {
                    expr.push(enums::Token::Op(enums::Operator::Negate));
                }
            }
            '*' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Mul));
            }
            '/' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Div));
            }
            '^' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Pow));
            }
            ',' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    builder = String::new();
                }
                expr.push(enums::Token::Comma);
            }
            '%' => {
                if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
                    expr.push(x);
                    builder = String::new();
                }
                expr.push(enums::Token::Op(enums::Operator::Mod));
            }
            '=' => {
                if builder.len() > 0 {
                    variable = builder.clone();
                    builder = String::new();
                }
            }
            _ => {
                builder.push(c);
            }
        }
    }
    if let Some(x) = decide_what_to_push(&builder, numeric_regex, function_regex) {
        expr.push(x);
    }
    (variable, expr)
}

fn convert_to_postfix(input: &String,
                      variable: String,
                      expr: Expression)
                      -> (String, Result<Expression, String>) {
    let mut op_stack: Vec<enums::Token> = Vec::with_capacity(input.len());
    let mut out_queue: Vec<enums::Token> = Vec::with_capacity(input.len());
    for i in 0..expr.len() {
        let current_token = expr.get_token(i);
        match current_token {
            &enums::Token::Literal(ref x) => out_queue.push(enums::Token::Literal(x.clone())),
            &enums::Token::Func(ref x) => op_stack.push(enums::Token::Func(x.clone())),
            &enums::Token::Comma => {
                loop {
                    let stack_token = op_stack.pop();
                    if !stack_token.is_some() {
                        return (variable,
                                Err("Malformed Expression, comma but no Parenthesis".to_owned()));
                    }
                    let stack_token = stack_token.unwrap();
                    match stack_token {
                        enums::Token::Open => {
                            op_stack.push(stack_token);
                            break;
                        }
                        _ => out_queue.push(stack_token),
                    }
                }
            }
            &enums::Token::Op(ref o1) => {
                loop {
                    if op_stack.len() < 1 {
                        break;
                    }
                    let o2 = op_stack.pop().unwrap(); // top of stack, must exist based off of previous if
                    match o1 {
                        &enums::Operator::Negate => {
                            op_stack.push(o2);
                            break;
                        }
                        &enums::Operator::Pow => {
                            match o2 {
                                enums::Token::Op(enums::Operator::Negate) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                        &enums::Operator::Mul |
                        &enums::Operator::Div |
                        &enums::Operator::Mod => {
                            match o2 {
                                enums::Token::Op(enums::Operator::Negate) |
                                enums::Token::Op(enums::Operator::Pow) |
                                enums::Token::Op(enums::Operator::Mul) |
                                enums::Token::Op(enums::Operator::Div) |
                                enums::Token::Op(enums::Operator::Mod) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                        &enums::Operator::Add |
                        &enums::Operator::Sub => {
                            match o2 {
                                enums::Token::Op(enums::Operator::Negate) |
                                enums::Token::Op(enums::Operator::Pow) |
                                enums::Token::Op(enums::Operator::Mul) |
                                enums::Token::Op(enums::Operator::Div) |
                                enums::Token::Op(enums::Operator::Mod) |
                                enums::Token::Op(enums::Operator::Add) |
                                enums::Token::Op(enums::Operator::Sub) => out_queue.push(o2),
                                _ => {
                                    op_stack.push(o2);
                                    break;
                                }
                            }
                        }
                    }
                }
                op_stack.push(enums::Token::Op(o1.clone()));
            }
            &enums::Token::Open => op_stack.push(enums::Token::Open),
            &enums::Token::Close => {
                loop {
                    let stack_token = op_stack.pop();
                    if !stack_token.is_some() {
                        return (variable,
                                Err("Malformed Expression, found a ) without (".to_owned()));
                    }
                    let stack_token = stack_token.unwrap();
                    match stack_token {
                        enums::Token::Open => break,
                        _ => out_queue.push(stack_token),
                    }
                }
                if op_stack.len() > 0 {
                    let next_stack_token = op_stack.pop().unwrap(); // must exist based off of previous if
                    match next_stack_token {
                        enums::Token::Func(ref x) => out_queue.push(enums::Token::Func(x.clone())),
                        _ => op_stack.push(next_stack_token),
                    }
                }
            }
            &enums::Token::Var(ref x) => out_queue.push(enums::Token::Var(x.clone())),
            &enums::Token::Unknown(ref x) => {
                let mut message: String = "You either misspelled a function, or it is not yet \
                                           implemented. The unknown string was: ".to_owned();
                message.push_str(x);
                return (variable, Err(message));
            }
            _ => break,
        }
    }
    while op_stack.len() > 0 {
        out_queue.push(op_stack.pop().unwrap()); // The item must exist
    }
    (variable, Ok(Expression::new(out_queue)))
}

fn decide_what_to_push(builder: &String,
                       numeric_regex: &Regex,
                       function_regex: &Regex)
                       -> Option<enums::Token> {
    if builder.len() > 0 {
        let to_push;
        if numeric_regex.is_match(&builder[..]) {
            to_push = enums::Token::Literal(builder.clone());
        } else if function_regex.is_match(&builder[..]) {
            to_push = enums::map_string_to_func(&builder);
        } else {
            to_push = enums::Token::Var(builder.clone());
        }
        return Some(to_push);
    }
    None
}
