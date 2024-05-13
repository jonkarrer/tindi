pub fn find_price_peaks(data: &[i32], slope_size: usize) -> Option<Vec<i32>> {
    // [5, 6, 8, 9, 6, 8, 9];
    let mut peaks = Vec::new();
    for (i, item) in data[slope_size..].iter().enumerate() {
        let mut this_number_is_a_peak = true;
        for z in i..slope_size + i {
            let is_less_than_peak = item - data[z] > 0;

            if is_less_than_peak {
                continue;
            } else {
                this_number_is_a_peak = false;
                break;
            }
        }

        if this_number_is_a_peak {
            peaks.push(item);
        }
    }
    dbg!(peaks);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_price_peaks() {
        find_price_peaks(&[5, 6, 8, 9, 6, 8, 9, 12], 3);
    }
}
