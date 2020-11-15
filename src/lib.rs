#![allow(dead_code)]

use crate::rational_number::RationalNumber;

mod rational_number;
mod math;

#[derive(Debug, Copy, Clone)]
pub enum LibError {
    ParseError,
}

enum Operation {
    Exponent(Box<Operation>, Box<Operation>),
    Division(Box<Operation>, Box<Operation>),
    Multiplication(Box<Operation>, Box<Operation>),
    Addition(Box<Operation>, Box<Operation>),
    Subtraction(Box<Operation>, Box<Operation>),
    None(RationalNumber),
}

fn evaluate(expression: &str) -> Result<RationalNumber, LibError> {
    if let Some((sub_expr, i)) = find_next_group(expression) {
        let next_expr = format!("{}{}{}", &expression[0..i], evaluate(&sub_expr)?, &expression[i+sub_expr.len()..]);
        evaluate(&next_expr)
    } else {
        println!("time for: {}", expression);
        unimplemented!()
    }
}

fn find_next_group(expression: &str) -> Option<(&str, usize)> {
    let mut start = 0;
    let mut levels_deep = 0;
    for (i, c) in expression.chars().into_iter().enumerate() {
        if c == '(' {
            if start == 0 {
                start = i+1;
            }
            levels_deep += 1;
        } else if c == ')' {
            levels_deep -= 1;
            if levels_deep == 0 {
                return Some((&expression[start..i], start));
            }
        }
    }
    None
}

// fn parse_next_operation(expression: &str) -> Option<Operation> {
//     if let Some(i) = expression.chars().position("^") {
//
//     }
// }

// fn parse_number_backwards(expression: &str)  -> RationalNumber {
//
// }
//
// fn parse_number_forwards(expression: &str)  -> RationalNumber {
//
// }