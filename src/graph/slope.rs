use super::Point;

/// # Slope
/// Find the slope between two points, (x1, y1) and (x2, y2)
/// Slope = (y2 - y1) / (x2 - x1), where x is time and y is price.
/// Use the intervals of your array for X, and price for Y.
///
/// ```no_run
/// let point1 = Point(1.0, 6.0);
/// let point2 = Point(3.0, 2.0);
/// assert_eq!(slope(point1, point2), -2.0);
/// ```

pub fn slope(start_point: Point, end_point: Point) -> f32 {
    let Point(x1, y1) = start_point;
    let Point(x2, y2) = end_point;
    (y2 - y1) / (x2 - x1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope() {
        let point1 = Point(1.0, 2.0);
        let point2 = Point(3.0, 4.0);
        assert_eq!(slope(point1, point2), 1.0);

        let point1 = Point(1.0, 4.0);
        let point2 = Point(3.0, 2.0);
        assert_eq!(slope(point1, point2), -1.0);

        let point1 = Point(1.0, 6.0);
        let point2 = Point(3.0, 2.0);
        assert_eq!(slope(point1, point2), -2.0);
    }
}
