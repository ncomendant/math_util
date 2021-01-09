use serde::{Serialize, Deserialize};

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

pub fn round_i64(num: i64, place_value: PlaceValue) -> i64 {
    let factor = 10f64.powi(place_value.into());
    let result = (num as f64 / factor).round() * factor;
    result as i64
}

pub fn round_f64(num: f64, place_value: PlaceValue) -> String {
    println!("round {} {:?}", num, place_value);
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