use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a trading pair (e.g., SOL/USDT)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradingPair {
    pub base: String,  // e.g., "SOL"
    pub quote: String, // e.g., "USDT"
}

impl fmt::Display for TradingPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.base, self.quote)
    }
}

impl TradingPair {
    pub fn new(base: &str, quote: &str) -> Self {
        Self {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_trading_pair_creation() {
        let pair = TradingPair::new("sol", "usdt");
        assert_eq!(pair.base, "SOL");
        assert_eq!(pair.quote, "USDT");
    }

    #[test]
    fn test_trading_pair_display() {
        let pair = TradingPair::new("SOL", "USDT");
        assert_eq!(pair.to_string(), "SOL/USDT");

        let pair = TradingPair::new("btc", "usd");
        assert_eq!(pair.to_string(), "BTC/USD");

        let pair = TradingPair::new("eth", "btc");
        assert_eq!(pair.to_string(), "ETH/BTC");
    }

    #[test]
    fn test_trading_pair_serialization() {
        let pair = TradingPair::new("SOL", "USDT");
        let json = serde_json::to_string(&pair).unwrap();

        assert!(json.contains("\"base\""));
        assert!(json.contains("\"quote\""));
        assert!(json.contains("\"SOL\""));
        assert!(json.contains("\"USDT\""));
    }

    #[test]
    fn test_trading_pair_deserialization() {
        let json = r#"{"base":"BTC","quote":"USDT"}"#;
        let pair: TradingPair = serde_json::from_str(json).unwrap();

        assert_eq!(pair.base, "BTC");
        assert_eq!(pair.quote, "USDT");
    }
}
