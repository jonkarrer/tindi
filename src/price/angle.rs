///Calculate the angle of the slope between two prices
/// Convert to degrees
///
/// ```no_run
/// fn test_angle() {
///     let angle = angle(5.0);
///     assert!(angle == 0.0);
/// }
/// ```

pub fn angle(m: f32) -> f32 {
    m.atan() * (180.0 / std::f32::consts::PI)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        let angle = angle(0.4636);
        assert_eq!(angle, 0.0);
    }
}
