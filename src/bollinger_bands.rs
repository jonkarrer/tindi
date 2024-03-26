use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

use super::{simple_moving_average, standard_deviation};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BollingerBands {
    pub top: f32,
    pub mid: f32,
    pub bottom: f32,
}

impl BollingerBands {
    pub fn new(data: &[f32]) -> Result<Self, Error> {
        if data.len() < 4 {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Bollinger: Given {}, Need at least 2 items", data.len()),
            ));
        }
        let mut top = 0.0;
        let mut mid = 0.0;
        let mut bottom = 0.0;

        // * Period has to be half the amount of data points.
        let period: usize = data.len() / 2;

        for i in period..data.len() {
            // * Progressivly "climb up" the array one value at a time
            let offset = &i - &period;
            let prices = &data[offset..data.len()];
            let mean = simple_moving_average(&prices);
            let std_dev = standard_deviation(&prices);

            // * Get the plot point of this current target slice
            let k = std_dev * 2.0;
            let upper_plot = mean + k;
            let bottom_plot = mean - k;

            // * When the last number is hit, set the values
            if data.len() - &i == 1 {
                top = upper_plot;
                mid = mean;
                bottom = bottom_plot;
            }
        }
        Ok(Self { top, mid, bottom })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bollinger_bands_result() {
        // Data from XLF, weekly bars, Mon Aug 08, 22 -> Mon Apr 10, 23
        // ... used trading view chart and indicator.
        let data = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
            36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
        ];
        let result = BollingerBands::new(&data).unwrap();
        dbg!(&result);
        let expect = BollingerBands {
            top: 38.164547,
            mid: 34.314735,
            bottom: 30.464926,
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn test_ema_error() {
        let data: Vec<f32> = vec![10.0, 12.0, 13.0];

        let result = BollingerBands::new(&data);
        assert!(result.is_err())
    }
}
