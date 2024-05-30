use std::io::{Error, ErrorKind};

/// # Rate of Change
/// A technical indicator that is used to identify the trend of a stock.
/// It is composed of a percentage change between the current price and the price n periods ago.
///
/// ## Example
/// ```no_run
/// let roc = rate_of_change(&[40.0, 43.0, 44.0, 50.0], 4).unwrap();
/// assert_eq!(roc, 25.0);
/// ```

pub fn rate_of_change(data: &[f32], period: usize) -> Result<f32, Error> {
    if data.len() < period {
        return Err(Error::new(
            ErrorKind::Other,
            format!("ROC: Given {}, Need {}", data.len(), period,),
        ));
    }

    let price_n_periods_ago = data[data.len() - period];
    let price_now = data[data.len() - 1];

    Ok(((price_now - price_n_periods_ago) / price_n_periods_ago) * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_of_change() {
        let roc = rate_of_change(&[40.0, 43.0, 44.0, 50.0], 4).unwrap();
        assert_eq!(roc, 25.0);
    }
}
