/**
 ** Channel Breakout
 *
 *  Support and resistance lines are drawn at the high and low of the historic price
 *  .. and a recent high above or low below is a channel breakout
 */
use super::Trend;

pub fn has_channel_breakout(prices: &[f32], window: usize, target_trend: &Trend) -> bool {
    if prices.len() < window {
        panic!("Window is Larger Than Prices Length")
    }

    let mut high_line_price = 0.0;
    let mut low_line_price = 0.0;

    let past_prices = &prices[..prices.len() - window];
    for &price in past_prices {
        if price > high_line_price {
            high_line_price = price
        }
        if price < low_line_price {
            low_line_price = price
        }
    }

    let mut window_high_price = 0.0;
    let mut window_low_price = 0.0;

    let recent_prices = &prices[prices.len() - window..];
    for &price in recent_prices {
        if price > window_high_price {
            window_high_price = price
        }
        if price < window_low_price {
            window_low_price = price
        }
    }
    match target_trend {
        Trend::Bullish => {
            if window_high_price > high_line_price {
                return true;
            } else {
                return false;
            }
        }
        Trend::Bearish => {
            if window_low_price < low_line_price {
                return true;
            } else {
                return false;
            }
        }
    }
}
