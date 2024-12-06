use serde::{Deserialize, Serialize};

mod standard_deviation;
pub use standard_deviation::*;

mod channel_breakout;
pub use channel_breakout::*;

mod sma_trend;
pub use sma_trend::*;

mod ema_trend;
pub use ema_trend::*;

mod fibonacci;
pub use fibonacci::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Trend {
    Bullish,
    Bearish,
}

impl ToString for Trend {
    fn to_string(&self) -> String {
        match self {
            Trend::Bullish => "bullish".to_string(),
            Trend::Bearish => "bearish".to_string(),
        }
    }
}
