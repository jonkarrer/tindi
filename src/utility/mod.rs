/// # Price Peaks
/// A price peak is a price on a chart that has sloping sides. The highest price between 2 slopes (up slope and down slope) is considered the peak.
/// Given a slice of data and the slope tolerance, a price peak can be found like so
///
/// ```no_run
/// fn test_find_price_peaks_() {
///     let peaks = find_price_peaks(&[5, 6, 8, 9, 6, 8, 7, 12], 3); // Some([9])
///     assert!(peaks.is_some_and(|x| x[0] == 9));
/// }
/// ```

pub fn find_price_peaks(data: &[i32], slope: usize) -> Option<Vec<i32>> {
    let mut peaks = Vec::new();

    for (i, peak) in data[slope..].iter().enumerate() {
        if (slope * 2) + (i + 1) > data.len() {
            break;
        }

        let left_slope_less = data[i..slope + i].iter().all(|&x| x < *peak);
        let right_slope_less = data[i + slope + 1..(slope * 2) + (i + 1)]
            .iter()
            .all(|&x| x < *peak);

        if left_slope_less && right_slope_less {
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
    fn test_find_price_peaks_() {
        let peaks = find_price_peaks(&[5, 6, 8, 9, 6, 8, 7, 12], 3);
        assert!(peaks.is_some_and(|x| x[0] == 9));
    }
}
