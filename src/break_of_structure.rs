use super::Trend;

#[allow(dead_code)]
pub struct Breaker {
    high: f32,
    low: f32,
    trend: Trend,
    timestamp: String,
}

pub fn find_breakers(prices: &[f32]) {
    let mut higher_highs: Vec<&f32> = Vec::new();
    let mut higher_lows: Vec<f32> = Vec::new();
    let mut lower_lows: Vec<&f32> = Vec::new();
    let mut lower_highs: Vec<f32> = Vec::new();
    let mut breakers: Vec<f32> = Vec::new();

    let mut i = 1;

    // Establish an achor point for comparison later on
    let mut higher_low_anchor = 0.0;
    let mut lower_high_anchor = 0.0;
    loop {
        if i == prices.len() {
            break;
        }

        //TODO Need to use high and low depending on trend
        let prev_price = &prices[i - 1];
        let curr_price = &prices[i];

        // Seed initial values
        if curr_price > prev_price {
            //First time through and a trend is not yet established
            if higher_highs.len() == 0 && lower_lows.len() == 0 {
                higher_highs.push(curr_price);
                higher_lows.push(*prev_price);
            }
        } else {
            //First time through and a trend is not yet established
            if lower_lows.len() == 0 && higher_highs.len() == 0 {
                lower_lows.push(curr_price);
                lower_highs.push(*prev_price);
            }
        }

        // !! BULL TREND RULES
        let are_in_bull_trend = higher_highs.len() != 0;
        if are_in_bull_trend {
            if curr_price > prev_price {
                // Get last higher high for comparison
                let last_higher_high = higher_highs[higher_highs.len() - 1];
                // If current price is greater than that, that means this is the new HH
                if curr_price > last_higher_high {
                    higher_highs.push(curr_price);
                    // The last lowest price before the new high was the higher low
                    if higher_low_anchor != 0.0 {
                        higher_lows.push(higher_low_anchor);
                    }
                    // Reset the higher low anchor back to 0 to start a fresh series track
                    higher_low_anchor = 0.0;
                }
            }
            if curr_price < prev_price {
                let last_higher_high = higher_highs[higher_highs.len() - 1];
                let last_higher_low = higher_lows[higher_lows.len() - 1];

                // This means a breaker has been found
                if curr_price < &last_higher_low {
                    // Required: 2 hh and 1 hl is the minimum requirement for a bull trend
                    //TODO this is not good enough, need a better way to identify a true HH or LL
                    if higher_highs.len() >= 2 && higher_lows.len() >= 1 {
                        dbg!(&higher_highs);
                        dbg!(&higher_lows);
                        breakers.push(last_higher_low);
                    }

                    // Trend has to be reset
                    higher_highs.clear();
                    higher_lows.clear();
                    higher_low_anchor = 0.0;
                }

                if curr_price < last_higher_high {
                    // If this is the first low after a new high
                    if higher_low_anchor == 0.0 {
                        // Set the new achor after the new high
                        higher_low_anchor = *curr_price;
                    }
                    // If price is below the current achor and below the current high
                    if curr_price < &higher_low_anchor {
                        // Set the new achor point below the previous one that was broken
                        higher_low_anchor = *curr_price;
                    }
                }
            }
        }

        // !! BEAR TREND RULES
        let are_in_bear_trend = lower_lows.len() != 0;
        if are_in_bear_trend {
            if curr_price < prev_price {
                let last_lower_low = lower_lows[lower_lows.len() - 1];
                // If current price is less than that, that means this is the new LL
                if curr_price < last_lower_low {
                    lower_lows.push(curr_price);
                    // The last highest price before the new low was the lower high
                    if lower_high_anchor != 0.0 {
                        lower_highs.push(lower_high_anchor);
                    }
                    // Reset the lower high anchor back to 0 to start a fresh series track
                    lower_high_anchor = 0.0;
                }
            }
            if curr_price > prev_price {
                let last_lower_low = lower_lows[lower_lows.len() - 1];
                let last_lower_high = lower_highs[lower_highs.len() - 1];

                // This means a breaker has been found
                if curr_price > &last_lower_high {
                    // !! Required: 2 hh and 1 hl is the minimum requirement for a bull trend
                    //TODO Busted
                    if lower_lows.len() >= 2 && lower_highs.len() >= 1 {
                        dbg!(&lower_lows);
                        dbg!(&lower_highs);
                        breakers.push(last_lower_high);
                    }
                    // Trend has to be reset
                    lower_lows.clear();
                    lower_highs.clear();
                    lower_high_anchor = 0.0;
                }

                if curr_price > last_lower_low {
                    // If this is the first high after a new low
                    if lower_high_anchor == 0.0 {
                        dbg!(&curr_price);
                        // Set the new achor after the new low
                        lower_high_anchor = *curr_price;
                    }
                    // If price is above the current achor and above the current low
                    if curr_price > &lower_high_anchor {
                        // Set the new achor point above the previous one that was broken
                        lower_high_anchor = *curr_price;
                    }
                }
            }
        }
        i += 1;
    }
    dbg!(&higher_highs);
    dbg!(&higher_lows);
    dbg!(&lower_highs);
    dbg!(&lower_lows);
    dbg!(&breakers);
}

impl Breaker {
    pub fn new(high: f32, low: f32, timestamp: &str, trend: Trend) -> Self {
        Self {
            high,
            low,
            trend,
            timestamp: timestamp.to_string(),
        }
    }
}
