/**
 ** Standard Deviation
 *
 * The average plus and average minus (deviations) of prices over a period
 */

pub fn standard_deviation(prices: &[f32]) -> f32 {
    let length = prices.len() as f32;
    let mean = prices.iter().sum::<f32>() / length;
    let variance = prices
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f32>()
        / (length - 1.0);
    variance.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_deviation_results() {
        let data = vec![
            53.73, 53.87, 53.85, 53.88, 54.08, 54.14, 54.5, 54.3, 54.4, 54.16,
        ];
        let result = standard_deviation(&data);
        dbg!(&result);
        let expect = 0.257227;

        assert_eq!(result, expect);
    }
}
