/// # Price Peaks
/// A price peak is a price on a chart that has a slope up followed by a slope down. The highest price between 2 slopes is considered the peak.
/// Given a slice of data and the slope tolerance, a price peak can be found like so
///
/// ```no_run
/// fn test_find_price_peaks_() {
///     let peaks = find_price_peaks(&[5.0, 6.0, 8.0, 9.0, 6.0, 8.0, 7.0, 12.0], 3); // Some([9])
///     assert!(peaks.is_some_and(|x| x[0] == 9.0));
/// }
/// ```

pub fn find_price_peaks(data: &[f32], slope: usize) -> Option<Vec<f32>> {
    let mut peaks = Vec::new();

    for (i, peak) in data[slope..].iter().enumerate() {
        let comparison_length = (slope * 2) + (i + 1);

        if comparison_length > data.len() {
            break;
        }

        // Take entire slice then remove the peak number
        let mut combined_slopes = data[i..comparison_length].to_vec();
        combined_slopes.swap_remove(slope);

        // Peak should be higher than numbers in slice
        if combined_slopes.iter().all(|x| x < peak) {
            peaks.push(*peak);
        }
    }

    if peaks.len() > 0 {
        return Some(peaks);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_price_peaks_three() {
        let peaks = find_price_peaks(&[5.0, 6.0, 8.0, 9.0, 6.0, 8.0, 7.0, 12.0], 3);
        assert!(peaks.is_some_and(|x| x[0] == 9.0));
    }

    #[test]
    fn test_find_price_peaks_five() {
        let peaks = find_price_peaks(
            &[
                5.0, 6.0, 8.0, 9.0, 6.0, 8.0, 7.0, 12.0, 5.0, 8.0, 9.0, 10.0, 5.0, 0.0, 4.0, 2.0,
                4.0, 5.0, 20.0,
            ],
            5,
        );
        assert!(peaks.is_some_and(|x| x[0] == 12.0));
    }
}
