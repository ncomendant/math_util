use regex::Regex;

use crate::rational_number::RationalNumber;
use std::fmt;
use std::ops;
use crate::{Result, Error};

const EXPONENT_RE: &str = r"^(?:\s*)\^(?:\s*)$";
const DIVISION_RE: &str = r"^(?:\s*)(?:/|\-:)(?:\s*)$";
const MULTIPLICATION_RE: &str = r"^(?:\s*)\*(?:\s*)$";
const ADDITION_RE: &str = r"^(?:\s*)\+(?:\s*)$";
const SUBTRACTION_RE: &str = r"^(?:\s*)\-(?:\s*)$";

pub(crate) fn parse_first_expression_value(s: &str) -> Result<(usize, ExpressionValue)> {
    if let Some((i, n)) = parse_first_number(s) {
        return Ok((i, n.into()));
    } else if let Ok(expr) = parse_first_expression(s) {
        if let Some((i, expr)) = expr {
            return Ok((i, expr.into()));
        }
    }
    Err(Error::ParseExpression)
}

pub(crate) fn parse_first_operation(expression: &str) -> Option<(usize, ExpressionOperation)> {
    let exponent_re = Regex::new(EXPONENT_RE).expect("invalid regex");
    let division_re = Regex::new(DIVISION_RE).expect("invalid regex");
    let multiplication_re = Regex::new(MULTIPLICATION_RE).expect("invalid regex");
    let addition_re = Regex::new(ADDITION_RE).expect("invalid regex");
    let subtraction_re = Regex::new(SUBTRACTION_RE).expect("invalid regex");

    let mut n = None;

    for i in 0..=expression.len() {
        let s = &expression[0..i];
        if exponent_re.is_match(s) {
            n = Some((i, ExpressionOperation::Exponent));
        } else if division_re.is_match(s) {
            n = Some((i, ExpressionOperation::Division));
        } else if multiplication_re.is_match(s) {
            n = Some((i, ExpressionOperation::Multiplication));
        } else if addition_re.is_match(s) {
            n = Some((i, ExpressionOperation::Addition));
        } else if subtraction_re.is_match(s) {
            n = Some((i, ExpressionOperation::Subtraction));
        }
    }
    n
}

pub(crate) fn parse_first_number(expression: &str) -> Option<(usize, RationalNumber)> {
    let mut n = None;
    for i in 0..=expression.len() {
        if let Ok(num) = RationalNumber::parse(&expression[0..i]) {
            n = Some((i, num))
        }
    }
    n
}

pub fn parse_first_expression(expression: &str) -> Result<Option<(usize, Expression)>> {
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
                start = Some(i + 1); // do not including grouping symbol at start
                depth = 1;
            } else if c == '[' {
                is_bracket = Some(true);
                start = Some(i + 1); // do not including grouping symbol at start
                depth = 1;
            }
        }
    }

    if let Some(start) = start {
        let end = end.ok_or(Error::ParseExpression)?;
        if start < end {
            let sub_expr = crate::parse_expression(&expression[start..end])?;
            Ok(Some((end + 1, sub_expr)))
        } else {
            Err(Error::ParseExpression)
        }
    } else {
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionValue {
    Expression(Expression),
    Number(RationalNumber),
}

impl ExpressionValue {
    pub fn expression(&self) -> &Expression {
        match self {
            ExpressionValue::Expression(e) => e,
            _ => panic!("not a number"),
        }
    }

    pub fn number(&self) -> &RationalNumber {
        match self {
            ExpressionValue::Number(n) => n,
            _ => panic!("not a number"),
        }
    }
}

impl From<Expression> for ExpressionValue {
    fn from(e: Expression) -> Self {
        ExpressionValue::Expression(e)
    }
}

impl From<RationalNumber> for ExpressionValue {
    fn from(n: RationalNumber) -> Self {
        ExpressionValue::Number(n)
    }
}

impl From<u32> for ExpressionValue {
    fn from(n: u32) -> Self {
        ExpressionValue::Number(n.into())
    }
}

impl From<i32> for ExpressionValue {
    fn from(n: i32) -> Self {
        ExpressionValue::Number(n.into())
    }
}

impl From<f32> for ExpressionValue {
    fn from(n: f32) -> Self {
        ExpressionValue::Number(n.into())
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    values: Vec<ExpressionValue>,
    operations: Vec<ExpressionOperation>,
}

impl Expression {
    pub fn new<T: Into<ExpressionValue>>(n: T) -> Self {
        Expression {
            values: vec![n.into()],
            operations: Vec::new(),
        }
    }

    pub fn values(&self) -> &Vec<ExpressionValue> {
        &self.values
    }

    pub fn operations(&self) -> &Vec<ExpressionOperation> {
        &self.operations
    }

    pub fn pow<T: Into<ExpressionValue>>(&self, n: T) -> Self {
        let mut e = self.clone();
        e.values.push(n.into());
        e.operations.push(ExpressionOperation::Exponent);
        e
    }

    pub fn push<T: Into<ExpressionValue>>(&self, operation: ExpressionOperation, value: T) -> Self {
        let mut e = self.clone();
        e.operations.push(operation);
        e.values.push(value.into());
        e
    }

    pub fn evaluate(&self) -> RationalNumber {
        let mut expr = self.clone();
        loop {
            if let Some(val) = expr.evaluate_next() {
                match val {
                    ExpressionValue::Expression(e) => expr = e,
                    ExpressionValue::Number(n) => return n,
                }
            } else {
                // if expression is only a number
                return expr
                    .values
                    .get(0)
                    .expect("failed to get number")
                    .number()
                    .clone();
            }
        }
    }

    pub fn evaluate_next(&self) -> Option<ExpressionValue> {
        if let Some(e) = self.evaluate_next_expression() {
            Some(e.into())
        } else if let Some(e) = self.evaluate_next_operation() {
            Some(e)
        } else {
            None
        }
    }

    fn evaluate_next_expression(&self) -> Option<Expression> {
        let mut expr = None;
        for (i, val) in self.values.iter().enumerate() {
            match val {
                ExpressionValue::Expression(sub_expr) => {
                    if let Some(sub_expr) = sub_expr.evaluate_next() {
                        let mut e = self.clone();
                        e.values[i] = sub_expr;
                        expr = Some(e);
                        break;
                    }
                }
                _ => {}
            }
        }
        expr
    }

    fn evaluate_next_operation(&self) -> Option<ExpressionValue> {
        if self.values.len() == 1 {
            return Some(
                self.values
                    .get(0)
                    .expect("failed to get only value")
                    .clone(),
            );
        }

        let mut next_op: Option<(usize, OperationPriority)> = None;
        for (i, op) in self.operations.iter().enumerate() {
            if let Some((_next_i, next_priority)) = next_op {
                if op.priority() > next_priority {
                    next_op = Some((i, op.priority()));
                }
            } else {
                next_op = Some((i, op.priority()));
            }
        }

        if let Some((next_i, _next_priority)) = next_op {
            let mut e = self.clone();
            let op = e.operations.remove(next_i);
            let a = *e.values.remove(next_i).number();
            let b = *e.values.remove(next_i).number();
            let val = match op {
                ExpressionOperation::Exponent => a.pow(&b),
                ExpressionOperation::Division => a / b,
                ExpressionOperation::Multiplication => a * b,
                ExpressionOperation::Addition => a + b,
                ExpressionOperation::Subtraction => a - b,
            };
            if e.values.is_empty() {
                Some(val.into())
            } else {
                e.values.insert(next_i, val.into());
                Some(e.into())
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = "".to_string();
        for (i, val) in self.values.iter().enumerate() {
            let val = match val {
                ExpressionValue::Expression(e) => format!("({})", e),
                ExpressionValue::Number(n) => n.as_str(None),
            };

            let op = match self.operations.get(i) {
                Some(op) => match op {
                    ExpressionOperation::Exponent => Some("^"),
                    ExpressionOperation::Division => Some(" -: "),
                    ExpressionOperation::Multiplication => Some(" * "),
                    ExpressionOperation::Addition => Some(" + "),
                    ExpressionOperation::Subtraction => Some(" - "),
                },
                None => None,
            };

            s.push_str(&val);
            if let Some(op) = op {
                s.push_str(op);
            }
        }
        write!(f, "{}", s)
    }
}

impl<T: Into<ExpressionValue>> ops::Div<T> for Expression {
    type Output = Expression;

    fn div(self, rhs: T) -> Self::Output {
        let mut e = self.clone();
        e.values.push(rhs.into());
        e.operations.push(ExpressionOperation::Division);
        e
    }
}

impl<T: Into<ExpressionValue>> ops::Mul<T> for Expression {
    type Output = Expression;

    fn mul(self, rhs: T) -> Self::Output {
        let mut e = self.clone();
        e.values.push(rhs.into());
        e.operations.push(ExpressionOperation::Multiplication);
        e
    }
}

impl<T: Into<ExpressionValue>> ops::Add<T> for Expression {
    type Output = Expression;

    fn add(self, rhs: T) -> Self::Output {
        let mut e = self.clone();
        e.values.push(rhs.into());
        e.operations.push(ExpressionOperation::Addition);
        e
    }
}

impl<T: Into<ExpressionValue>> ops::Sub<T> for Expression {
    type Output = Expression;

    fn sub(self, rhs: T) -> Self::Output {
        let mut e = self.clone();
        e.values.push(rhs.into());
        e.operations.push(ExpressionOperation::Subtraction);
        e
    }
}

pub type OperationPriority = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionOperation {
    Exponent,
    Division,
    Multiplication,
    Addition,
    Subtraction,
}

impl fmt::Display for ExpressionOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionOperation::Exponent => write!(f, "Exponent"),
            ExpressionOperation::Division => write!(f, "Division"),
            ExpressionOperation::Multiplication => write!(f, "Multiplication"),
            ExpressionOperation::Addition => write!(f, "Addition"),
            ExpressionOperation::Subtraction => write!(f, "Subtraction"),
        }
    }
}

impl ExpressionOperation {
    fn priority(&self) -> OperationPriority {
        match self {
            ExpressionOperation::Exponent => 2,
            ExpressionOperation::Division => 1,
            ExpressionOperation::Multiplication => 1,
            ExpressionOperation::Addition => 0,
            ExpressionOperation::Subtraction => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::Expression;
    use crate::rational_number::NumberDisplayFormat;

    #[test]
    fn evaluates_raw_operations() {
        let e = Expression::new(7);
        assert_eq!(e.evaluate().as_i32().unwrap(), 7);

        let e = Expression::new(3) + 5;
        assert_eq!(e.evaluate().as_i32().unwrap(), 8);

        let e = Expression::new(3) + -5;
        assert_eq!(e.evaluate().as_i32().unwrap(), -2);

        let e = Expression::new(3) - -5;
        assert_eq!(e.evaluate().as_i32().unwrap(), 8);

        let e = Expression::new(3) / -5;
        assert_eq!(e.evaluate().as_f32(), -0.6);

        let e = Expression::new(1.3) + 0.2;
        assert_eq!(e.evaluate().as_f32(), 1.5);

        let e = crate::parse_expression("(2 + 3)/2 * 7^2").unwrap();
        assert_eq!(
            e.evaluate().as_str(Some(NumberDisplayFormat::Mixed)),
            "122 1/2"
        );
    }

    #[test]
    fn evaluates_with_subexpressions() {
        let sub_a = Expression::new(4) + 1;
        let sub_b = Expression::new(5) + 8;

        let e = sub_a - sub_b;
        assert_eq!(e.evaluate().as_i32().unwrap(), -8);
    }
}
