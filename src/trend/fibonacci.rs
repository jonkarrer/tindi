use super::Trend;

#[derive(Debug)]
pub struct FibonacciLevels {
    pub trend: Trend,
    pub first_level: f32,  // 23.6%
    pub second_level: f32, // 38.2%
    pub third_level: f32,  // 50.0%
    pub fourth_level: f32, // 61.8%
    pub fifth_level: f32,  // 78.6%
}

pub fn calc_fib_retrace(high: f32, low: f32, trend: Trend) -> FibonacciLevels {
    let range = high - low;
    if trend == Trend::Bearish {
        return FibonacciLevels {
            trend: trend.clone(),
            first_level: low + (range * 0.236),
            second_level: low + (range * 0.382),
            third_level: low + (range * 0.5),
            fourth_level: low + (range * 0.618),
            fifth_level: low + (range * 0.786),
        };
    }

    return FibonacciLevels {
        trend: trend.clone(),
        first_level: high - (range * 0.236),
        second_level: high - (range * 0.382),
        third_level: high - (range * 0.5),
        fourth_level: high - (range * 0.618),
        fifth_level: high - (range * 0.786),
    };
}
