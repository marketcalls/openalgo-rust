//! Strategy webhook module for OpenAlgo (TradingView-style webhook poster).
//!
//! Mirrors Python's standalone `Strategy` class: unlike the rest of this crate,
//! [`Strategy`] is not part of [`crate::OpenAlgo`] — it is its own small, independent
//! client that POSTs a signal to `{host_url}/strategy/webhook/{webhook_id}`. The
//! strategy mode (LONG_ONLY, SHORT_ONLY, BOTH) is configured on the OpenAlgo server,
//! not in the SDK call.

use crate::client::OpenAlgoError;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
struct StrategyOrderPayload {
    symbol: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    position_size: Option<String>,
}

/// Standalone TradingView-style strategy webhook poster.
///
/// # Example
/// ```rust,no_run
/// use openalgo::Strategy;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let strategy = Strategy::new("http://127.0.0.1:5000", "your-webhook-id");
/// let result = strategy.strategy_order("RELIANCE", "BUY", None).await?;
/// println!("{:?}", result);
/// # Ok(())
/// # }
/// ```
pub struct Strategy {
    host_url: String,
    webhook_id: String,
    http_client: Client,
}

impl Strategy {
    /// Create a new strategy webhook client.
    ///
    /// * `host_url` - OpenAlgo server URL (e.g. `"http://127.0.0.1:5000"`)
    /// * `webhook_id` - Strategy's webhook ID from OpenAlgo
    pub fn new(host_url: &str, webhook_id: &str) -> Self {
        Self {
            host_url: host_url.trim_end_matches('/').to_string(),
            webhook_id: webhook_id.to_string(),
            http_client: Client::new(),
        }
    }

    /// The full webhook URL this client posts to.
    pub fn webhook_url(&self) -> String {
        format!("{}/strategy/webhook/{}", self.host_url, self.webhook_id)
    }

    /// Send a strategy order via webhook to OpenAlgo.
    ///
    /// The strategy mode (LONG_ONLY, SHORT_ONLY, BOTH) is configured in OpenAlgo,
    /// not passed here.
    ///
    /// * `symbol` - Trading symbol (e.g., "RELIANCE", "NIFTY")
    /// * `action` - Order action ("BUY" or "SELL")
    /// * `position_size` - Position size, required for BOTH mode
    pub async fn strategy_order(
        &self,
        symbol: &str,
        action: &str,
        position_size: Option<i64>,
    ) -> Result<Value, OpenAlgoError> {
        let payload = StrategyOrderPayload {
            symbol: symbol.to_string(),
            action: action.to_uppercase(),
            position_size: position_size.map(|p| p.to_string()),
        };

        let response = self
            .http_client
            .post(self.webhook_url())
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(OpenAlgoError::ApiError(format!(
                "HTTP {} - {}",
                status, text
            )));
        }

        let result: Value = serde_json::from_str(&text)?;
        Ok(result)
    }
}
