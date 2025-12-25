//! Account API module for OpenAlgo.

use crate::client::{OpenAlgoClient, OpenAlgoError};
use crate::types::*;
use std::sync::Arc;

/// Account API client
pub struct AccountAPI {
    client: Arc<OpenAlgoClient>,
}

impl AccountAPI {
    /// Create a new Account API client
    pub fn new(client: Arc<OpenAlgoClient>) -> Self {
        Self { client }
    }

    /// Get account funds
    pub async fn funds(&self) -> Result<FundsResponse, OpenAlgoError> {
        let request = FundsRequest {
            apikey: self.client.api_key.clone(),
        };

        self.client.post("funds", &request).await
    }

    /// Get orderbook
    pub async fn orderbook(&self) -> Result<OrderbookResponse, OpenAlgoError> {
        let request = OrderbookRequest {
            apikey: self.client.api_key.clone(),
        };

        self.client.post("orderbook", &request).await
    }

    /// Get tradebook
    pub async fn tradebook(&self) -> Result<TradebookResponse, OpenAlgoError> {
        let request = TradebookRequest {
            apikey: self.client.api_key.clone(),
        };

        self.client.post("tradebook", &request).await
    }

    /// Get positionbook
    pub async fn positionbook(&self) -> Result<PositionbookResponse, OpenAlgoError> {
        let request = PositionbookRequest {
            apikey: self.client.api_key.clone(),
        };

        self.client.post("positionbook", &request).await
    }

    /// Get holdings
    pub async fn holdings(&self) -> Result<HoldingsResponse, OpenAlgoError> {
        let request = HoldingsRequest {
            apikey: self.client.api_key.clone(),
        };

        self.client.post("holdings", &request).await
    }

    /// Get margin requirement
    ///
    /// # Arguments
    ///
    /// * `positions` - Vector of positions to calculate margin for
    pub async fn margin(
        &self,
        positions: Vec<MarginPosition>,
    ) -> Result<MarginResponse, OpenAlgoError> {
        let request = MarginRequest {
            apikey: self.client.api_key.clone(),
            positions,
        };

        self.client.post("margin", &request).await
    }
}
