/**
 ** Moving Average Convergence Divergence
 *
 * A momentum indicator that uses the exponential moving averages
 */
use crate::exponential_moving_average;

#[derive(Debug, PartialEq)]
pub struct MovingAverageConvergenceDivergence {
    pub baseline: Vec<f32>,
    pub signal: f32,
    pub gap: f32,
}

impl MovingAverageConvergenceDivergence {
    pub fn new(prices: &[f32]) -> Self {
        if prices.len() < 26 + 9 {
            panic!("Spread Length Is Too Short For MACD")
        }
        let mut baseline = Vec::new();

        // Using the last 9 values in the prices, fill the MACD vec
        for i in 0..9 {
            // Need to walk backwards through the prices to get present day MACD line
            // * Full length, full length - 1, full length -2, etc...
            let target_prices = &prices[0..prices.len() - i];

            // MACD line = 12-period EMA - 26-period EMA
            let short_period = exponential_moving_average(&target_prices, 12);
            let long_period = exponential_moving_average(&target_prices, 26);
            let macd = short_period - long_period;

            // Place value at the start of the vec on each loop. This ensures
            // ... that the most recent MACD point is at the end of the vec
            baseline.insert(0, macd);
        }

        let signal = exponential_moving_average(&baseline, 9);
        let gap = baseline.last().unwrap() - signal;
        Self {
            baseline,
            signal,
            gap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macd_result() {
        // Data from XLF, weekly bars, Mon Aug 08, 22 -> Mon Apr 10, 23
        // ... used trading view chart and indicator.
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
            36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
        ];

        // used https://www.easycalculation.com/finance/macd.php to test this.
        let result = MovingAverageConvergenceDivergence::new(&data);
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
            gap: -0.72585636,
        };

        // These are the values trading view had
        //let expect = (vec![0.6227, 0.5839, 0.5731, 0.3140, -0.0481, -0.3305, -0.4554, -0.5608, -0.6079], -0.1712);
        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic(expected = "Spread Length Is Too Short For MACD")]
    fn test_macd_error() {
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11,
        ];
        MovingAverageConvergenceDivergence::new(&data);
    }
}
