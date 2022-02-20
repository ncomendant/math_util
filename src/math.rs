use rand::{Rng, prelude::SliceRandom};
use serde::{Serialize, Deserialize};

const VARIABLES: &str = "abcdefghijkmnpqrstuvwxyz";

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

pub fn rand_variable<R: Rng + ?Sized>(rng: &mut R) -> char {
    *VARIABLES
        .chars()
        .into_iter()
        .collect::<Vec<_>>()
        .choose(rng)
        .expect("failed to choose variable")
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