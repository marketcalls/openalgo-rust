//! Data API module for OpenAlgo.

use crate::client::{OpenAlgoClient, OpenAlgoError};
use crate::types::*;
use std::collections::HashMap;
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

    /// Legacy alias for [`DataAPI::intervals`]. Mirrors Python's `interval()`.
    pub async fn interval(&self) -> Result<IntervalsResponse, OpenAlgoError> {
        self.intervals().await
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

    /// Search symbols across exchanges
    ///
    /// * `exchange` - Optional exchange filter (NSE, NFO, BSE, BFO, MCX, CDS, BCD,
    ///   NCDEX, NSE_INDEX, BSE_INDEX, MCX_INDEX). Pass `None` to search all exchanges.
    /// * `extra` - Extra broker-specific fields forwarded verbatim, mirroring Python's `**kwargs`.
    pub async fn search(
        &self,
        query: &str,
        exchange: Option<&str>,
        extra: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<SearchResponse, OpenAlgoError> {
        let request = SearchRequest {
            apikey: self.client.api_key.clone(),
            query: query.to_string(),
            exchange: exchange.map(|s| s.to_string()),
            extra: extra.unwrap_or_default(),
        };

        self.client.post("search", &request).await
    }

    /// Get option symbol details by underlying + offset, without placing an order
    ///
    /// * `expiry_date` - Optional; resolvable when `underlying` already embeds an
    ///   expiry (e.g. `NIFTY28OCT25FUT`).
    /// * `strategy` - Deprecated in Python, kept for parity. Optional.
    /// * `strike_int` - Deprecated in Python, kept for parity. Optional.
    /// * `extra` - Extra broker-specific fields forwarded verbatim, mirroring Python's `**kwargs`.
    #[allow(clippy::too_many_arguments)]
    pub async fn option_symbol(
        &self,
        underlying: &str,
        exchange: &str,
        offset: &str,
        option_type: &str,
        expiry_date: Option<&str>,
        strategy: Option<&str>,
        strike_int: Option<i32>,
        extra: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<OptionSymbolResponse, OpenAlgoError> {
        let request = OptionSymbolRequest {
            apikey: self.client.api_key.clone(),
            underlying: underlying.to_string(),
            exchange: exchange.to_string(),
            expiry_date: expiry_date.map(|s| s.to_string()),
            offset: offset.to_string(),
            option_type: option_type.to_string(),
            strategy: strategy.map(|s| s.to_string()),
            strike_int: strike_int.map(|v| v.to_string()),
            extra: extra.unwrap_or_default(),
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

    /// Get option Greeks (Delta, Gamma, Theta, Vega, Rho) and implied volatility
    ///
    /// Only `symbol`/`exchange` are required — everything else is optional and
    /// auto-detected/defaulted server-side, mirroring Python's `optiongreeks()`.
    ///
    /// * `interest_rate` - Risk-free rate (annualized %). Defaults to 0 server-side.
    /// * `forward_price` - Custom forward/synthetic futures price; skips the
    ///   underlying price fetch when provided.
    /// * `underlying_symbol` / `underlying_exchange` - Override auto-detection.
    /// * `expiry_time` - Custom expiry time in HH:MM (required for MCX contracts
    ///   with non-standard expiry times).
    /// * `extra` - Extra broker-specific fields forwarded verbatim, mirroring Python's `**kwargs`.
    #[allow(clippy::too_many_arguments)]
    pub async fn option_greeks(
        &self,
        symbol: &str,
        exchange: &str,
        interest_rate: Option<f64>,
        forward_price: Option<f64>,
        underlying_symbol: Option<&str>,
        underlying_exchange: Option<&str>,
        expiry_time: Option<&str>,
        extra: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<OptionGreeksResponse, OpenAlgoError> {
        let request = OptionGreeksRequest {
            apikey: self.client.api_key.clone(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            interest_rate,
            forward_price,
            underlying_symbol: underlying_symbol.map(|s| s.to_string()),
            underlying_exchange: underlying_exchange.map(|s| s.to_string()),
            expiry_time: expiry_time.map(|s| s.to_string()),
            extra: extra.unwrap_or_default(),
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

    /// Download instrument master data, with optional exchange filtering.
    ///
    /// Mirrors Python's `instruments(exchange=None)`: when `exchange` is `None`,
    /// every supported exchange is fetched and the results combined into one
    /// response, matching the "download ALL exchanges" behaviour of the
    /// reference SDK. Uses GET (query params), matching the server's actual
    /// `instruments` endpoint contract.
    pub async fn instruments(
        &self,
        exchange: Option<&str>,
    ) -> Result<InstrumentsResponse, OpenAlgoError> {
        match exchange {
            Some(exch) => self.instruments_for_exchange(exch).await,
            None => {
                const ALL_EXCHANGES: [&str; 9] = [
                    "NSE", "BSE", "NFO", "BFO", "MCX", "CDS", "BCD", "NSE_INDEX", "BSE_INDEX",
                ];

                let mut combined: Vec<SymbolData> = Vec::new();
                let mut any_success = false;

                for exch in ALL_EXCHANGES {
                    if let Ok(resp) = self.instruments_for_exchange(exch).await {
                        if resp.status == "success" {
                            if let Some(data) = resp.data {
                                any_success = true;
                                combined.extend(data);
                            }
                        }
                    }
                }

                if any_success {
                    Ok(InstrumentsResponse {
                        status: "success".to_string(),
                        data: Some(combined),
                        message: None,
                    })
                } else {
                    Ok(InstrumentsResponse {
                        status: "error".to_string(),
                        data: None,
                        message: Some("Failed to fetch instruments from any exchange".to_string()),
                    })
                }
            }
        }
    }

    /// Fetch instruments for a single exchange (internal helper for [`DataAPI::instruments`]).
    async fn instruments_for_exchange(
        &self,
        exchange: &str,
    ) -> Result<InstrumentsResponse, OpenAlgoError> {
        self.client
            .get(
                "instruments",
                &[("apikey", self.client.api_key.as_str()), ("exchange", exchange)],
            )
            .await
    }
}
