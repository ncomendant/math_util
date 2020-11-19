use crate::rational_number::{RationalNumber};
use regex::Regex;
use crate::expression::{Expression, Operation, ExpressionValue};

pub mod rational_number;
pub mod expression;
mod math;


const EXPONENT_RE: &str = r"^(?:\s*)\^(?:\s*)$";
const DIVISION_RE: &str = r"^(?:\s*)(?:/|\-:)(?:\s*)$";
const MULTIPLICATION_RE: &str = r"^(?:\s*)\*(?:\s*)$";
const ADDITION_RE: &str = r"^(?:\s*)\+(?:\s*)$";
const SUBTRACTION_RE: &str = r"^(?:\s*)\-(?:\s*)$";

#[derive(Debug, Copy, Clone)]
pub enum OooParserError {
    ParseError,
}

pub fn parse_expression(s: &str) -> Result<Expression, OooParserError> {
    let mut index = 0;

    let (i, val) = parse_first_expression_value(s)?;

    index += i;

    let mut expr = Expression::new(val);
    while index < s.len() {
        if let Some((i, op)) = parse_first_operation(&s[index..]) {
            index += i;
            let (i, val) = parse_first_expression_value(&s[index..])?;
            index += i;
            expr = expr.push(op, val);
        } else if let Some((i, val)) = parse_first_expression(&s[index..])? { // implied multiplication
            index += i;
            expr = expr.push(Operation::Multiplication, val);
        } else {
            return Err(OooParserError::ParseError);
        }
    }

    Ok(expr)
}

fn parse_first_expression_value(s: &str) -> Result<(usize, ExpressionValue), OooParserError> {
    if let Some((i, n)) = parse_first_number(s) {
        return Ok((i, n.into()));
    } else if let Ok(expr) = parse_first_expression(s) {
        if let Some((i, expr)) = expr {
            return Ok((i, expr.into()));
        }
    }
    Err(OooParserError::ParseError)
}

fn parse_first_operation(expression: &str) -> Option<(usize, Operation)> {
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

fn parse_first_number(expression: &str) -> Option<(usize, RationalNumber)> {
    let mut n = None;
    for i in 0..=expression.len() {
        if let Ok(num) = RationalNumber::parse(&expression[0..i]) {
            n = Some((i, num))
        }
    }
    n
}

fn parse_first_expression(expression: &str) -> Result<Option<(usize, Expression)>, OooParserError> {
    let mut is_bracket = None;
    let mut start = None;
    let mut end = None;
    let mut depth = 0;
    for (i, c) in expression.chars().enumerate() {
        if let Some(is_bracket) = is_bracket {
            if is_bracket {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(i); // do not including grouping symbol at end
                        break;
                    }
                }
            } else {
                if c == '(' {
                    depth += 1;
                } else if c == ')' {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(i); // do not including grouping symbol at end
                        break;
                    }
                }
            }
        } else {
            if c == '(' {
                is_bracket = Some(false);
                start = Some(i+1); // do not including grouping symbol at start
                depth = 1;
            } else if c == '[' {
                is_bracket = Some(true);
                start = Some(i+1); // do not including grouping symbol at start
                depth = 1;
            }
        }
    }

    if let Some(start) = start {
        let end = end.ok_or(OooParserError::ParseError)?;
        if start < end {
            let sub_expr = parse_expression(&expression[start..end])?;
            Ok(Some((end + 1, sub_expr)))
        } else {
            Err(OooParserError::ParseError)
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_expression;

    #[test]
    fn parses_strings() {
        let e = parse_expression("3.1+1-:2 * -5").unwrap();
        assert_eq!(e.evaluate().simplify().as_str(None), "0.6");

        let e = parse_expression("8/4 * 2 + -3 - 12").unwrap();
        assert_eq!(e.evaluate().simplify().as_str(None), "-11");

        let e = parse_expression("(3 + 1) * (4 - 7)").unwrap();
        assert_eq!(e.evaluate().simplify().as_str(None), "-12");

        let e = parse_expression("(2)").unwrap();
        assert_eq!(e.evaluate().simplify().as_str(None), "2");

        let e = parse_expression("(2)^2(-3)(-4)").unwrap();
        assert_eq!(e.evaluate().simplify().as_str(None), "48");

        let e = parse_expression("3(3 + 1) - (2 + 1)^3").unwrap();
        assert_eq!(e.evaluate().simplify().as_str(None), "-15");
    }
}