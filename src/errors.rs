use std::fmt;

#[derive(Debug)]
pub enum TindiError {
    OutOfRange(String),
    NotEnoughData(String),
}

impl fmt::Display for TindiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TindiError::NotEnoughData(msg) => {
                write!(
                    f,
                    "The input data has too few data points to compute: {}",
                    msg
                )
            }
            TindiError::OutOfRange(msg) => {
                write!(
                    f,
                    "The input data is shorter than the period length: {}",
                    msg
                )
            }
        }
    }
}
