use serde::{Deserialize, Serialize};

mod standard_deviation;
pub use standard_deviation::*;

mod channel_breakout;
pub use channel_breakout::*;

mod sma_trend;
pub use sma_trend::*;

mod ema_trend;
pub use ema_trend::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Trend {
    Bullish,
    Bearish,
}
