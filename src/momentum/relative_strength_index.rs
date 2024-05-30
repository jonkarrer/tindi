use crate::simple_moving_average;

/// # Relative Strength Index
/// The Relative Strength Index (RSI) is a momentum oscillator that measures
/// the speed and change of price movements. The RSI compares the magnitude
/// of recent price changes to that of a past period. The RSI ranges from
/// 0 to 100. The RSI is interpreted as an overbought/oversold indicator when
/// the value is over 70/below 30.
///
/// ## Example
/// ```no_run
/// let data: Vec<f32> = vec![
///     44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
///     45.61, 46.28, 46.28,
/// ];
/// let result = relative_strength_index(&data);
/// let expect = 70.46411;
///
/// assert_eq!(result, expect);
/// ```

pub fn relative_strength_index(data: &[f32]) -> f32 {
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

    // Calculate rsi
    let avg_gain = simple_moving_average(&gains);
    let avg_loss = simple_moving_average(&losses);

    if avg_loss == 0.0 {
        return 100.0 - (100.0 / (1.0 + avg_gain));
    }

    100.0 - (100.0 / (1.0 + (avg_gain / avg_loss)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_result() {
        // https://school.stockcharts.com/doku.php?id=technical_indicators:relative_strength_index_rsi
        let data: Vec<f32> = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28,
        ];
        let result = relative_strength_index(&data);
        dbg!(&result);
        let expect = 70.46411;

        assert_eq!(result, expect);
    }
}
