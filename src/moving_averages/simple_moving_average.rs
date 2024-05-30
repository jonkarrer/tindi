/// # Simple Moving Average
/// The (SMA) is a basic indicator that displays the average price of an asset over a
/// period of time.
///
/// ## Example
/// ```no_run
/// let data = vec![
///    71.9, 72.51, 70.38, 71.63, 71.5, 71.11, 71.56, 70.34, 70.32, 70.05, 67.72, 66.45,
///    67.12, 66.86, 66.7, 67.26, 67.52, 68.0, 67.43, 67.68, 68.86, 68.62, 67.27, 67.9, 67.74,
///    66.45, 65.78, 66.88, 67.13, 66.65, 66.77, 65.86, 66.63, 65.55, 65.24, 64.74, 64.56,
///    64.37, 63.06, 62.32, 63.67, 64.81, 65.23, 64.33, 64.73, 64.55, 63.94, 65.15, 66.18,
///    67.65, 68.12, 67.9, 68.55, 67.13, 66.71, 66.34, 68.59, 68.24, 68.39, 69.34, 69.06,
/// ];
///
/// let result = simple_moving_average(&data);
/// let expected = 67.29558;
/// assert_eq!(result, expected);   
/// ```

pub fn simple_moving_average(data: &[f32]) -> f32 {
    let mut sum = 0.0;
    let length = data.len();

    if length < 1 {
        return 0.0;
    }

    for number in data {
        sum += number
    }

    sum / data.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma_wholes() {
        let data = vec![
            71.9, 72.51, 70.38, 71.63, 71.5, 71.11, 71.56, 70.34, 70.32, 70.05, 67.72, 66.45,
            67.12, 66.86, 66.7, 67.26, 67.52, 68.0, 67.43, 67.68, 68.86, 68.62, 67.27, 67.9, 67.74,
            66.45, 65.78, 66.88, 67.13, 66.65, 66.77, 65.86, 66.63, 65.55, 65.24, 64.74, 64.56,
            64.37, 63.06, 62.32, 63.67, 64.81, 65.23, 64.33, 64.73, 64.55, 63.94, 65.15, 66.18,
            67.65, 68.12, 67.9, 68.55, 67.13, 66.71, 66.34, 68.59, 68.24, 68.39, 69.34, 69.06,
        ];
        let result = simple_moving_average(&data);
        let expected = 67.29558;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sma_decimals() {
        let data = vec![0.21, 0.40, 0.60, 0.15, 0.21];
        let result = simple_moving_average(&data);
        let expected = 0.314;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sma_short() {
        let data = vec![0.21];
        let result = simple_moving_average(&data);
        let expected = 0.21;
        assert_eq!(result, expected);
    }
}
