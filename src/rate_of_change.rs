pub fn calculate_rate_of_change(prices: &[f32], n: usize) -> f32 {
    if prices.len() < n {
        panic!("Period is Longer Than Number of Prices for Rate Of Change")
    }
    let price_n_periods_ago = prices[prices.len() - n];
    let price_now = prices[prices.len() - 1];
    ((price_now - price_n_periods_ago) / price_n_periods_ago) * 100.0
}
