use std::fmt;

#[derive(Debug)]
pub enum MathError {
    OutOfRange(String),
    NotEnoughData(String),
    DivisionByZero,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::NotEnoughData(msg) => {
                write!(
                    f,
                    "The input data has too few data points to compute: {}",
                    msg
                )
            }
            MathError::OutOfRange(msg) => {
                write!(
                    f,
                    "The input data is shorter than the period length: {}",
                    msg
                )
            }
            MathError::DivisionByZero => {
                write!(f, "Tried to divide by zero",)
            }
        }
    }
}
