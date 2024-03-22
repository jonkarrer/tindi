pub use super::Trend;
use crate::TindiError;
pub use crate::{exponential_moving_average, simple_moving_average};

pub fn simple_moving_average_trend(data: &[f32], short_period: usize, long_period: usize) -> Trend {
    if data.len() < long_period {
        panic!("Less data Than Long Period")
    }
    let short = simple_moving_average(&data[data.len() - short_period..]);
    let long = simple_moving_average(&data[data.len() - long_period..]);

    if long < short {
        return Trend::Bullish;
    }
    Trend::Bearish
}

pub fn exponential_moving_average_trend(
    data: &[f32],
    short_period: usize,
    long_period: usize,
) -> Result<Trend, TindiError> {
    if data.len() < (long_period * 2) {
        panic!("Need at Least Twice the Long Period Of data for EMA Breakout")
    }
    let short_period =
        exponential_moving_average(&data[data.len() - (short_period * 2)..], short_period)?;
    let long_period =
        exponential_moving_average(&data[data.len() - (long_period * 2)..], long_period)?;

    if long_period < short_period {
        return Ok(Trend::Bullish);
    }
    Ok(Trend::Bearish)
}
