#![allow(dead_code)]

mod rational_number;
mod math;

#[derive(Debug, Copy, Clone)]
pub enum LibError {
    ParseError,
}

