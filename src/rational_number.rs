use std::{fmt, ops};
use std::ops::{Neg, Add};
use regex::Regex;
use std::str::FromStr;
use crate::{math, LibError};
use crate::math::lcm;

const MIXED_NUMER_RE: &str = r"([+-]?)(?:\s*)(\d+)(?:\s+)(\d+)(?:\s*)/(?:\s*)(\d+)(?:\s*)";
const FRACTION_RE: &str = r"([+-]?)(?:\s*)(\d+)(?:\s*)/(?:\s*)(\d+)(?:\s*)";
const DECIMAL_RE: &str = r"([+-]?)(?:\s*)(\d+).?(\d*)";

pub enum NumberPrintFormat {
    Decimal,
    Fraction,
    Mixed,
}

#[derive(Debug, Copy, Clone)]
pub struct RationalNumber {
    pub numerator: u32,
    pub denominator: u32,
    pub negative: bool,
}

impl RationalNumber {
    pub fn as_f32(&self) -> f32 {
        let mut f = (self.numerator as f32) / (self.denominator as f32);
        if self.negative {
            f = f.neg();
        }
        f
    }

    pub fn as_i32(&self) -> Result<i32, LibError> {
        if self.numerator % self.denominator != 0 {
            Err(LibError::ParseError)
        } else {
            let mut n = (self.numerator / self.denominator) as i32;
            if self.negative {
                n = n.neg();
            }
            Ok(n)
        }
    }

    pub fn parse(s: &str) -> Result<Self, LibError> {
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
            })
        } else if let Some(captures) = Regex::new(FRACTION_RE).unwrap().captures(s) {
            let negative_str = captures.get(1).unwrap().as_str();
            let numerator_str = captures.get(2).unwrap().as_str();
            let denominator_str = captures.get(3).unwrap().as_str();

            let negative = negative_str == "-";
            let numerator = u32::from_str(&numerator_str).unwrap();
            let denominator = u32::from_str(&denominator_str).unwrap();

            Ok(RationalNumber {
                negative,
                numerator,
                denominator,
            })
        } else if let Some(captures) = Regex::new(DECIMAL_RE).unwrap().captures(s) {
            let negative_str = captures.get(1).unwrap().as_str();
            let whole_str = captures.get(2).unwrap().as_str();
            let remainder_str = captures.get(3).unwrap().as_str();

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
            })
        } else {
            Err(LibError::ParseError)
        }
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
        }
    }

    pub fn neg(&self) -> RationalNumber {
        RationalNumber {
            negative: !self.negative,
            numerator: self.numerator,
            denominator: self.denominator,
        }
    }

    pub fn reciprocal(&self) -> RationalNumber {
        RationalNumber {
            negative: self.negative,
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    pub fn as_str(&self, format: NumberPrintFormat) -> String {
        match format {
            NumberPrintFormat::Decimal => {
                let remainder = self.numerator % self.denominator;
                if remainder == 0 {
                    (self.numerator / self.denominator).to_string()
                } else {
                    self.as_f32().to_string()
                }
            }
            NumberPrintFormat::Fraction => {
                if self.negative {
                    format!("-{}/{}", self.numerator, self.denominator)
                } else {
                    format!("{}/{}", self.numerator, self.denominator)
                }
            }
            NumberPrintFormat::Mixed => {
                let whole = self.numerator / self.denominator;
                let remainder = self.numerator % self.denominator;
                if self.numerator < self.denominator {
                    self.as_str(NumberPrintFormat::Fraction)
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
        }
    }
}

impl From<i32> for RationalNumber {
    fn from(n: i32) -> Self {
        RationalNumber {
            numerator: n.abs() as u32,
            denominator: 1,
            negative: n < 0,
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
                numerator = self.numerator - rhs.numerator;
                negative = self.negative;
            } else {
                numerator = rhs.numerator - self.numerator;
                negative = rhs.negative;
            }
        }
        RationalNumber {
            numerator,
            denominator,
            negative,
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
            denominator : self.denominator * rhs.denominator,
            negative: self.negative != rhs.negative,
        }
    }
}

impl ops::Div<RationalNumber> for RationalNumber {
    type Output = RationalNumber;

    fn div(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber {
            numerator: self.numerator * rhs.denominator,
            denominator : self.denominator * rhs.numerator,
            negative: self.negative != rhs.negative,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::ops::Neg;
    use crate::rational_number::{RationalNumber, NumberPrintFormat};

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
        assert_eq!(
            RationalNumber::parse("1/3")
                .unwrap()
                .as_str(NumberPrintFormat::Fraction),
            "1/3"
        );
        assert_eq!(
            RationalNumber::parse("7/5")
                .unwrap()
                .as_str(NumberPrintFormat::Fraction),
            "7/5"
        );
        assert_eq!(
            RationalNumber::parse("10/20")
                .unwrap()
                .as_str(NumberPrintFormat::Fraction),
            "10/20"
        );
    }

    #[test]
    fn parses_specific_mixed_numbers() {
        assert_eq!(
            RationalNumber::parse("2 1/5")
                .unwrap()
                .as_str(NumberPrintFormat::Mixed),
            "2 1/5"
        );
        assert_eq!(
            RationalNumber::parse("-2 1/5")
                .unwrap()
                .as_str(NumberPrintFormat::Mixed),
            "-2 1/5"
        );
        assert_eq!(
            RationalNumber::parse("5 4/10")
                .unwrap()
                .as_str(NumberPrintFormat::Mixed),
            "5 4/10"
        );
    }

    #[test]
    fn parses_random_mixed_numbers() {
        let mut rng = rand::thread_rng();
        for _i in 0..100 {
            let whole = rng.gen_range(1u32, 10_000u32);
            let denominator = rng.gen_range(1u32, 10_000u32);
            let numerator = rng.gen_range(1, denominator);

            let neg_str = if rng.gen_bool(0.5) { "-" } else { "" };

            let mixed_number_str = format!("{}{} {}/{}", neg_str, whole, numerator, denominator);
            println!("{}", mixed_number_str);
            assert_eq!(
                RationalNumber::parse(&mixed_number_str)
                    .unwrap()
                    .as_str(NumberPrintFormat::Mixed),
                mixed_number_str
            );
        }
    }
}