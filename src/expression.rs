use crate::rational_number::RationalNumber;
use std::fmt;
use std::ops;

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
    operations: Vec<Operation>,
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

    pub fn operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    pub fn pow<T: Into<ExpressionValue>>(&self, n: T) -> Self {
        let mut e = self.clone();
        e.values.push(n.into());
        e.operations.push(Operation::Exponent);
        e
    }

    pub fn push<T: Into<ExpressionValue>>(&self, operation: Operation, value: T) -> Self {
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
                Operation::Exponent => a.pow(&b),
                Operation::Division => a / b,
                Operation::Multiplication => a * b,
                Operation::Addition => a + b,
                Operation::Subtraction => a - b,
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
                    Operation::Exponent => Some("^"),
                    Operation::Division => Some(" -: "),
                    Operation::Multiplication => Some(" * "),
                    Operation::Addition => Some(" + "),
                    Operation::Subtraction => Some(" - "),
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
        e.operations.push(Operation::Division);
        e
    }
}

impl<T: Into<ExpressionValue>> ops::Mul<T> for Expression {
    type Output = Expression;

    fn mul(self, rhs: T) -> Self::Output {
        let mut e = self.clone();
        e.values.push(rhs.into());
        e.operations.push(Operation::Multiplication);
        e
    }
}

impl<T: Into<ExpressionValue>> ops::Add<T> for Expression {
    type Output = Expression;

    fn add(self, rhs: T) -> Self::Output {
        let mut e = self.clone();
        e.values.push(rhs.into());
        e.operations.push(Operation::Addition);
        e
    }
}

impl<T: Into<ExpressionValue>> ops::Sub<T> for Expression {
    type Output = Expression;

    fn sub(self, rhs: T) -> Self::Output {
        let mut e = self.clone();
        e.values.push(rhs.into());
        e.operations.push(Operation::Subtraction);
        e
    }
}

pub type OperationPriority = u8;

#[derive(Debug, Clone)]
pub enum Operation {
    Exponent,
    Division,
    Multiplication,
    Addition,
    Subtraction,
}

impl Operation {
    fn priority(&self) -> OperationPriority {
        match self {
            Operation::Exponent => 2,
            Operation::Division => 1,
            Operation::Multiplication => 1,
            Operation::Addition => 0,
            Operation::Subtraction => 0,
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
