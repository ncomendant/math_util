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

impl Operation {
    pub fn evaluate(&self) -> RationalNumber {
        match self {
            Operation::Exponent(a, b) => a.evaluate().pow(&b.evaluate()),
            Operation::Division(a, b) => a.evaluate() / b.evaluate(),
            Operation::Multiplication(a, b) => a.evaluate() * b.evaluate(),
            Operation::Addition(a, b) => a.evaluate() + b.evaluate(),
            Operation::Subtraction(a, b) => a.evaluate() - b.evaluate(),
            Operation::None(n) => *n
        }
    }

    pub fn divide(a: RationalNumber, b: RationalNumber) -> Operation {
        Operation::Division(Box::new(a.into()), Box::new(b.into()))
    }

    pub fn multiply(a: RationalNumber, b: RationalNumber) -> Operation {
        Operation::Multiplication(Box::new(a.into()), Box::new(b.into()))
    }

    pub fn add(a: RationalNumber, b: RationalNumber) -> Operation {
        Operation::Addition(Box::new(a.into()), Box::new(b.into()))
    }

    pub fn subtract(a: RationalNumber, b: RationalNumber) -> Operation {
        Operation::Subtraction(Box::new(a.into()), Box::new(b.into()))
    }
}

impl From<RationalNumber> for Operation {
    fn from(n: RationalNumber) -> Self {
        Operation::None(n)
    }
}

impl From<u32> for Operation {
    fn from(n: u32) -> Self {
        Operation::None(n.into())
    }
}

impl From<i32> for Operation {
    fn from(n: i32) -> Self {
        Operation::None(n.into())
    }
}

impl From<f32> for Operation {
    fn from(f: f32) -> Self {
        Operation::None(f.into())
    }
}

impl From<u32> for Box<Operation> {
    fn from(n: u32) -> Self {
        Box::new(n.into())
    }
}

impl From<i32> for Box<Operation> {
    fn from(n: i32) -> Self {
        Box::new(n.into())
    }
}

impl From<f32> for Box<Operation> {
    fn from(n: f32) -> Self {
        Box::new(n.into())
    }
}

fn evaluate_str(expression: &str) -> Result<RationalNumber, LibError> {
    if let Some((sub_expr, i)) = find_next_group(expression) {
        let next_expr = format!("{}{}{}", &expression[0..i], evaluate_str(&sub_expr)?, &expression[i+sub_expr.len()..]);
        evaluate_str(&next_expr)
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

#[cfg(test)]
mod tests {
    use crate::Operation;

    #[test]
    fn evaluates_raw_operations() {
        let o = Operation::Addition(2.into(), 3.into());
        assert_eq!(o.evaluate().as_i32().unwrap(), 5);

        let o = Operation::Addition((-10).into(), 3.into());
        assert_eq!(o.evaluate().as_i32().unwrap(), -7);

        let o = Operation::Multiplication((-3).into(), Operation::Addition(4.into(), 3.into()).into());
        assert_eq!(o.evaluate().as_i32().unwrap(), -21);
    }
}