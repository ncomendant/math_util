use crate::rational_number::RationalNumber;
use crate::LibError;

#[derive(Debug, Clone)]
struct Expression {
    pub number: RationalNumber,
    pub parent: Option<Operation>,
}

impl Expression {
    pub fn new<T: Into<RationalNumber>>(n: T) -> Self {
        Expression {
            number: n.into(),
            parent: None,
        }
    }

    pub fn pow<T: Into<RationalNumber>>(&mut self, n: T) -> Self {
        Expression {
            parent: Some(Operation::Exponent(Box::new(self.clone()))),
            number: n.into(),
        }
    }

    pub fn divide<T: Into<RationalNumber>>(&mut self, n: T) -> Self {
        Expression {
            parent: Some(Operation::Division(Box::new(self.clone()))),
            number: n.into(),
        }
    }

    pub fn multiply<T: Into<RationalNumber>>(&mut self, n: T) -> Self {
        Expression {
            parent: Some(Operation::Multiplication(Box::new(self.clone()))),
            number: n.into(),
        }
    }

    pub fn add<T: Into<RationalNumber>>(&mut self, n: T) -> Self {
        Expression {
            parent: Some(Operation::Addition(Box::new(self.clone()))),
            number: n.into(),
        }
    }

    pub fn subtract<T: Into<RationalNumber>>(&mut self, n: T) -> Self {
        Expression {
            parent: Some(Operation::Subtraction(Box::new(self.clone()))),
            number: n.into(),
        }
    }

    pub fn evaluate(&self) -> RationalNumber {
        if let Some(p) = &self.parent {
            match p {
                Operation::Exponent(e) =>  e.evaluate().pow(&self.number),
                Operation::Division(e) => e.evaluate() / self.number,
                Operation::Multiplication(e) => e.evaluate() * self.number,
                Operation::Addition(e) => e.evaluate() + self.number,
                Operation::Subtraction(e) => e.evaluate() - self.number,
            }
        } else {
            self.number
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Exponent(Box<Expression>),
    Division(Box<Expression>),
    Multiplication(Box<Expression>),
    Addition(Box<Expression>),
    Subtraction(Box<Expression>),
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

#[cfg(test)]
mod tests {
    use crate::expression::Expression;

    #[test]
    fn evaluates_raw_operations() {
        let e = Expression::new(3).add(5);
        assert_eq!(e.evaluate().as_i32().unwrap(), 8);

        let e = Expression::new(3).add(-5);
        assert_eq!(e.evaluate().as_i32().unwrap(), -2);

        let e = Expression::new(3).subtract(-5);
        assert_eq!(e.evaluate().as_i32().unwrap(), 8);

        let e = Expression::new(3).divide(-5);
        assert_eq!(e.evaluate().as_f32(), -0.6);

        let e = Expression::new(1.3).add(0.2);
        assert_eq!(e.evaluate().as_f32(), 1.5);

        let e = Expression::new(2)
            .add(3)
            .multiply(7);
        assert_eq!(e.evaluate().as_i32().unwrap(), 35);
    }
}