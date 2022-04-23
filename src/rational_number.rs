use crate::{Result, Error, PlaceValue};
use regex::Regex;
use std::cmp::Ordering;
use std::ops::{Add, Neg};
use std::str::FromStr;
use std::{fmt, ops};
use serde::{Serialize, Deserialize};

const MIXED_NUMER_RE: &str = r"^(?:\s*)([+-]?)(?:\s*)(\d+)(?:\s+)(\d+)(?:\s*)/(?:\s*)(\d+)(?:\s*)$";
const FRACTION_RE: &str = r"^(?:\s*)([+-]?)(?:\s*)(\d+)(?:\s*)/(?:\s*)(\d+)(?:\s*)$";
const DECIMAL_RE: &str = r"^(?:\s*)([+-]?)(?:\s*)(\d+)\.?(\d*)(?:\s*)$";
const DECIMAL_RE_2: &str = r"^(?:\s*)([+-]?)(?:\s*)\.(\d+)(?:\s*)$";

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum NumberDisplayFormat {
    Decimal(Option<PlaceValue>),
    Fraction,
    Mixed,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RationalNumber {
    pub numerator: u32,
    pub denominator: u32,
    pub negative: bool,
    pub format: NumberDisplayFormat,
}

impl RationalNumber {
    pub fn new(numerator: u32, denominator: u32, negative: bool, format: NumberDisplayFormat) -> Self {
        RationalNumber {
            numerator,
            denominator,
            negative,
            format
        }
    }

    pub fn as_f32(&self) -> f32 {
        let mut f = (self.numerator as f32) / (self.denominator as f32);
        if self.negative {
            f = f.neg();
        }
        f
    }

    pub fn as_i32(&self) -> Result<i32> {
        if self.numerator % self.denominator != 0 {
            Err(Error::ParseError)
        } else {
            let mut n = (self.numerator / self.denominator) as i32;
            if self.negative {
                n = n.neg();
            }
            Ok(n)
        }
    }

    pub fn parse(s: &str) -> Result<Self> {
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
            Err(Error::ParseError)
        }
    }

    fn parse_decimal(negative_str: &str, whole_str: &str, remainder_str: &str) -> Result<Self> {
        let negative = negative_str == "-";
        let whole = u32::from_str(whole_str.trim_start_matches('0')).unwrap();
        let remainder = if let Ok(r) = u32::from_str(&remainder_str.trim_end_matches('0')) {
            r
        } else {
            0
        };
        let denominator = 10u32.pow(remainder_str.len() as u32);

        let f = crate::gcf(remainder, denominator);

        let remainder = remainder / f;
        let denominator = denominator / f;

        let numerator = denominator * whole + remainder;

        Ok(RationalNumber {
            numerator,
            denominator,
            negative,
            format: NumberDisplayFormat::Decimal(None),
        })
    }

    pub fn pow(&self, exp: &RationalNumber) -> RationalNumber {
        self.as_f32().powf(exp.as_f32()).into()
    }

    pub fn display_format(&self) -> NumberDisplayFormat {
        self.format
    }

    pub fn set_display_format(&self, format: NumberDisplayFormat) -> RationalNumber {
        RationalNumber {
            numerator: self.numerator,
            denominator: self.denominator,
            negative: self.negative,
            format,
        }
    }

    pub fn simplify(&self) -> RationalNumber {
        let gcf = crate::gcf(self.numerator, self.denominator);
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

    pub fn abs(&self) -> RationalNumber {
        RationalNumber {
            negative: false,
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

    // ignores specified digits after decimal
    pub fn as_decimal_str(&self) -> (String, Option<usize>) { // decimal, repeating_digit_count
        let negative_str = if self.negative {
            "-"
        } else {
            ""
        };
        let mut s = format!("{}{}", negative_str, (self.numerator / self.denominator));
        let mut remainder = self.numerator % self.denominator;
        if remainder != 0 {
            s.push_str(".");
            let mut remainders = Vec::new();
            remainders.push(remainder);
            while remainder != 0 {
                remainder *= 10;
                let digit_str = (remainder / self.denominator).to_string();
                remainder %= self.denominator;
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
        if self.numerator == 0 {
            return "0".to_string();
        }

        let format = match format {
            Some(f) => f,
            None => self.format,
        };

        match format {
            NumberDisplayFormat::Decimal(place_value) => {
                let (original_str, repeating_digit_count) = self.as_decimal_str();
                if let Some(place_value) = place_value {
                    let mut s = original_str.clone();
                    let decimal_position = s.chars().position(|c| c == '.').unwrap_or_else(|| {
                        s.push_str(".");
                        s.len() - 1
                    });
                    let digits_needed = (place_value as i64 * -1) - (s.len() - decimal_position - 1) as i64 + 1; // one extra for rounding
                    if digits_needed > 0 {
                        let digits_needed = digits_needed as usize;
                        if let Some(repeating_digit_count) = repeating_digit_count {
                            for i in 0..digits_needed {
                                let position = decimal_position + (i % repeating_digit_count) + 1;
                                let next_digit = original_str.chars().nth(position).expect("failed to get repeating digit");
                                s.push_str(&next_digit.to_string());
                            }
                        } else {
                            for _i in 0..digits_needed {
                                s.push_str("0");
                            }
                        }
                    }
                    // round if needed
                    let num = f64::from_str(&s).expect("failed to parse float");
                    return crate::round_f64(num, place_value)
                } else {
                    if let Some(repeating_digit_count) = repeating_digit_count {
                        let split_index = original_str.len() - repeating_digit_count;
                        format!("{}bar{}", &original_str[..split_index], &original_str[split_index..])
                    } else {
                        original_str
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
                if remainder == 0 && whole == 0 {
                    "0".to_string()
                } else if self.numerator < self.denominator {
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

impl PartialEq for RationalNumber {
    fn eq(&self, other: &Self) -> bool {
        let a = self.simplify();
        let b = other.simplify();
        a.numerator == b.numerator && a.denominator == b.denominator && a.negative == b.negative
    }
}

impl PartialOrd for RationalNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = if self.negative && !other.negative {
            std::cmp::Ordering::Less
        } else if !self.negative && other.negative {
            std::cmp::Ordering::Greater
        } else if self.negative || other.negative {
            let lcm = crate::lcm(self.denominator, other.denominator);
            let a = lcm/self.denominator;
            let b = lcm/other.denominator;
            (other.numerator * b).cmp(&(self.numerator * a))
        } else { // both are positive
            let lcm = crate::lcm(self.denominator, other.denominator);
            let a = lcm/self.denominator;
            let b = lcm/other.denominator;
            (self.numerator * a).cmp(&(other.numerator * b))
        };
        Some(ord)
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
            format: NumberDisplayFormat::Decimal(None),
        }
    }
}

impl From<i32> for RationalNumber {
    fn from(n: i32) -> Self {
        RationalNumber {
            numerator: n.abs() as u32,
            denominator: 1,
            negative: n < 0,
            format: NumberDisplayFormat::Decimal(None),
        }
    }
}

impl fmt::Display for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str(None))
    }
}

impl ops::Add<u32> for RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: u32) -> Self::Output {
        self + RationalNumber::from(rhs)
    }
}

impl ops::Add<RationalNumber> for u32 {
    type Output = RationalNumber;

    fn add(self, rhs: RationalNumber) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<f32> for RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: f32) -> Self::Output {
        self + RationalNumber::from(rhs)
    }
}

impl ops::Add<RationalNumber> for f32 {
    type Output = RationalNumber;

    fn add(self, rhs: RationalNumber) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<i32> for RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: i32) -> Self::Output {
        self + RationalNumber::from(rhs)
    }
}

impl ops::Add<RationalNumber> for i32 {
    type Output = RationalNumber;

    fn add(self, rhs: RationalNumber) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<RationalNumber> for RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: RationalNumber) -> Self::Output {
        let denominator = crate::lcm(self.denominator, rhs.denominator);
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

impl ops::Sub<u32> for RationalNumber {
    type Output = RationalNumber;

    fn sub(self, rhs: u32) -> Self::Output {
        self - RationalNumber::from(rhs)
    }
}

impl ops::Sub<RationalNumber> for u32 {
    type Output = RationalNumber;

    fn sub(self, rhs: RationalNumber) -> Self::Output {
        rhs - RationalNumber::from(self)
    }
}

impl ops::Sub<i32> for RationalNumber {
    type Output = RationalNumber;

    fn sub(self, rhs: i32) -> Self::Output {
        self - RationalNumber::from(rhs)
    }
}

impl ops::Sub<RationalNumber> for i32 {
    type Output = RationalNumber;

    fn sub(self, rhs: RationalNumber) -> Self::Output {
        rhs - RationalNumber::from(self)
    }
}

impl ops::Sub<f32> for RationalNumber {
    type Output = RationalNumber;

    fn sub(self, rhs: f32) -> Self::Output {
        self - RationalNumber::from(rhs)
    }
}

impl ops::Sub<RationalNumber> for f32 {
    type Output = RationalNumber;

    fn sub(self, rhs: RationalNumber) -> Self::Output {
        rhs - RationalNumber::from(self)
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

impl ops::Mul<u32> for RationalNumber {
    type Output = RationalNumber;

    fn mul(self, rhs: u32) -> Self::Output {
        self * RationalNumber::from(rhs)
    }
}

impl ops::Mul<RationalNumber> for u32 {
    type Output = RationalNumber;

    fn mul(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber::from(self) * rhs
    }
}

impl ops::Mul<i32> for RationalNumber {
    type Output = RationalNumber;

    fn mul(self, rhs: i32) -> Self::Output {
        self * RationalNumber::from(rhs)
    }
}

impl ops::Mul<RationalNumber> for i32 {
    type Output = RationalNumber;

    fn mul(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber::from(self) * rhs
    }
}


impl ops::Mul<f32> for RationalNumber {
    type Output = RationalNumber;

    fn mul(self, rhs: f32) -> Self::Output {
        self * RationalNumber::from(rhs)
    }
}

impl ops::Mul<RationalNumber> for f32 {
    type Output = RationalNumber;

    fn mul(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber::from(self) * rhs
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

impl ops::Div<u32> for RationalNumber {
    type Output = RationalNumber;

    fn div(self, rhs: u32) -> Self::Output {
        self/RationalNumber::from(rhs)
    }
}

impl ops::Div<RationalNumber> for u32 {
    type Output = RationalNumber;

    fn div(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber::from(rhs)/self
    }
}

impl ops::Div<i32> for RationalNumber {
    type Output = RationalNumber;

    fn div(self, rhs: i32) -> Self::Output {
        self/RationalNumber::from(rhs)
    }
}

impl ops::Div<RationalNumber> for i32 {
    type Output = RationalNumber;

    fn div(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber::from(rhs)/self
    }
}

impl ops::Div<f32> for RationalNumber {
    type Output = RationalNumber;

    fn div(self, rhs: f32) -> Self::Output {
        self/RationalNumber::from(rhs)
    }
}

impl ops::Div<RationalNumber> for f32 {
    type Output = RationalNumber;

    fn div(self, rhs: RationalNumber) -> Self::Output {
        RationalNumber::from(rhs)/self
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
    use crate::PlaceValue;

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
                .as_str(Some(NumberDisplayFormat::Decimal(None))),
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
            let whole = rng.gen_range(1u32..=10_000u32);
            let denominator = rng.gen_range(2u32..=10_000u32);
            let numerator = rng.gen_range(1..=denominator);

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

    #[test]
    fn prints_string_decimals() {
        assert_eq!(RationalNumber::parse("1/5").unwrap().as_str(Some(NumberDisplayFormat::Decimal(None))), "0.2");
        assert_eq!(RationalNumber::parse("-1/4").unwrap().as_str(Some(NumberDisplayFormat::Decimal(None))), "-0.25");
        assert_eq!(RationalNumber::parse("1/3").unwrap().as_str(Some(NumberDisplayFormat::Decimal(Some(PlaceValue::Hundredths)))), "0.33");
        assert_eq!(RationalNumber::parse("7/11").unwrap().as_str(Some(NumberDisplayFormat::Decimal(Some(PlaceValue::TenThousandths)))), "0.6364");
        assert_eq!(RationalNumber::parse("21/5").unwrap().as_str(Some(NumberDisplayFormat::Decimal(Some(PlaceValue::Thousandths)))), "4.200");
        assert_eq!(RationalNumber::parse("7/11").unwrap().as_str(Some(NumberDisplayFormat::Decimal(None))), "0.bar63");
        assert_eq!(RationalNumber::parse("1/7").unwrap().as_str(Some(NumberDisplayFormat::Decimal(None))), "0.bar142857");
    }
}
