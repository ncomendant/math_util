#![allow(dead_code)]

use crate::rational_number::{RationalNumber, NumberPrintFormat};
use regex::Regex;
use crate::expression::{Expression, Operation};

mod rational_number;
mod math;
mod expression;

const EXPONENT_RE: &str = r"^(?:\s*)\^(?:\s*)$";
const DIVISION_RE: &str = r"^(?:\s*)(?:/|\-:)(?:\s*)$";
const MULTIPLICATION_RE: &str = r"^(?:\s*)\*(?:\s*)$";
const ADDITION_RE: &str = r"^(?:\s*)\+(?:\s*)$";
const SUBTRACTION_RE: &str = r"^(?:\s*)\-(?:\s*)$";

#[derive(Debug, Copy, Clone)]
pub enum LibError {
    ParseError,
}

fn parse_expression(expression_str: &str) -> Result<Expression, LibError> {
    let mut index = 0;

    if let Some((i, n)) = parse_number_forwards(expression_str) {
        index += i;
        let mut expr = Expression::new(n);
        while index < expression_str.len() {
            if let Some((i, op)) = parse_operation_forwards(&expression_str[index..]) {
                index += i;
                if let Some((i, n)) = parse_number_forwards(&expression_str[index..]) {
                    index += i;
                    expr = expr.push(op, n);
                }
            } else {
                return Err(LibError::ParseError);
            }
        }
        Ok(expr)
    } else {
        Err(LibError::ParseError)
    }
}

fn parse_operation_forwards(expression: &str) -> Option<(usize, Operation)> {
    let exponent_re = Regex::new(EXPONENT_RE).expect("invalid regex");
    let division_re = Regex::new(DIVISION_RE).expect("invalid regex");
    let multiplication_re = Regex::new(MULTIPLICATION_RE).expect("invalid regex");
    let addition_re = Regex::new(ADDITION_RE).expect("invalid regex");
    let subtraction_re = Regex::new(SUBTRACTION_RE).expect("invalid regex");

    let mut n = None;

    for i in 0..=expression.len() {
        let s = &expression[0..i];
        if exponent_re.is_match(s) {
            n = Some((i, Operation::Exponent));
        } else if division_re.is_match(s) {
            n = Some((i, Operation::Division));
        } else if multiplication_re.is_match(s) {
            n = Some((i, Operation::Multiplication));
        } else if addition_re.is_match(s) {
            n = Some((i, Operation::Addition));
        } else if subtraction_re.is_match(s) {
            n = Some((i, Operation::Subtraction));
        }
    }
    n
}

fn parse_number_forwards(expression: &str) -> Option<(usize, RationalNumber)> {
    let mut n = None;
    for i in 0..=expression.len() {
        if let Ok(num) = RationalNumber::parse(&expression[0..i]) {
            n = Some((i, num))
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use crate::parse_expression;
    use crate::rational_number::NumberPrintFormat;

    #[test]
    fn parses_strings() {
        let e = parse_expression("3.1+1/2 * -5").unwrap();
        assert_eq!(e.evaluate().simplify().as_str(NumberPrintFormat::Decimal), "0.6");

        let e = parse_expression("8/4 * 2 + -3 - 12").unwrap();
        assert_eq!(e.evaluate().simplify().as_str(NumberPrintFormat::Decimal), "-11");
    }
}