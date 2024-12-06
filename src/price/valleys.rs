/// # Price Valleys
/// A price valley is a price on a chart that has a slope down followed by a slope up. The lowest price between 2 slopes is considered the valley.
/// Given a slice of data and the slope tolerance, a price valley can be found like so
///
/// ```no_run
/// let valleys = find_price_valley(&[5.0, 6.0, 8.0, 9.0, 6.0, 8.0, 7.0]); // Some([9])
/// assert!(valleys.is_some_and(|x| x == 9.0));
/// ```

pub fn find_price_valley(prices: &[f32]) -> Option<f32> {
    let slope_size = prices.len() / 2;
    let middle_price = prices[slope_size];

    for p in prices {
        if p < &middle_price {
            return None;
        }
    }

    Some(middle_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_price_valley() {
        let valleys = find_price_valley(&[5.0, 6.0, 8.0, 1.0, 6.0, 8.0, 7.0]);
        assert!(valleys.is_some_and(|x| x == 1.0));
    }
}
