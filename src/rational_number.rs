use crate::math::lcm;
use crate::{math, OooParserError};
use regex::Regex;
use std::ops::{Add, Neg};
use std::str::FromStr;
use std::{fmt, ops};

const MIXED_NUMER_RE: &str = r"^(?:\s*)([+-]?)(?:\s*)(\d+)(?:\s+)(\d+)(?:\s*)/(?:\s*)(\d+)(?:\s*)$";
const FRACTION_RE: &str = r"^(?:\s*)([+-]?)(?:\s*)(\d+)(?:\s*)/(?:\s*)(\d+)(?:\s*)$";
const DECIMAL_RE: &str = r"^(?:\s*)([+-]?)(?:\s*)(\d+)\.?(\d*)(?:\s*)$";
const DECIMAL_RE_2: &str = r"^(?:\s*)([+-]?)(?:\s*)\.(\d+)(?:\s*)$";

#[derive(Debug, Copy, Clone)]
pub enum NumberDisplayFormat {
    Decimal,
    Fraction,
    Mixed,
}

#[derive(Debug, Copy, Clone)]
pub struct RationalNumber {
    pub numerator: u32,
    pub denominator: u32,
    pub negative: bool,
    pub format: NumberDisplayFormat,
}

impl RationalNumber {
    pub fn as_f32(&self) -> f32 {
        let mut f = (self.numerator as f32) / (self.denominator as f32);
        if self.negative {
            f = f.neg();
        }
        f
    }

    pub fn as_i32(&self) -> Result<i32, OooParserError> {
        if self.numerator % self.denominator != 0 {
            Err(OooParserError::ParseError)
        } else {
            let mut n = (self.numerator / self.denominator) as i32;
            if self.negative {
                n = n.neg();
            }
            Ok(n)
        }
    }

    pub fn parse(s: &str) -> Result<Self, OooParserError> {
        if let Some(captures) = Regex::new(MIXED_NUMER_RE).unwrap().captures(s) {
            let negative_str = captures.get(1).unwrap().as_str();
            let whole_str = captures.get(2).unwrap().as_str();
            let numerator_str = captures.get(3).unwrap().as_str();
            let denominator_str = captures.get(4).unwrap().as_str();

            let negative = negative_str == "-";
            let whole = u32::from_str(whole_str).unwrap();
            let numerator = u32::from_str(&numerator_str).unwrap();
            let denominator = u32::from_str(&denominator_str).unwrap();

            let numerator = denominator * whole + numerator;

            Ok(RationalNumber {
                negative,
                numerator,
                denominator,
                format: NumberDisplayFormat::Mixed,
            })
        } else if let Some(captures) = Regex::new(FRACTION_RE).unwrap().captures(s) {
            let negative_str = captures.get(1).unwrap().as_str();
            let numerator_str = captures.get(2).unwrap().as_str();
            let denominator_str = captures.get(3).unwrap().as_str();

            let negative = negative_str == "-";
            let numerator = u32::from_str(&numerator_str).unwrap();
            let denominator = u32::from_str(&denominator_str).unwrap();

            let format = if numerator >= denominator {
                NumberDisplayFormat::Fraction
            } else {
                NumberDisplayFormat::Mixed
            };

            Ok(RationalNumber {
                negative,
                numerator,
                denominator,
                format,
            })
        } else if let Some(captures) = Regex::new(DECIMAL_RE).unwrap().captures(s) {
            let negative_str = captures.get(1).unwrap().as_str();
            let whole_str = captures.get(2).unwrap().as_str();
            let remainder_str = captures.get(3).unwrap().as_str();
            RationalNumber::parse_decimal(negative_str, whole_str, remainder_str)
        } else if let Some(captures) = Regex::new(DECIMAL_RE_2).unwrap().captures(s) {
            let negative_str = captures.get(1).unwrap().as_str();
            let whole_str = "0";
            let remainder_str = captures.get(2).unwrap().as_str();
            RationalNumber::parse_decimal(negative_str, whole_str, remainder_str)
        } else {
            Err(OooParserError::ParseError)
        }
    }

    fn parse_decimal(negative_str: &str, whole_str: &str, remainder_str: &str) -> Result<Self, OooParserError> {
        let negative = negative_str == "-";
        let whole = u32::from_str(whole_str).unwrap();
        let remainder = if let Ok(r) = u32::from_str(&remainder_str) {
            r
        } else {
            0
        };
        let denominator = 10u32.pow(remainder_str.len() as u32);

        let f = math::gcf(remainder, denominator);

        let remainder = remainder / f;
        let denominator = denominator / f;

        let numerator = denominator * whole + remainder;

        Ok(RationalNumber {
            numerator,
            denominator,
            negative,
            format: NumberDisplayFormat::Decimal,
        })
    }

    pub fn pow(&self, exp: &RationalNumber) -> RationalNumber {
        self.as_f32().powf(exp.as_f32()).into()
    }

    pub fn simplify(&self) -> RationalNumber {
        let gcf = math::gcf(self.numerator, self.denominator);
        RationalNumber {
            numerator: self.numerator / gcf,
            denominator: self.denominator / gcf,
            negative: self.negative,
            format: self.format,
        }
    }

    pub fn neg(&self) -> RationalNumber {
        RationalNumber {
            negative: !self.negative,
            numerator: self.numerator,
            denominator: self.denominator,
            format: self.format,
        }
    }

    pub fn reciprocal(&self) -> RationalNumber {
        RationalNumber {
            negative: self.negative,
            numerator: self.denominator,
            denominator: self.numerator,
            format: self.format,
        }
    }

    pub fn repeating(&self) -> bool {
        let (_, repeating_digit_count) = self.as_decimal_str();
        repeating_digit_count.is_some()
    }

    pub fn as_decimal_str(&self) -> (String, Option<usize>) { // decimal, repeating_digit_count
        let mut s = (self.numerator / self.denominator).to_string();
        let mut remainder = self.numerator % self.denominator;
        if remainder != 0 {
            s.push_str(".");
            let mut remainders = Vec::new();
            remainders.push(remainder);
            while remainder != 0 {
                remainder *= 10;
                println!("{}", remainder);
                let digit_str = (remainder / self.denominator).to_string();
                println!("digit: {}", digit_str);
                remainder %= self.denominator;
                println!("then {}", remainder);
                s.push_str(&digit_str);
                if let Some(index) = remainders.iter().position(|x| *x == remainder) {
                    // let decimal_index = s.chars().position(|c| c == '.').expect("could not find decimal");
                    return (s, Some(remainders.len() - index))
                } else {
                    remainders.push(remainder);
                }
            }
        }
        (s, None)
    }

    pub fn as_str(&self, format: Option<NumberDisplayFormat>) -> String {
        let format = match format {
            Some(f) => f,
            None => self.format,
        };

        match format {
            NumberDisplayFormat::Decimal => {
                let remainder = self.numerator % self.denominator;
                if remainder == 0 {
                    if self.negative {
                        format!("-{}", (self.numerator / self.denominator))
                    } else {
                        (self.numerator / self.denominator).to_string()
                    }
                } else {
                    if self.negative {
                        format!("-{}", self.as_f32())
                    } else {
                        self.as_f32().to_string()
                    }
                }
            }
            NumberDisplayFormat::Fraction => {
                if self.negative {
                    format!("-{}/{}", self.numerator, self.denominator)
                } else {
                    format!("{}/{}", self.numerator, self.denominator)
                }
            }
            NumberDisplayFormat::Mixed => {
                let whole = self.numerator / self.denominator;
                let remainder = self.numerator % self.denominator;
                if self.numerator < self.denominator {
                    self.as_str(Some(NumberDisplayFormat::Fraction))
                } else if remainder == 0 {
                    if self.negative {
                        format!("-{}", whole)
                    } else {
                        format!("{}", whole)
                    }
                } else {
                    if self.negative {
                        format!("-{} {}/{}", whole, remainder, self.denominator)
                    } else {
                        format!("{} {}/{}", whole, remainder, self.denominator)
                    }
                }
            }
        }
    }
}

impl From<f32> for RationalNumber {
    fn from(n: f32) -> Self {
        RationalNumber::parse(&n.to_string()).expect("failed to parse f32")
    }
}

impl From<u32> for RationalNumber {
    fn from(n: u32) -> Self {
        RationalNumber {
            numerator: n,
            denominator: 1,
            negative: false,
            format: NumberDisplayFormat::Decimal,
        }
    }
}

impl From<i32> for RationalNumber {
    fn from(n: i32) -> Self {
        RationalNumber {
            numerator: n.abs() as u32,
            denominator: 1,
            negative: n < 0,
            format: NumberDisplayFormat::Decimal,
        }
    }
}

impl fmt::Display for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            write!(f, "-{}/{}", self.numerator, self.denominator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl ops::Add<RationalNumber> for RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: RationalNumber) -> Self::Output {
        let denominator = lcm(self.denominator, rhs.denominator);
        let self_factor = denominator / self.denominator;
        let rhs_factor = denominator / rhs.denominator;
        let self_numerator = self.numerator * self_factor;
        let rhs_numerator = rhs.numerator * rhs_factor;
        let numerator;
        let negative;

        if self.negative == rhs.negative {
            numerator = self_numerator + rhs_numerator;
            negative = self.negative;
        } else {
            if self_numerator > rhs_numerator {
                numerator = self_numerator - rhs_numerator;
                negative = self.negative;
            } else {
                numerator = rhs_numerator - self_numerator;
                negative = rhs.negative;
            }
        }

        RationalNumber {
            numerator,
            denominator,
            negative,
            format: evaluated_format(&self, &rhs),
        }
    }
}

impl ops::Sub<RationalNumber> for RationalNumber {
    type Output = RationalNumber;

    fn sub(self, rhs: RationalNumber) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl ops::Mul<RationalNumber> for RationalNumber {
    type Output = RationalNumber;

    fn mul(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
            negative: self.negative != rhs.negative,
            format: evaluated_format(&self, &rhs),
        }
    }
}

impl ops::Div<RationalNumber> for RationalNumber {
    type Output = RationalNumber;

    fn div(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
            negative: self.negative != rhs.negative,
            format: evaluated_format(&self, &rhs),
        }
    }
}

fn evaluated_format(a: &RationalNumber, b: &RationalNumber) -> NumberDisplayFormat {
    if a.simplify().denominator == 1 {
        b.format
    } else {
        a.format
    }
}

#[cfg(test)]
mod tests {
    use crate::rational_number::{NumberDisplayFormat, RationalNumber};
    use rand::Rng;
    use std::ops::Neg;

    #[test]
    fn adds() {
        let a = RationalNumber::parse("2.5").unwrap();
        let b = RationalNumber::parse("1.5").unwrap();
        assert_eq!((a + b).as_f32(), 4.0);

        let a = RationalNumber::parse("-2.5").unwrap();
        let b = RationalNumber::parse("1.5").unwrap();
        assert_eq!((a + b).as_f32(), -1.0);

        let a = RationalNumber::parse("7").unwrap();
        let b = RationalNumber::parse("2").unwrap();
        assert_eq!((a + b).as_f32(), 9.0);

        let a = RationalNumber::parse("10").unwrap();
        let b = RationalNumber::parse("3").unwrap();
        assert_eq!((a + b).as_f32(), 13.0);
    }

    #[test]
    fn subtracts() {
        let a = RationalNumber::parse("2.5").unwrap();
        let b = RationalNumber::parse("1.5").unwrap();
        assert_eq!((a - b).as_f32(), 1.0);

        let a = RationalNumber::parse("-2.5").unwrap();
        let b = RationalNumber::parse("1.5").unwrap();
        assert_eq!((a - b).as_f32(), -4.0);

        let a = RationalNumber::parse("7").unwrap();
        let b = RationalNumber::parse("2").unwrap();
        assert_eq!((a - b).as_f32(), 5.0);

        let a = RationalNumber::parse("10").unwrap();
        let b = RationalNumber::parse("3").unwrap();
        assert_eq!((a - b).as_f32(), 7.0);
    }

    #[test]
    fn parses_specific_decimals() {
        assert_eq!(RationalNumber::parse("2.15").unwrap().as_f32(), 2.15);
        assert_eq!(RationalNumber::parse("0.15").unwrap().as_f32(), 0.15);
        assert_eq!(RationalNumber::parse(".15").unwrap().as_f32(), 0.15);
    }

    #[test]
    fn parses_random_decimals() {
        let mut rng = rand::thread_rng();
        for _i in 0..100 {
            let mut f = rng.gen::<f32>();
            if rng.gen_bool(0.5) {
                f = f.neg();
            }
            let f = (f * 10_000_000 as f32).round() / 10_000_000 as f32;
            assert_eq!(RationalNumber::parse(&f.to_string()).unwrap().as_f32(), f);
        }
    }

    #[test]
    fn parses_specific_fractions() {
        assert_eq!(RationalNumber::parse("1/3").unwrap().as_str(None), "1/3");
        assert_eq!(RationalNumber::parse("7/5").unwrap().as_str(None), "7/5");
        assert_eq!(
            RationalNumber::parse("10/20").unwrap().as_str(None),
            "10/20"
        );
    }

    #[test]
    fn parses_specific_mixed_numbers() {
        assert_eq!(
            RationalNumber::parse("2 1/5").unwrap().as_str(None),
            "2 1/5"
        );
        assert_eq!(
            RationalNumber::parse("-2 1/5").unwrap().as_str(None),
            "-2 1/5"
        );
        assert_eq!(
            RationalNumber::parse("5 4/10")
                .unwrap()
                .as_str(Some(NumberDisplayFormat::Decimal)),
            "5.4"
        );
        assert_eq!(
            RationalNumber::parse("8939 5776/9593").unwrap().as_str(None),
            "8939 5776/9593"
        );
    }

    #[test]
    fn parses_random_mixed_numbers() {
        let mut rng = rand::thread_rng();
        for _i in 0..1_000 {
            let whole = rng.gen_range(1u32, 10_000u32);
            let denominator = rng.gen_range(2u32, 10_000u32);
            let numerator = rng.gen_range(1, denominator);

            let neg_str = if rng.gen_bool(0.5) { "-" } else { "" };

            let mixed_number_str = format!("{}{} {}/{}", neg_str, whole, numerator, denominator);
            assert_eq!(
                RationalNumber::parse(&mixed_number_str)
                    .unwrap()
                    .as_str(Some(NumberDisplayFormat::Mixed)),
                mixed_number_str
            );
        }
    }

    #[test]
    fn checks_repeating() {
        assert_eq!(RationalNumber::parse("1/2").unwrap().repeating(), false);
        assert_eq!(RationalNumber::parse("1/8").unwrap().repeating(), false);
        assert_eq!(RationalNumber::parse("0").unwrap().repeating(), false);
        assert_eq!(RationalNumber::parse("4/2").unwrap().repeating(), false);
        assert_eq!(RationalNumber::parse("4/3").unwrap().repeating(), true);
        assert_eq!(RationalNumber::parse("1/7").unwrap().repeating(), true);
        assert_eq!(RationalNumber::parse("14/7").unwrap().repeating(), false);
        assert_eq!(RationalNumber::parse("1/11").unwrap().repeating(), true);
    }

    #[test]
    fn prints_decimals() {
        assert_eq!(RationalNumber::parse("1/100").unwrap().as_decimal_str(), ("0.01".to_string(), None));
        assert_eq!(RationalNumber::parse("19/270").unwrap().as_decimal_str(), ("0.0703".to_string(), Some(3)));
        assert_eq!(RationalNumber::parse("0").unwrap().as_decimal_str(), ("0".to_string(), None));
        assert_eq!(RationalNumber::parse("4/2").unwrap().as_decimal_str(), ("2".to_string(), None));
        assert_eq!(RationalNumber::parse("4/3").unwrap().as_decimal_str(), ("1.3".to_string(), Some(1)));
        assert_eq!(RationalNumber::parse("10/3").unwrap().as_decimal_str(), ("3.3".to_string(), Some(1)));
        assert_eq!(RationalNumber::parse("1/7").unwrap().as_decimal_str(), ("0.142857".to_string(), Some(6)));
        assert_eq!(RationalNumber::parse("14/7").unwrap().as_decimal_str(), ("2".to_string(), None));
        assert_eq!(RationalNumber::parse("1/11").unwrap().as_decimal_str(), ("0.09".to_string(), Some(2)));
    }
}
