mod peaks;
pub use peaks::*;

mod valleys;
pub use valleys::*;

mod average_true_range;
pub use average_true_range::*;

pub fn find_low(numbers: &[f32]) -> f32 {
    let mut min = numbers[0];
    for &num in &numbers[1..] {
        if num < min {
            min = num;
        }
    }
    min
}

pub fn find_high(numbers: &[f32]) -> f32 {
    let mut max = numbers[0];
    for &num in &numbers[1..] {
        if num > max {
            max = num;
        }
    }
    max
}
