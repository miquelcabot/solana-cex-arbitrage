use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents different exchanges we support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Exchange {
    Binance,
    Solana,
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exchange::Binance => write!(f, "Binance"),
            Exchange::Solana => write!(f, "Solana"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_exchange_display() {
        assert_eq!(Exchange::Binance.to_string(), "Binance");
        assert_eq!(Exchange::Solana.to_string(), "Solana");
    }

    #[test]
    fn test_exchange_serialization() {
        // Test JSON serialization
        let binance_json = serde_json::to_string(&Exchange::Binance).unwrap();
        assert_eq!(binance_json, "\"Binance\"");

        let solana_json = serde_json::to_string(&Exchange::Solana).unwrap();
        assert_eq!(solana_json, "\"Solana\"");
    }

    #[test]
    fn test_exchange_deserialization() {
        // Test JSON deserialization
        let binance: Exchange = serde_json::from_str("\"Binance\"").unwrap();
        assert_eq!(binance, Exchange::Binance);

        let solana: Exchange = serde_json::from_str("\"Solana\"").unwrap();
        assert_eq!(solana, Exchange::Solana);
    }
}
