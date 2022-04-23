use crate::expression::{Expression};
use expression::ExpressionOperation;
use rational_number::RationalNumber;
use serde::{Serialize, Deserialize};
use std::{fmt, num::ParseIntError};

pub mod expression;
pub mod rational_number;

#[derive(Debug, Clone)]
pub enum Error {
    ParseRationalExpression,
    ParseExpression,
    ParseInt(ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseExpression => write!(f, "ParseExpressionError"),
            Error::ParseInt(e) => write!(f, "{}", e),
            Error::ParseRationalExpression =>  write!(f, "ParseRationalExpressionError"),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub const VARIABLES: &str = "abcdefghijkmnpqrstuvwxyz";

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PlaceValue {
    Millions = 6,
    HundredThousands = 5,
    TenThousands = 4,
    Thousands = 3,
    Hundreds = 2,
    Tens = 1,
    Ones = 0,
    Tenths = -1,
    Hundredths = -2,
    Thousandths = -3,
    TenThousandths = -4,
    HundredThousandths = -5,
    Millionths = -6,
}

impl PlaceValue {
    pub fn as_str(&self) -> String {
        let s = match self {
            PlaceValue::Millions => "millions",
            PlaceValue::HundredThousands => "hundred thousands",
            PlaceValue::TenThousands => "ten thousands",
            PlaceValue::Thousands => "thousands",
            PlaceValue::Hundreds => "hundreds",
            PlaceValue::Tens => "tens",
            PlaceValue::Ones => "ones",
            PlaceValue::Tenths => "tenths",
            PlaceValue::Hundredths => "hundredths",
            PlaceValue::Thousandths => "thousandths",
            PlaceValue::TenThousandths => "ten thousandths",
            PlaceValue::HundredThousandths => "hundred thousandths",
            PlaceValue::Millionths => "millionths",
        };
        s.to_string()
    }
}


impl From<PlaceValue> for i32 {
    fn from(p: PlaceValue) -> Self {
        match p {
            PlaceValue::Millions => 6,
            PlaceValue::HundredThousands => 5,
            PlaceValue::TenThousands => 4,
            PlaceValue::Thousands => 3,
            PlaceValue::Hundreds => 2,
            PlaceValue::Tens => 1,
            PlaceValue::Ones => 0,
            PlaceValue::Tenths => -1,
            PlaceValue::Hundredths => -2,
            PlaceValue::Thousandths => -3,
            PlaceValue::TenThousandths => -4,
            PlaceValue::HundredThousandths => -5,
            PlaceValue::Millionths => -6,
        }
    }
}

impl From<i32> for PlaceValue {
    fn from(n: i32) -> Self {
        if n == 6 {
            PlaceValue::Millions
        } else if n == 5 {
            PlaceValue::HundredThousands
        } else if n == 4 {
            PlaceValue::TenThousands
        } else if n == 3 {
            PlaceValue::Thousands
        } else if n == 2 {
            PlaceValue::Hundreds
        } else if n == 1 {
            PlaceValue::Tens
        } else if n == 0 {
            PlaceValue::Ones
        } else if n == -1 {
            PlaceValue::Tenths
        } else if n == -2 {
            PlaceValue::Hundredths
        } else if n == -3 {
            PlaceValue::Thousandths
        } else if n == -4 {
            PlaceValue::TenThousandths
        } else if n == -5 {
            PlaceValue::HundredThousandths
        } else if n == -6 {
            PlaceValue::Millionths
        } else {
            panic!("cannot convert i32 to PlaceValue")
        }
    }
}

pub fn gcf(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcf(b, a % b)
    }
}

pub fn lcm(a: u32, b: u32) -> u32 {
    (a / gcf(a, b)) * b
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Operation {
    Multiplication,
    Division,
    Addition,
    Subtraction,
}

impl Operation {
    pub fn as_symbol(&self) -> char {
        match self {
            Operation::Multiplication => '*',
            Operation::Division => '/',
            Operation::Addition => '+',
            Operation::Subtraction => '-',
        }
    }
}

pub fn round_i32(num: i32, place_value: PlaceValue) -> i32 {
    let factor = 10f32.powi(place_value.into());
    let result = (num as f32 / factor).round() * factor;
    result as i32
}

pub fn round_f32(num: f32, place_value: PlaceValue) -> String {
    match place_value {
        PlaceValue::Tenths => format!("{:.1}", num),
        PlaceValue::Hundredths => format!("{:.2}", num),
        PlaceValue::Thousandths => format!("{:.3}", num),
        PlaceValue::TenThousandths => format!("{:.4}", num),
        PlaceValue::HundredThousandths => format!("{:.5}", num),
        PlaceValue::Millionths => format!("{:.6}", num),
        _ => format!("{}", round_i32(num as i32, place_value)),
    }
}

pub fn round_i64(num: i64, place_value: PlaceValue) -> i64 {
    let factor = 10f64.powi(place_value.into());
    let result = (num as f64 / factor).round() * factor;
    result as i64
}

pub fn round_f64(num: f64, place_value: PlaceValue) -> String {
    match place_value {
        PlaceValue::Tenths => format!("{:.1}", num),
        PlaceValue::Hundredths => format!("{:.2}", num),
        PlaceValue::Thousandths => format!("{:.3}", num),
        PlaceValue::TenThousandths => format!("{:.4}", num),
        PlaceValue::HundredThousandths => format!("{:.5}", num),
        PlaceValue::Millionths => format!("{:.6}", num),
        _ => format!("{}", round_i64(num as i64, place_value)),
    }
}

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
            expr = expr.push(ExpressionOperation::Multiplication, val);
        } else {
            return Err(Error::ParseExpression);
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

pub trait WrapNumber {
    fn wrap_if_neg(self) -> String;
}

impl WrapNumber for RationalNumber {
    fn wrap_if_neg(self) -> String {
        if self.negative {
            format!("({})", self)
        } else {
            self.to_string()
        }
    }
}