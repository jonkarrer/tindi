use super::Trend;

pub fn single_bar_trend(open_price: f32, close_price: f32) -> Trend {
    let signal = close_price - open_price;

    if signal > 0.0 {
        return Trend::Bullish;
    } else {
        return Trend::Bearish;
    }
}
