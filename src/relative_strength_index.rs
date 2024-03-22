use crate::{simple_moving_average, TindiError};

pub fn relative_strength_index(data: &[f32]) -> Result<Vec<f32>, TindiError> {
    let period = 14;
    if data.len() < period {
        return Err(TindiError::NotEnoughData(format!(
            "RSI: Given {}, Need {}",
            data.len(),
            period,
        )));
    }

    // Calculate gains and losses
    let mut gains = Vec::new();
    let mut losses = Vec::new();
    for i in 0..data.len() - 1 {
        let diff = &data[i + 1] - &data[i];
        if diff > 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            losses.push(diff.abs());
            gains.push(0.0);
        }
    }

    // Caclulate rsi
    let mut rsi_line = Vec::new();
    let iterations = data.len() - period;
    for i in 0..iterations {
        let avg_gain = simple_moving_average(&gains[i..14 + i]);
        let avg_loss = simple_moving_average(&losses[i..14 + i]);
        let rsi = 100.0 - (100.0 / (1.0 + (avg_gain / avg_loss)));
        rsi_line.push(rsi);
    }

    Ok(rsi_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_result() {
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
            36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
        ];
        let result = relative_strength_index(&data);
        dbg!(&result);
        let expect = 35.794403;
        let last = result.unwrap();

        assert_eq!(last[last.len() - 1], expect);
    }

    #[test]
    fn test_rsi_error() {
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 30.36, 30.89, 31.01, 32.19, 34.19, 30.0, 30.0,
        ];

        let rsi = relative_strength_index(&data);
        assert!(rsi.is_err())
    }
}
