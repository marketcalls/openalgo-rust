//! OpenAlgo Rust SDK
//!
//! A Rust wrapper for the OpenAlgo API with WebSocket support.
//!
//! # Example
//!
//! ```rust,no_run
//! use openalgo::OpenAlgo;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = OpenAlgo::new("your_api_key");
//!
//!     // Get quotes
//!     let quotes = client.quotes("RELIANCE", "NSE").await?;
//!     println!("{:?}", quotes);
//!
//!     // Place a simple market order
//!     let order = client.place_order("Test", "RELIANCE", "BUY", "NSE", "MARKET", "MIS", "1").await?;
//!     println!("{:?}", order);
//!
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod client;
pub mod orders;
pub mod data;
pub mod account;
pub mod utilities;
pub mod analyzer;
pub mod websocket;

pub use types::*;
pub use client::OpenAlgoClient;
pub use orders::OrderAPI;
pub use data::DataAPI;
pub use account::AccountAPI;
pub use utilities::UtilitiesAPI;
pub use analyzer::AnalyzerAPI;
pub use websocket::OpenAlgoWebSocket;

use std::sync::Arc;

/// OpenAlgo API client combining all API modules
pub struct OpenAlgo {
    client: Arc<OpenAlgoClient>,
    pub orders: OrderAPI,
    pub data: DataAPI,
    pub account: AccountAPI,
    pub utilities: UtilitiesAPI,
    pub analyzer: AnalyzerAPI,
}

impl OpenAlgo {
    /// Create a new OpenAlgo client with default settings
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your OpenAlgo API key
    pub fn new(api_key: &str) -> Self {
        Self::with_config(api_key, "http://127.0.0.1:5000", "v1", "ws://127.0.0.1:8765")
    }

    /// Create a new OpenAlgo client with custom configuration
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your OpenAlgo API key
    /// * `host` - API host URL
    /// * `version` - API version
    /// * `ws_url` - WebSocket URL
    pub fn with_config(api_key: &str, host: &str, version: &str, ws_url: &str) -> Self {
        let client = Arc::new(OpenAlgoClient::new(api_key, host, version, ws_url));

        Self {
            orders: OrderAPI::new(Arc::clone(&client)),
            data: DataAPI::new(Arc::clone(&client)),
            account: AccountAPI::new(Arc::clone(&client)),
            utilities: UtilitiesAPI::new(Arc::clone(&client)),
            analyzer: AnalyzerAPI::new(Arc::clone(&client)),
            client,
        }
    }

    /// Create a WebSocket client for real-time data
    pub fn websocket(&self) -> OpenAlgoWebSocket {
        OpenAlgoWebSocket::new(&self.client.api_key, &self.client.ws_url)
    }

    // =========================================================================
    // Order API - Simple Interface
    // =========================================================================

    /// Place a market order (simplest form)
    ///
    /// # Example
    /// ```rust,no_run
    /// let order = client.place_order("Strategy1", "RELIANCE", "BUY", "NSE", "MARKET", "MIS", "1").await?;
    /// ```
    pub async fn place_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        pricetype: &str,
        product: &str,
        quantity: &str,
    ) -> Result<OrderResponse, crate::client::OpenAlgoError> {
        self.orders.place_order(strategy, symbol, action, exchange, pricetype, product, quantity).await
    }

    /// Place a limit order with price
    ///
    /// # Example
    /// ```rust,no_run
    /// let order = client.place_limit_order("Strategy1", "RELIANCE", "BUY", "NSE", "MIS", "1", "2500.00").await?;
    /// ```
    pub async fn place_limit_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        product: &str,
        quantity: &str,
        price: &str,
    ) -> Result<OrderResponse, crate::client::OpenAlgoError> {
        self.orders.place_limit_order(strategy, symbol, action, exchange, product, quantity, price).await
    }

    /// Place a stop-loss order
    ///
    /// # Example
    /// ```rust,no_run
    /// let order = client.place_sl_order("Strategy1", "RELIANCE", "BUY", "NSE", "MIS", "1", "2500.00", "2490.00").await?;
    /// ```
    pub async fn place_sl_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        product: &str,
        quantity: &str,
        price: &str,
        trigger_price: &str,
    ) -> Result<OrderResponse, crate::client::OpenAlgoError> {
        self.orders.place_sl_order(strategy, symbol, action, exchange, product, quantity, price, trigger_price).await
    }

    /// Place a smart order with position sizing
    ///
    /// # Example
    /// ```rust,no_run
    /// let order = client.place_smart_order("Strategy1", "RELIANCE", "BUY", "NSE", "MARKET", "MIS", "1", "5").await?;
    /// ```
    pub async fn place_smart_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        pricetype: &str,
        product: &str,
        quantity: &str,
        position_size: &str,
    ) -> Result<OrderResponse, crate::client::OpenAlgoError> {
        self.orders.place_smart_order(strategy, symbol, action, exchange, pricetype, product, quantity, position_size).await
    }

    /// Place an options order
    ///
    /// # Example
    /// ```rust,no_run
    /// let order = client.options_order("Strategy1", "NIFTY", "NFO", "241226", "0", "CE", "BUY", "50", "MARKET", "MIS", "50").await?;
    /// ```
    pub async fn options_order(
        &self,
        strategy: &str,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
        offset: &str,
        option_type: &str,
        action: &str,
        quantity: &str,
        pricetype: &str,
        product: &str,
        splitsize: &str,
    ) -> Result<OptionsOrderResponse, crate::client::OpenAlgoError> {
        self.orders.options_order(strategy, underlying, exchange, expiry_date, offset, option_type, action, quantity, pricetype, product, splitsize).await
    }

    /// Place a multi-leg options order
    ///
    /// # Example
    /// ```rust,no_run
    /// use openalgo::OptionsLeg;
    /// let legs = vec![
    ///     OptionsLeg::new("0", "CE", "BUY", "50"),
    ///     OptionsLeg::new("2", "CE", "SELL", "50"),
    /// ];
    /// let order = client.options_multi_order("Strategy1", "NIFTY", "NFO", "241226", legs).await?;
    /// ```
    pub async fn options_multi_order(
        &self,
        strategy: &str,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
        legs: Vec<OptionsLeg>,
    ) -> Result<OptionsMultiOrderResponse, crate::client::OpenAlgoError> {
        self.orders.options_multi_order(strategy, underlying, exchange, expiry_date, legs).await
    }

    /// Place basket orders
    ///
    /// # Example
    /// ```rust,no_run
    /// use openalgo::BasketOrderItem;
    /// let orders = vec![
    ///     BasketOrderItem::new("RELIANCE", "NSE", "BUY", "1", "MARKET", "MIS"),
    ///     BasketOrderItem::new("TCS", "NSE", "BUY", "1", "MARKET", "MIS"),
    /// ];
    /// let result = client.basket_order("Strategy1", orders).await?;
    /// ```
    pub async fn basket_order(
        &self,
        strategy: &str,
        orders: Vec<BasketOrderItem>,
    ) -> Result<BasketOrderResponse, crate::client::OpenAlgoError> {
        self.orders.basket_order(strategy, orders).await
    }

    /// Place split orders
    ///
    /// # Example
    /// ```rust,no_run
    /// let result = client.split_order("Strategy1", "RELIANCE", "BUY", "NSE", 100, 25, "MARKET", "MIS").await?;
    /// ```
    pub async fn split_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        quantity: i32,
        splitsize: i32,
        pricetype: &str,
        product: &str,
    ) -> Result<SplitOrderResponse, crate::client::OpenAlgoError> {
        self.orders.split_order(strategy, symbol, action, exchange, quantity, splitsize, pricetype, product).await
    }

    /// Modify an order
    ///
    /// # Example
    /// ```rust,no_run
    /// let result = client.modify_order("1234567890", "Strategy1", "RELIANCE", "BUY", "NSE", "LIMIT", "MIS", "1", "2550.00").await?;
    /// ```
    pub async fn modify_order(
        &self,
        orderid: &str,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        pricetype: &str,
        product: &str,
        quantity: &str,
        price: &str,
    ) -> Result<OrderResponse, crate::client::OpenAlgoError> {
        self.orders.modify_order(orderid, strategy, symbol, action, exchange, pricetype, product, quantity, price).await
    }

    /// Cancel an order
    ///
    /// # Example
    /// ```rust,no_run
    /// let result = client.cancel_order("1234567890", "Strategy1").await?;
    /// ```
    pub async fn cancel_order(
        &self,
        orderid: &str,
        strategy: &str,
    ) -> Result<OrderResponse, crate::client::OpenAlgoError> {
        self.orders.cancel_order(orderid, strategy).await
    }

    /// Cancel all orders
    ///
    /// # Example
    /// ```rust,no_run
    /// let result = client.cancel_all_order("Strategy1").await?;
    /// ```
    pub async fn cancel_all_order(
        &self,
        strategy: &str,
    ) -> Result<CancelAllOrderResponse, crate::client::OpenAlgoError> {
        self.orders.cancel_all_order(strategy).await
    }

    /// Close all positions
    ///
    /// # Example
    /// ```rust,no_run
    /// let result = client.close_position("Strategy1").await?;
    /// ```
    pub async fn close_position(
        &self,
        strategy: &str,
    ) -> Result<StatusResponse, crate::client::OpenAlgoError> {
        self.orders.close_position(strategy).await
    }

    /// Get order status
    ///
    /// # Example
    /// ```rust,no_run
    /// let status = client.order_status("1234567890", "Strategy1").await?;
    /// ```
    pub async fn order_status(
        &self,
        orderid: &str,
        strategy: &str,
    ) -> Result<OrderStatusResponse, crate::client::OpenAlgoError> {
        self.orders.order_status(orderid, strategy).await
    }

    /// Get open position
    ///
    /// # Example
    /// ```rust,no_run
    /// let position = client.open_position("Strategy1", "RELIANCE", "NSE", "MIS").await?;
    /// ```
    pub async fn open_position(
        &self,
        strategy: &str,
        symbol: &str,
        exchange: &str,
        product: &str,
    ) -> Result<OpenPositionResponse, crate::client::OpenAlgoError> {
        self.orders.open_position(strategy, symbol, exchange, product).await
    }

    // =========================================================================
    // Data API
    // =========================================================================

    /// Get quotes for a symbol
    ///
    /// # Example
    /// ```rust,no_run
    /// let quotes = client.quotes("RELIANCE", "NSE").await?;
    /// ```
    pub async fn quotes(
        &self,
        symbol: &str,
        exchange: &str,
    ) -> Result<QuotesResponse, crate::client::OpenAlgoError> {
        self.data.quotes(symbol, exchange).await
    }

    /// Get quotes for multiple symbols
    ///
    /// # Example
    /// ```rust,no_run
    /// let quotes = client.multi_quotes(&[("RELIANCE", "NSE"), ("TCS", "NSE")]).await?;
    /// ```
    pub async fn multi_quotes(
        &self,
        symbols: &[(&str, &str)],
    ) -> Result<MultiQuotesResponse, crate::client::OpenAlgoError> {
        self.data.multi_quotes(symbols).await
    }

    /// Get market depth
    ///
    /// # Example
    /// ```rust,no_run
    /// let depth = client.depth("RELIANCE", "NSE").await?;
    /// ```
    pub async fn depth(
        &self,
        symbol: &str,
        exchange: &str,
    ) -> Result<DepthResponse, crate::client::OpenAlgoError> {
        self.data.depth(symbol, exchange).await
    }

    /// Get historical data
    ///
    /// # Example
    /// ```rust,no_run
    /// let history = client.history("RELIANCE", "NSE", "5m").await?;
    /// ```
    pub async fn history(
        &self,
        symbol: &str,
        exchange: &str,
        interval: &str,
    ) -> Result<serde_json::Value, crate::client::OpenAlgoError> {
        self.data.history(symbol, exchange, interval).await
    }

    /// Get historical data with date range
    ///
    /// # Example
    /// ```rust,no_run
    /// let history = client.history_range("RELIANCE", "NSE", "5m", "2024-01-01", "2024-01-31").await?;
    /// ```
    pub async fn history_range(
        &self,
        symbol: &str,
        exchange: &str,
        interval: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<serde_json::Value, crate::client::OpenAlgoError> {
        self.data.history_range(symbol, exchange, interval, start_date, end_date).await
    }

    /// Get available intervals
    ///
    /// # Example
    /// ```rust,no_run
    /// let intervals = client.intervals().await?;
    /// ```
    pub async fn intervals(&self) -> Result<IntervalsResponse, crate::client::OpenAlgoError> {
        self.data.intervals().await
    }

    /// Get option chain
    ///
    /// # Example
    /// ```rust,no_run
    /// let chain = client.option_chain("NIFTY", "NFO", "241226").await?;
    /// ```
    pub async fn option_chain(
        &self,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
    ) -> Result<OptionChainResponse, crate::client::OpenAlgoError> {
        self.data.option_chain(underlying, exchange, expiry_date).await
    }

    /// Get symbol info
    ///
    /// # Example
    /// ```rust,no_run
    /// let info = client.symbol("RELIANCE", "NSE").await?;
    /// ```
    pub async fn symbol(
        &self,
        symbol: &str,
        exchange: &str,
    ) -> Result<SymbolResponse, crate::client::OpenAlgoError> {
        self.data.symbol(symbol, exchange).await
    }

    /// Search symbols
    ///
    /// # Example
    /// ```rust,no_run
    /// let results = client.search("RELI", "NSE").await?;
    /// ```
    pub async fn search(
        &self,
        query: &str,
        exchange: &str,
    ) -> Result<SearchResponse, crate::client::OpenAlgoError> {
        self.data.search(query, exchange).await
    }

    /// Get option symbol
    ///
    /// # Example
    /// ```rust,no_run
    /// let symbol = client.option_symbol("NIFTY", "NFO", "241226", "0", "CE").await?;
    /// ```
    pub async fn option_symbol(
        &self,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
        offset: &str,
        option_type: &str,
    ) -> Result<OptionSymbolResponse, crate::client::OpenAlgoError> {
        self.data.option_symbol(underlying, exchange, expiry_date, offset, option_type).await
    }

    /// Get synthetic future price
    ///
    /// # Example
    /// ```rust,no_run
    /// let future = client.synthetic_future("NIFTY", "NFO", "241226").await?;
    /// ```
    pub async fn synthetic_future(
        &self,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
    ) -> Result<SyntheticFutureResponse, crate::client::OpenAlgoError> {
        self.data.synthetic_future(underlying, exchange, expiry_date).await
    }

    /// Get option Greeks
    ///
    /// # Example
    /// ```rust,no_run
    /// let greeks = client.option_greeks("NIFTY24DEC24000CE", "NFO", 6.5, "NIFTY", "NSE").await?;
    /// ```
    pub async fn option_greeks(
        &self,
        symbol: &str,
        exchange: &str,
        interest_rate: f64,
        underlying_symbol: &str,
        underlying_exchange: &str,
    ) -> Result<OptionGreeksResponse, crate::client::OpenAlgoError> {
        self.data.option_greeks(symbol, exchange, interest_rate, underlying_symbol, underlying_exchange).await
    }

    /// Get expiry dates
    ///
    /// # Example
    /// ```rust,no_run
    /// let expiries = client.expiry("NIFTY", "NFO", "OPT").await?;
    /// ```
    pub async fn expiry(
        &self,
        symbol: &str,
        exchange: &str,
        instrumenttype: &str,
    ) -> Result<ExpiryResponse, crate::client::OpenAlgoError> {
        self.data.expiry(symbol, exchange, instrumenttype).await
    }

    /// Get instruments
    ///
    /// # Example
    /// ```rust,no_run
    /// let instruments = client.instruments("NSE").await?;
    /// ```
    pub async fn instruments(
        &self,
        exchange: &str,
    ) -> Result<InstrumentsResponse, crate::client::OpenAlgoError> {
        self.data.instruments(exchange).await
    }

    // =========================================================================
    // Account API
    // =========================================================================

    /// Get account funds
    ///
    /// # Example
    /// ```rust,no_run
    /// let funds = client.funds().await?;
    /// ```
    pub async fn funds(&self) -> Result<FundsResponse, crate::client::OpenAlgoError> {
        self.account.funds().await
    }

    /// Get orderbook
    ///
    /// # Example
    /// ```rust,no_run
    /// let orderbook = client.orderbook().await?;
    /// ```
    pub async fn orderbook(&self) -> Result<OrderbookResponse, crate::client::OpenAlgoError> {
        self.account.orderbook().await
    }

    /// Get tradebook
    ///
    /// # Example
    /// ```rust,no_run
    /// let tradebook = client.tradebook().await?;
    /// ```
    pub async fn tradebook(&self) -> Result<TradebookResponse, crate::client::OpenAlgoError> {
        self.account.tradebook().await
    }

    /// Get positionbook
    ///
    /// # Example
    /// ```rust,no_run
    /// let positions = client.positionbook().await?;
    /// ```
    pub async fn positionbook(&self) -> Result<PositionbookResponse, crate::client::OpenAlgoError> {
        self.account.positionbook().await
    }

    /// Get holdings
    ///
    /// # Example
    /// ```rust,no_run
    /// let holdings = client.holdings().await?;
    /// ```
    pub async fn holdings(&self) -> Result<HoldingsResponse, crate::client::OpenAlgoError> {
        self.account.holdings().await
    }

    /// Get margin requirement
    ///
    /// # Example
    /// ```rust,no_run
    /// use openalgo::MarginPosition;
    /// let positions = vec![
    ///     MarginPosition::new("NIFTY24DEC24000CE", "NFO", "BUY", "MIS", "MARKET", "50"),
    /// ];
    /// let margin = client.margin(positions).await?;
    /// ```
    pub async fn margin(
        &self,
        positions: Vec<MarginPosition>,
    ) -> Result<MarginResponse, crate::client::OpenAlgoError> {
        self.account.margin(positions).await
    }

    // =========================================================================
    // Utilities API
    // =========================================================================

    /// Get market holidays
    ///
    /// # Example
    /// ```rust,no_run
    /// let holidays = client.holidays(2024).await?;
    /// ```
    pub async fn holidays(
        &self,
        year: i32,
    ) -> Result<HolidaysResponse, crate::client::OpenAlgoError> {
        self.utilities.holidays(year).await
    }

    /// Get exchange timings
    ///
    /// # Example
    /// ```rust,no_run
    /// let timings = client.timings("2024-12-25").await?;
    /// ```
    pub async fn timings(
        &self,
        date: &str,
    ) -> Result<TimingsResponse, crate::client::OpenAlgoError> {
        self.utilities.timings(date).await
    }

    /// Send Telegram message with default priority (5)
    ///
    /// # Example
    /// ```rust,no_run
    /// let result = client.telegram("username", "Hello from OpenAlgo!").await?;
    /// ```
    pub async fn telegram(
        &self,
        username: &str,
        message: &str,
    ) -> Result<TelegramResponse, crate::client::OpenAlgoError> {
        self.utilities.telegram(username, message).await
    }

    /// Send Telegram message with custom priority
    ///
    /// # Example
    /// ```rust,no_run
    /// let result = client.telegram_priority("username", "Urgent alert!", 10).await?;
    /// ```
    pub async fn telegram_priority(
        &self,
        username: &str,
        message: &str,
        priority: i32,
    ) -> Result<TelegramResponse, crate::client::OpenAlgoError> {
        self.utilities.telegram_priority(username, message, priority).await
    }

    // =========================================================================
    // Analyzer API
    // =========================================================================

    /// Get analyzer status
    ///
    /// # Example
    /// ```rust,no_run
    /// let status = client.analyzer_status().await?;
    /// ```
    pub async fn analyzer_status(&self) -> Result<AnalyzerStatusResponse, crate::client::OpenAlgoError> {
        self.analyzer.status().await
    }

    /// Toggle analyzer mode
    ///
    /// # Example
    /// ```rust,no_run
    /// let result = client.analyzer_toggle(true).await?;
    /// ```
    pub async fn analyzer_toggle(
        &self,
        mode: bool,
    ) -> Result<AnalyzerToggleResponse, crate::client::OpenAlgoError> {
        self.analyzer.toggle(mode).await
    }
}

/// Library version
pub const VERSION: &str = "1.0.0";
