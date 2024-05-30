use crate::{simple_moving_average, standard_deviation};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

/// # Bollinger Bands
/// A technical indicator that is used to identify the trend of a stock.
/// It is composed of three lines: the top band, middle band, and bottom band.
/// The top and bottom bands are typically 2 standard deviations above and below the middle band.
/// The middle band is a simple moving average of the price.
///
/// ## Example
/// ```no_run
/// let data = vec![
///     35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
///     33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
///     36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
/// ];
/// let periods = 20;
/// let result = BollingerBands::new(&data, period).unwrap();
///
/// let expect = BollingerBands {
///    top_band: 38.211624,
///     mid_band: 34.3955,
///     bottom_band: 30.579376,
/// };
///
/// assert_eq!(result, expect);
/// ```
///

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BollingerBands {
    pub top_band: f32,
    pub mid_band: f32,
    pub bottom_band: f32,
}

impl BollingerBands {
    pub fn new(data: &[f32], period: usize) -> Result<Self, Error> {
        if data.len() < period {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Bollinger: Given {}, Need at least 2 items", data.len()),
            ));
        }

        let prices = &data[data.len() - period..];

        let mean = simple_moving_average(&prices);
        let std_dev = standard_deviation(&prices);

        let k = std_dev * 2.0;
        let top_band = mean + k;
        let bottom_band = mean - k;

        Ok(Self {
            top_band,
            mid_band: mean,
            bottom_band,
        })
    }

    pub fn near_bottom_band_bollinger_band(&self, recent_price: f32, tolerance: f32) -> bool {
        let range = recent_price * tolerance;
        let distance_from_bottom_band_band = (recent_price - self.bottom_band).abs();
        if distance_from_bottom_band_band < range {
            return true;
        }
        false
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
        let result = BollingerBands::new(&data, 20).unwrap();
        dbg!(&result);
        let expect = BollingerBands {
            top_band: 38.211624,
            mid_band: 34.3955,
            bottom_band: 30.579376,
        };

        assert_eq!(result, expect);
    }

    #[test]
    fn test_ema_error() {
        let data: Vec<f32> = vec![10.0, 12.0, 13.0];

        let res = BollingerBands::new(&data, 10);
        assert!(res.is_err())
    }
}
