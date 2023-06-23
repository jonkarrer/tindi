pub mod bar_trends;
pub use super::bar_trends::*;

pub mod averages;
pub use super::averages::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Trend {
    Bullish,
    Bearish,
}
