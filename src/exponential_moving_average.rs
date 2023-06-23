/**
 ** Exponential Moving average
 *
 * Similar to a simple average, but weights the recent prices more heavily
 */

pub fn exponential_moving_average(spread: &[f32], period: usize) -> f32 {
    if spread.len() < period {
        panic!("Period For The EMA Is Out Of Bounds")
    }
    let mut ema = 0.0;
    let alpha = 2.0 / (period as f32 + 1.0);

    // Inital SMA value for data set
    for i in 0..period {
        ema += spread[i];
    }
    ema /= period as f32;

    // EMA calculation
    for i in period..spread.len() {
        ema = spread[i] * alpha + ema * (1.0 - alpha);
    }

    ema
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ema_result() {
        let data: Vec<f32> = vec![
            10.0, 12.0, 13.0, 15.0, 16.0, 18.0, 17.0, 16.0, 19.0, 20.0, 22.0, 23.0, 25.0, 24.0,
            22.0, 20.0, 18.0, 19.0, 21.0, 23.0, 24.0, 26.0, 27.0, 25.0, 23.0, 22.0, 20.0, 18.0,
            19.0, 21.0, 23.0,
        ];

        let result = exponential_moving_average(&data, 5);
        let expected = 21.329712;
        dbg!(&result);
        assert!(
            (result - expected).abs() < 1e-6,
            "EMA of the dataset with period 5 is incorrect"
        );
    }

    #[test]
    #[should_panic(expected = "Period For The EMA Is Out Of Bounds")]
    fn test_ema_error() {
        let data: Vec<f32> = vec![
            10.0, 12.0, 13.0, 15.0, 16.0, 18.0, 17.0, 16.0, 19.0, 20.0, 22.0, 23.0, 25.0, 24.0,
            22.0, 20.0, 18.0, 19.0, 21.0, 23.0, 24.0, 26.0, 27.0, 25.0, 23.0, 22.0, 20.0, 18.0,
            19.0, 21.0, 23.0,
        ];

        exponential_moving_average(&data, 41);
    }
}
