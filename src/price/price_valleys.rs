/// # Price Valleys
/// A price valley is a price on a chart that has a slope down followed by a slope up. The lowest price between 2 slopes is considered the valley.
/// Given a slice of data and the slope tolerance, a price valley can be found like so
///
/// ```no_run
/// fn test_find_price_valleys() {
///     let valleys = find_price_valleys(&[5, 6, 8, 9, 6, 8, 7, 12], 3); // Some([9])
///     assert!(valleys.is_some_and(|x| x[0] == 9));
/// }
/// ```

pub fn find_price_valleys(data: &[i32], slope: usize) -> Option<Vec<i32>> {
    let mut valleys = Vec::new();

    for (i, valley) in data[slope..].iter().enumerate() {
        let comparison_length = (slope * 2) + (i + 1);

        if comparison_length > data.len() {
            break;
        }

        // Take entire slice then remove the valley number
        let mut combined_slopes = data[i..comparison_length].to_vec();
        combined_slopes.swap_remove(slope);

        // Valley should be lower than numbers in slice
        if combined_slopes.iter().all(|x| x > valley) {
            valleys.push(*valley);
        }
    }

    if valleys.len() > 0 {
        return Some(valleys);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_price_valleys_three() {
        let valleys = find_price_valleys(&[5, 6, 8, 1, 6, 8, 7, 12], 3);
        dbg!(&valleys);
        assert!(valleys.is_some_and(|x| x[0] == 1));
    }

    #[test]
    fn test_find_price_valleys_five() {
        let valleys = find_price_valleys(
            &[5, 6, 8, 9, 6, 8, 7, 1, 5, 8, 9, 10, 5, 2, 4, 2, 4, 5, 20],
            5,
        );
        assert!(valleys.is_some_and(|x| x[0] == 1));
    }
}
