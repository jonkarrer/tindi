/// # Price Peaks
/// A price peak is a price on a chart that has a slope up followed by a slope down. The highest price between 2 slopes is considered the peak.
/// Given a slice of data and the slope tolerance, a price peak can be found like so
///
/// ```no_run
/// let peaks = find_price_peaks(&[5.0, 6.0, 8.0, 9.0, 6.0, 8.0, 7.0]); // Some([9])
/// assert!(peaks.is_some_and(|x| x == 9.0));
/// ```

pub fn find_price_peak(prices: &[f32]) -> Option<f32> {
    let slope_size = prices.len() / 2;
    let middle_price = prices[slope_size];

    for p in prices {
        if p > &middle_price {
            return None;
        }
    }

    Some(middle_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_price_peak() {
        let peaks = find_price_peak(&[5.0, 6.0, 8.0, 9.0, 6.0, 8.0, 7.0]);
        assert!(peaks.is_some_and(|x| x == 9.0));
    }
}
