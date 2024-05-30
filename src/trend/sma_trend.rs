use crate::{simple_moving_average, Trend};
use std::io::{Error, ErrorKind};

/// # Simple Moving Average Trend
/// This indicator is based on the simple moving average (SMA).
/// It is used to determine if the trend is bullish or bearish
/// by comparing the SMA of the short period with the SMA of the long period.
///
/// ## Example
/// ```no_run
/// let data = vec![
///     71.9, 72.51, 70.38, 71.63, 71.5, 71.11, 71.56, 70.34, 70.32, 70.05, 67.72, 66.45,
///     67.12, 66.86, 66.7, 67.26, 67.52, 68.0, 67.43, 67.68, 68.86, 68.62, 67.27, 67.9, 67.74,
///     67.65, 68.12, 67.9, 68.55, 67.13, 66.71, 66.34, 68.59, 68.24, 68.39, 69.34, 69.06,
///     100.00, 100.00, 120.00, 110.00, 120.22,
/// ];
///
/// let trend = simple_moving_average_trend(&data, 6, 20).unwrap();
/// assert_eq!(trend, Trend::Bullish);
/// ```

pub fn simple_moving_average_trend(
    data: &[f32],
    short_period: usize,
    long_period: usize,
) -> Result<Trend, Error> {
    if data.len() < long_period {
        return Err(Error::new(
            ErrorKind::Other,
            format!("SMA Trend: Given {}, Need {}", data.len(), long_period,),
        ));
    }

    let short = simple_moving_average(&data[data.len() - short_period..]);
    let long = simple_moving_average(&data[data.len() - long_period..]);

    if long < short {
        return Ok(Trend::Bullish);
    }

    Ok(Trend::Bearish)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_moving_average_trend() {
        let data = vec![
            71.9, 72.51, 70.38, 71.63, 71.5, 71.11, 71.56, 70.34, 70.32, 70.05, 67.72, 66.45,
            67.12, 66.86, 66.7, 67.26, 67.52, 68.0, 67.43, 67.68, 68.86, 68.62, 67.27, 67.9, 67.74,
            67.65, 68.12, 67.9, 68.55, 67.13, 66.71, 66.34, 68.59, 68.24, 68.39, 69.34, 69.06,
            100.00, 100.00, 120.00, 110.00, 120.22,
        ];

        let trend = simple_moving_average_trend(&data, 6, 20).unwrap();

        assert_eq!(trend, Trend::Bullish);
    }
}
