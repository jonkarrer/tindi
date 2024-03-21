pub fn rate_of_change(prices: &[f32], periods: usize) -> Option<f32> {
    if prices.len() < periods {
        return None;
    }
    let price_n_periods_ago = prices[prices.len() - periods];
    let price_now = prices[prices.len() - 1];

    Some(((price_now - price_n_periods_ago) / price_n_periods_ago) * 100.0)
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
