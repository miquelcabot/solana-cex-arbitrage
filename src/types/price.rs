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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json;

    fn create_test_price() -> Price {
        Price {
            exchange: Exchange::Binance,
            pair: TradingPair::new("SOL", "USDT"),
            bid: 100.0,
            ask: 101.0,
            timestamp: Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap(),
        }
    }

    fn create_test_price_with_values(bid: f64, ask: f64) -> Price {
        Price {
            exchange: Exchange::Solana,
            pair: TradingPair::new("BTC", "USDC"),
            bid,
            ask,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_price_creation() {
        let price = create_test_price();
        assert_eq!(price.exchange, Exchange::Binance);
        assert_eq!(price.pair.base, "SOL");
        assert_eq!(price.pair.quote, "USDT");
        assert_eq!(price.bid, 100.0);
        assert_eq!(price.ask, 101.0);
    }

    #[test]
    fn test_mid_price_calculation() {
        let price = create_test_price_with_values(100.0, 102.0);
        assert_eq!(price.mid_price(), 101.0);

        let price = create_test_price_with_values(50.0, 60.0);
        assert_eq!(price.mid_price(), 55.0);

        let price = create_test_price_with_values(99.99, 100.01);
        assert_eq!(price.mid_price(), 100.0);
    }

    #[test]
    fn test_spread_calculation() {
        let price = create_test_price_with_values(100.0, 101.0);
        assert_eq!(price.spread(), 1.0);

        let price = create_test_price_with_values(50.0, 55.0);
        assert_eq!(price.spread(), 5.0);

        let price = create_test_price_with_values(99.5, 100.5);
        assert_eq!(price.spread(), 1.0);
    }

    #[test]
    fn test_spread_percentage_calculation() {
        let price = create_test_price_with_values(100.0, 101.0);
        // Spread: 1.0, Mid: 100.5, Percentage: (1.0 / 100.5) * 100 ≈ 0.995%
        let expected = (1.0 / 100.5) * 100.0;
        assert!((price.spread_percentage() - expected).abs() < 1e-10);

        let price = create_test_price_with_values(100.0, 102.0);
        // Spread: 2.0, Mid: 101.0, Percentage: (2.0 / 101.0) * 100 ≈ 1.98%
        let expected = (2.0 / 101.0) * 100.0;
        assert!((price.spread_percentage() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_price_serialization() {
        let price = create_test_price();
        let json = serde_json::to_string(&price).unwrap();

        // Should serialize all fields
        assert!(json.contains("\"exchange\""));
        assert!(json.contains("\"pair\""));
        assert!(json.contains("\"bid\""));
        assert!(json.contains("\"ask\""));
        assert!(json.contains("\"timestamp\""));
    }

    #[test]
    fn test_price_deserialization() {
        let json = r#"{
            "exchange": "Binance",
            "pair": {
                "base": "SOL",
                "quote": "USDT"
            },
            "bid": 100.0,
            "ask": 101.0,
            "timestamp": "2024-01-01T12:00:00Z"
        }"#;

        let price: Price = serde_json::from_str(json).unwrap();
        assert_eq!(price.exchange, Exchange::Binance);
        assert_eq!(price.pair.base, "SOL");
        assert_eq!(price.pair.quote, "USDT");
        assert_eq!(price.bid, 100.0);
        assert_eq!(price.ask, 101.0);
    }
}
