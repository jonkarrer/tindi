/**
 ** Simple Moving Average
 *
 * The mean of the stock prices over a period
 */

pub fn simple_moving_average(prices: &[f32]) -> f32 {
    let mut sum = 0.0;
    for number in prices {
        sum += number
    }
    sum / prices.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma_wholes() {
        let data = vec![4.21, 9.40, 15.0, 18.1, 21.0];
        let result = simple_moving_average(&data);
        let expected = 13.542;
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
