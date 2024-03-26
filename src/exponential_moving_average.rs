use std::io::{Error, ErrorKind};

pub fn exponential_moving_average(data: &[f32], period: usize) -> Result<f32, Error> {
    if data.len() < period {
        return Err(Error::new(
            ErrorKind::Other,
            format!("EMA: Given {}, Need {}", data.len(), period,),
        ));
    }

    let mut ema = 0.0;
    let alpha = 2.0 / (period as f32 + 1.0);

    // Inital SMA value for data set
    for i in 0..period {
        ema += data[i];
    }
    ema /= period as f32;

    // EMA calculation
    for i in period..data.len() {
        ema = data[i] * alpha + ema * (1.0 - alpha);
    }

    Ok(ema)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ema_result() {
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
            36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
        ];

        let result = exponential_moving_average(&data, 12).unwrap();
        let expected = 33.297977;
        dbg!(&result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ema_error() {
        let data: Vec<f32> = vec![
            10.0, 12.0, 13.0, 15.0, 16.0, 18.0, 17.0, 16.0, 19.0, 20.0, 22.0, 23.0, 25.0, 24.0,
            22.0, 20.0, 18.0, 19.0, 21.0, 23.0, 24.0, 26.0, 27.0, 25.0, 23.0, 22.0, 20.0, 18.0,
            19.0, 21.0, 23.0,
        ];

        let res = exponential_moving_average(&data, 41);

        dbg!(&res);

        assert!(res.is_err());
    }
}
