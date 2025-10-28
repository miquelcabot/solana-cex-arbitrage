use crate::types::{Exchange, Price, TradingPair};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

/// Generic trait that all exchange clients must implement
#[async_trait]
pub trait ExchangeClient: Send + Sync {
    /// Get the exchange identifier
    fn exchange(&self) -> Exchange;

    /// Start monitoring the specified trading pairs
    /// Sends price updates through the provided channel
    async fn start_monitoring(
        &mut self,
        pairs: &[TradingPair],
        price_sender: mpsc::UnboundedSender<Price>,
    ) -> Result<()>;

    /// Stop monitoring (graceful shutdown)
    async fn stop_monitoring(&mut self) -> Result<()>;

    /// Check if the client is currently connected and monitoring
    fn is_monitoring(&self) -> bool;

    /// Get supported trading pairs for this exchange
    async fn get_supported_pairs(&self) -> Result<Vec<TradingPair>>;
}
