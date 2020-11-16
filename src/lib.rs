#![allow(dead_code)]

use crate::rational_number::RationalNumber;

mod rational_number;
mod math;
mod expression;

#[derive(Debug, Copy, Clone)]
pub enum LibError {
    ParseError,
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