use super::{slope, Point};

/// # Angle
/// Calculate the angle of the slope between two price points in degrees. (x1, y1) and (x2, y2).
/// Use the intervals of an array for X, and price for Y.
///
/// ```no_run
/// let point1 = Point(1.0, 2.0);
/// let point2 = Point(3.0, 4.0);
/// let angle = angle_in_degrees(point1, point2);
/// assert_eq!(angle, 45.0);
/// ```

pub fn angle_in_degrees(start_point: Point, end_point: Point) -> f32 {
    let m = slope(start_point, end_point);
    dbg!(m);
    m.atan() * (180.0 / std::f32::consts::PI)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        let point1 = Point(7.0, 40.98);
        let point2 = Point(19.0, 42.67);
        let angle = angle_in_degrees(point1, point2);
        assert_eq!(angle, 45.0);
    }
}
