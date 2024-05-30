use super::Trend;
use std::io::{Error, ErrorKind};

/// # Channel Breakout
/// This indicator checks if the price is above or below the high and low line of the past n periods.
/// The high line is the highest price of the past n periods.
/// The low line is the lowest price of the past n periods.
/// If the price is above the high line, the trend is bullish.
/// If the price is below the low line, the trend is bearish.
///
/// ## Example
/// ```no_run
/// let data = vec![
///     71.9, 72.51, 70.38, 71.63, 71.5, 71.11, 71.56, 70.34, 70.32, 70.05, 67.72, 66.45,
///     67.12, 66.86, 66.7, 67.26, 67.52, 68.0, 67.43, 67.68, 68.86, 68.62, 67.27, 67.9, 67.74,
///     66.45, 65.78, 66.88, 67.13, 66.65, 66.77, 65.86, 66.63, 65.55, 65.24, 64.74, 64.56,
///     64.37, 63.06, 62.32, 63.67, 64.81, 65.23, 64.33, 64.73, 64.55, 63.94, 65.15, 66.18,
///     67.65, 68.12, 67.9, 68.55, 67.13, 66.71, 66.34, 68.59, 68.24, 68.39, 90.34, 69.06,
/// ];
///
/// let result = has_channel_breakout(&data, 10, &Trend::Bullish).unwrap();
///
/// assert_eq!(result, true);
/// ```

pub fn has_channel_breakout(
    data: &[f32],
    window: usize,
    target_trend: &Trend,
) -> Result<bool, Error> {
    if data.len() < window {
        return Err(Error::new(
            ErrorKind::Other,
            format!("ChannelBreakout: Given {}, Need {}", data.len(), window,),
        ));
    }

    let mut high_line_price = 0.0;
    let mut low_line_price = 0.0;

    let past_data = &data[..data.len() - window];
    for &price in past_data {
        if price > high_line_price {
            high_line_price = price
        }
        if price < low_line_price {
            low_line_price = price
        }
    }

    let mut window_high_price = 0.0;
    let mut window_low_price = 0.0;

    let recent_data = &data[data.len() - window..];
    for &price in recent_data {
        if price > window_high_price {
            window_high_price = price
        }
        if price < window_low_price {
            window_low_price = price
        }
    }
    match target_trend {
        Trend::Bullish => {
            if window_high_price > high_line_price {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        Trend::Bearish => {
            if window_low_price < low_line_price {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_breakout() {
        let data = vec![
            71.9, 72.51, 70.38, 71.63, 71.5, 71.11, 71.56, 70.34, 70.32, 70.05, 67.72, 66.45,
            67.12, 66.86, 66.7, 67.26, 67.52, 68.0, 67.43, 67.68, 68.86, 68.62, 67.27, 67.9, 67.74,
            66.45, 65.78, 66.88, 67.13, 66.65, 66.77, 65.86, 66.63, 65.55, 65.24, 64.74, 64.56,
            64.37, 63.06, 62.32, 63.67, 64.81, 65.23, 64.33, 64.73, 64.55, 63.94, 65.15, 66.18,
            67.65, 68.12, 67.9, 68.55, 67.13, 66.71, 66.34, 68.59, 68.24, 68.39, 90.34, 69.06,
        ];

        let result = has_channel_breakout(&data, 10, &Trend::Bullish).unwrap();

        assert_eq!(result, true);
    }
}
