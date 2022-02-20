use crate::expression::{Expression, Operation};

pub mod expression;
pub mod math;
pub mod rational_number;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    ParseError,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse_expression(s: &str) -> Result<Expression> {
    let mut index = 0;

    let (i, val) = expression::parse_first_expression_value(s)?;

    index += i;

    let mut expr = Expression::new(val);
    while index < s.len() {
        if let Some((i, op)) = expression::parse_first_operation(&s[index..]) {
            index += i;
            let (i, val) = expression::parse_first_expression_value(&s[index..])?;
            index += i;
            expr = expr.push(op, val);
        } else if let Some((i, val)) = expression::parse_first_expression(&s[index..])? {
            // implied multiplication
            index += i;
            expr = expr.push(Operation::Multiplication, val);
        } else {
            return Err(Error::ParseError);
        }
    }

    Ok(expr)
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
