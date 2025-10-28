use crate::types::{Exchange, TradingPair};
use serde::{Deserialize, Serialize};

/// Represents price information from an exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub exchange: Exchange,
    pub pair: TradingPair,
    pub bid: f64, // Best bid price
    pub ask: f64, // Best ask price
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Price {
    pub fn mid_price(&self) -> f64 {
        (self.bid + self.ask) / 2.0
    }

    pub fn spread(&self) -> f64 {
        self.ask - self.bid
    }

    pub fn spread_percentage(&self) -> f64 {
        (self.spread() / self.mid_price()) * 100.0
    }
}
