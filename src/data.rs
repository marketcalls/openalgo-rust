//! Data API module for OpenAlgo.

use crate::client::{OpenAlgoClient, OpenAlgoError};
use crate::types::*;
use std::sync::Arc;

/// Data API client
pub struct DataAPI {
    client: Arc<OpenAlgoClient>,
}

impl DataAPI {
    /// Create a new Data API client
    pub fn new(client: Arc<OpenAlgoClient>) -> Self {
        Self { client }
    }

    /// Get quotes for a symbol
    pub async fn quotes(
        &self,
        symbol: &str,
        exchange: &str,
    ) -> Result<QuotesResponse, OpenAlgoError> {
        let request = QuotesRequest {
            apikey: self.client.api_key.clone(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
        };

        self.client.post("quotes", &request).await
    }

    /// Get quotes for multiple symbols (simplified API with tuples)
    pub async fn multi_quotes(
        &self,
        symbols: &[(&str, &str)],
    ) -> Result<MultiQuotesResponse, OpenAlgoError> {
        let symbols_vec: Vec<MultiQuotesSymbol> = symbols
            .iter()
            .map(|(symbol, exchange)| MultiQuotesSymbol::new(symbol, exchange))
            .collect();

        let request = MultiQuotesRequest {
            apikey: self.client.api_key.clone(),
            symbols: symbols_vec,
        };

        self.client.post("multiquotes", &request).await
    }

    /// Get market depth for a symbol
    pub async fn depth(
        &self,
        symbol: &str,
        exchange: &str,
    ) -> Result<DepthResponse, OpenAlgoError> {
        let request = DepthRequest {
            apikey: self.client.api_key.clone(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
        };

        self.client.post("depth", &request).await
    }

    /// Get historical data (simple form - latest data)
    pub async fn history(
        &self,
        symbol: &str,
        exchange: &str,
        interval: &str,
    ) -> Result<serde_json::Value, OpenAlgoError> {
        let request = HistoryRequest {
            apikey: self.client.api_key.clone(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            interval: interval.to_string(),
            start_date: None,
            end_date: None,
        };

        self.client.post("history", &request).await
    }

    /// Get historical data with date range
    pub async fn history_range(
        &self,
        symbol: &str,
        exchange: &str,
        interval: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<serde_json::Value, OpenAlgoError> {
        let request = HistoryRequest {
            apikey: self.client.api_key.clone(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            interval: interval.to_string(),
            start_date: Some(start_date.to_string()),
            end_date: Some(end_date.to_string()),
        };

        self.client.post("history", &request).await
    }

    /// Get available intervals
    pub async fn intervals(&self) -> Result<IntervalsResponse, OpenAlgoError> {
        let request = IntervalsRequest {
            apikey: self.client.api_key.clone(),
        };

        self.client.post("intervals", &request).await
    }

    /// Get option chain
    pub async fn option_chain(
        &self,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
    ) -> Result<OptionChainResponse, OpenAlgoError> {
        let request = OptionChainRequest {
            apikey: self.client.api_key.clone(),
            underlying: underlying.to_string(),
            exchange: exchange.to_string(),
            expiry_date: expiry_date.to_string(),
            strike_count: None,
        };

        self.client.post("optionchain", &request).await
    }

    /// Get option chain with strike count
    pub async fn option_chain_strikes(
        &self,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
        strike_count: i32,
    ) -> Result<OptionChainResponse, OpenAlgoError> {
        let request = OptionChainRequest {
            apikey: self.client.api_key.clone(),
            underlying: underlying.to_string(),
            exchange: exchange.to_string(),
            expiry_date: expiry_date.to_string(),
            strike_count: Some(strike_count),
        };

        self.client.post("optionchain", &request).await
    }

    /// Get symbol info
    pub async fn symbol(
        &self,
        symbol: &str,
        exchange: &str,
    ) -> Result<SymbolResponse, OpenAlgoError> {
        let request = SymbolRequest {
            apikey: self.client.api_key.clone(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
        };

        self.client.post("symbol", &request).await
    }

    /// Search symbols
    pub async fn search(
        &self,
        query: &str,
        exchange: &str,
    ) -> Result<SearchResponse, OpenAlgoError> {
        let request = SearchRequest {
            apikey: self.client.api_key.clone(),
            query: query.to_string(),
            exchange: exchange.to_string(),
        };

        self.client.post("search", &request).await
    }

    /// Get option symbol
    pub async fn option_symbol(
        &self,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
        offset: &str,
        option_type: &str,
    ) -> Result<OptionSymbolResponse, OpenAlgoError> {
        let request = OptionSymbolRequest {
            apikey: self.client.api_key.clone(),
            underlying: underlying.to_string(),
            exchange: exchange.to_string(),
            expiry_date: expiry_date.to_string(),
            offset: offset.to_string(),
            option_type: option_type.to_string(),
        };

        self.client.post("optionsymbol", &request).await
    }

    /// Get synthetic future price
    pub async fn synthetic_future(
        &self,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
    ) -> Result<SyntheticFutureResponse, OpenAlgoError> {
        let request = SyntheticFutureRequest {
            apikey: self.client.api_key.clone(),
            underlying: underlying.to_string(),
            exchange: exchange.to_string(),
            expiry_date: expiry_date.to_string(),
        };

        self.client.post("syntheticfuture", &request).await
    }

    /// Get option Greeks
    pub async fn option_greeks(
        &self,
        symbol: &str,
        exchange: &str,
        interest_rate: f64,
        underlying_symbol: &str,
        underlying_exchange: &str,
    ) -> Result<OptionGreeksResponse, OpenAlgoError> {
        let request = OptionGreeksRequest {
            apikey: self.client.api_key.clone(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            interest_rate,
            underlying_symbol: underlying_symbol.to_string(),
            underlying_exchange: underlying_exchange.to_string(),
        };

        self.client.post("optiongreeks", &request).await
    }

    /// Get expiry dates
    pub async fn expiry(
        &self,
        symbol: &str,
        exchange: &str,
        instrumenttype: &str,
    ) -> Result<ExpiryResponse, OpenAlgoError> {
        let request = ExpiryRequest {
            apikey: self.client.api_key.clone(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            instrumenttype: instrumenttype.to_string(),
        };

        self.client.post("expiry", &request).await
    }

    /// Get instruments
    pub async fn instruments(
        &self,
        exchange: &str,
    ) -> Result<InstrumentsResponse, OpenAlgoError> {
        let request = InstrumentsRequest {
            apikey: self.client.api_key.clone(),
            exchange: exchange.to_string(),
        };

        self.client.post("instruments", &request).await
    }
}
