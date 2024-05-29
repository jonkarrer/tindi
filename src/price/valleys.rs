/// # Price Valleys
/// A price valley is a price on a chart that has a slope down followed by a slope up. The lowest price between 2 slopes is considered the valley.
/// Given a slice of data and the slope tolerance, a price valley can be found like so
///
/// ```no_run
/// let valleys = find_price_valleys(&[5.0, 6.0, 8.0, 9.0, 6.0, 8.0, 7.0, 12.0], 3); // Some([9])
/// assert!(valleys.is_some_and(|x| x[0] == 9.0));
/// ```

pub fn find_price_valleys(data: &[f32], slope: usize) -> Option<Vec<f32>> {
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
        let valleys = find_price_valleys(&[5.0, 6.0, 8.0, 1.0, 6.0, 8.0, 7.0, 12.0], 3);
        dbg!(&valleys);
        assert!(valleys.is_some_and(|x| x[0] == 1.0));
    }

    #[test]
    fn test_find_price_valleys_five() {
        let valleys = find_price_valleys(
            &[
                5.0, 6.0, 8.0, 9.0, 6.0, 8.0, 7.0, 1.0, 5.0, 8.0, 9.0, 10.0, 5.0, 0.0, 4.0, 2.0,
                4.0, 5.0, 20.0,
            ],
            5,
        );
        assert!(valleys.is_some_and(|x| x[0] == 1.0));
    }
}
