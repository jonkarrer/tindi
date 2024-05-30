use crate::exponential_moving_average;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

/// # Moving Average Convergence Divergence
/// The MACD is a trend-following momentum indicator that shows the relationship between
/// two Exponential Moving Averages. The MACD is calculated by subtracting the 26-period
/// exponential moving average from the 12-period exponential moving average.
///
/// ## Example
/// ```no_run
/// let data = vec![
///   35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
///   33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
///   36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
/// ];
///
/// let result = MovingAverageConvergenceDivergence::new(&data).unwrap();
/// let expect = MovingAverageConvergenceDivergence {
///     baseline: vec![
///         1.1245346,
///         1.0470009,
///         1.0006447,
///         0.70882416,
///         0.31655502,
///         0.0064086914,
///         -0.14411926,
///         -0.2731743,
///         -0.3432541,
///     ],
///     signal: 0.38260227,
///     hist: -0.72585636,
/// };
///
/// assert_eq!(result, expect);
/// ```

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MovingAverageConvergenceDivergence {
    pub baseline: Vec<f32>,
    pub signal: f32,
    pub hist: f32,
}

impl MovingAverageConvergenceDivergence {
    pub fn new(data: &[f32]) -> Result<Self, Error> {
        let data_len_needed = 26 + 9;
        if data.len() < data_len_needed {
            return Err(Error::new(
                ErrorKind::Other,
                format!("MACD: Given {}, Need {}", data.len(), data_len_needed,),
            ));
        }

        let mut baseline = Vec::new();

        // Using the last 9 values in the data, fill the MACD vec
        for i in 0..9 {
            // Need to walk backwards through the data to get present day MACD line
            // * Full length, full length - 1, full length -2, etc...
            let target_data = &data[0..data.len() - i];

            // MACD line = 12-period EMA - 26-period EMA
            let short_period = exponential_moving_average(&target_data, 12)?;
            let long_period = exponential_moving_average(&target_data, 26)?;
            let macd = short_period - long_period;

            // Place value at the start of the vec on each loop. This ensures
            // ... that the most recent MACD point is at the end of the vec
            baseline.insert(0, macd);
        }

        let signal = exponential_moving_average(&baseline, 9)?;
        let hist = baseline.last().unwrap() - signal;
        Ok(Self {
            baseline,
            signal,
            hist,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macd_result() {
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
            36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
        ];

        let result = MovingAverageConvergenceDivergence::new(&data).unwrap();
        dbg!(&result);
        let expect = MovingAverageConvergenceDivergence {
            baseline: vec![
                1.1245346,
                1.0470009,
                1.0006447,
                0.70882416,
                0.31655502,
                0.0064086914,
                -0.14411926,
                -0.2731743,
                -0.3432541,
            ],
            signal: 0.38260227,
            hist: -0.72585636,
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn test_macd_error() {
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11,
        ];
        let res = MovingAverageConvergenceDivergence::new(&data);
        assert!(res.is_err())
    }
}
