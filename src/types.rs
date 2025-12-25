//! Type definitions for OpenAlgo API requests and responses.

use serde::{Deserialize, Serialize};

// ============================================================================
// Common Types
// ============================================================================

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    #[serde(flatten)]
    pub data: Option<T>,
    pub message: Option<String>,
}

/// Simple status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub message: Option<String>,
}

/// Order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub status: String,
    pub orderid: Option<String>,
    pub message: Option<String>,
}

// ============================================================================
// Order Types
// ============================================================================

/// Place order request
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderRequest {
    pub apikey: String,
    pub strategy: String,
    pub symbol: String,
    pub action: String,
    pub exchange: String,
    pub pricetype: String,
    pub product: String,
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosed_quantity: Option<String>,
}

/// Smart order request
#[derive(Debug, Clone, Serialize)]
pub struct PlaceSmartOrderRequest {
    pub apikey: String,
    pub strategy: String,
    pub symbol: String,
    pub action: String,
    pub exchange: String,
    pub pricetype: String,
    pub product: String,
    pub quantity: String,
    pub position_size: String,
}

/// Options order request
#[derive(Debug, Clone, Serialize)]
pub struct OptionsOrderRequest {
    pub apikey: String,
    pub strategy: String,
    pub underlying: String,
    pub exchange: String,
    pub expiry_date: String,
    pub offset: String,
    pub option_type: String,
    pub action: String,
    pub quantity: String,
    pub pricetype: String,
    pub product: String,
    pub splitsize: String,
}

/// Options order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsOrderResponse {
    pub status: String,
    pub orderid: Option<String>,
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub offset: Option<String>,
    pub option_type: Option<String>,
    pub underlying: Option<String>,
    pub underlying_ltp: Option<f64>,
    pub mode: Option<String>,
    pub message: Option<String>,
}

/// Options multi-order leg
#[derive(Debug, Clone, Serialize)]
pub struct OptionsLeg {
    pub offset: String,
    pub option_type: String,
    pub action: String,
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
}

impl OptionsLeg {
    /// Create a new options leg
    ///
    /// # Example
    /// ```rust
    /// use openalgo::OptionsLeg;
    /// let leg = OptionsLeg::new("0", "CE", "BUY", "50");
    /// ```
    pub fn new(offset: &str, option_type: &str, action: &str, quantity: &str) -> Self {
        Self {
            offset: offset.to_string(),
            option_type: option_type.to_string(),
            action: action.to_string(),
            quantity: quantity.to_string(),
            expiry_date: None,
        }
    }

    /// Create a new options leg with custom expiry
    ///
    /// # Example
    /// ```rust
    /// use openalgo::OptionsLeg;
    /// let leg = OptionsLeg::with_expiry("0", "CE", "BUY", "50", "241226");
    /// ```
    pub fn with_expiry(offset: &str, option_type: &str, action: &str, quantity: &str, expiry_date: &str) -> Self {
        Self {
            offset: offset.to_string(),
            option_type: option_type.to_string(),
            action: action.to_string(),
            quantity: quantity.to_string(),
            expiry_date: Some(expiry_date.to_string()),
        }
    }
}

/// Options multi-order request
#[derive(Debug, Clone, Serialize)]
pub struct OptionsMultiOrderRequest {
    pub apikey: String,
    pub strategy: String,
    pub underlying: String,
    pub exchange: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    pub legs: Vec<OptionsLeg>,
}

/// Options multi-order result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsMultiOrderResult {
    pub leg: i32,
    pub status: String,
    pub orderid: Option<String>,
    pub symbol: Option<String>,
    pub offset: Option<String>,
    pub option_type: Option<String>,
    pub action: Option<String>,
    pub mode: Option<String>,
}

/// Options multi-order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsMultiOrderResponse {
    pub status: String,
    pub underlying: Option<String>,
    pub underlying_ltp: Option<f64>,
    pub results: Option<Vec<OptionsMultiOrderResult>>,
    pub message: Option<String>,
}

/// Basket order item
#[derive(Debug, Clone, Serialize)]
pub struct BasketOrderItem {
    pub symbol: String,
    pub exchange: String,
    pub action: String,
    pub quantity: i32,
    pub pricetype: String,
    pub product: String,
}

impl BasketOrderItem {
    /// Create a new basket order item
    ///
    /// # Example
    /// ```rust
    /// use openalgo::BasketOrderItem;
    /// let item = BasketOrderItem::new("RELIANCE", "NSE", "BUY", 1, "MARKET", "MIS");
    /// ```
    pub fn new(symbol: &str, exchange: &str, action: &str, quantity: i32, pricetype: &str, product: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            action: action.to_string(),
            quantity,
            pricetype: pricetype.to_string(),
            product: product.to_string(),
        }
    }
}

/// Basket order request
#[derive(Debug, Clone, Serialize)]
pub struct BasketOrderRequest {
    pub apikey: String,
    pub strategy: String,
    pub orders: Vec<BasketOrderItem>,
}

/// Basket order result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasketOrderResult {
    pub symbol: String,
    pub status: String,
    pub orderid: Option<String>,
}

/// Basket order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasketOrderResponse {
    pub status: String,
    pub results: Option<Vec<BasketOrderResult>>,
    pub message: Option<String>,
}

/// Split order request
#[derive(Debug, Clone, Serialize)]
pub struct SplitOrderRequest {
    pub apikey: String,
    pub strategy: String,
    pub symbol: String,
    pub action: String,
    pub exchange: String,
    pub quantity: i32,
    pub splitsize: i32,
    pub pricetype: String,
    pub product: String,
}

/// Split order result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitOrderResult {
    pub order_num: Option<i32>,
    pub orderid: Option<String>,
    pub quantity: Option<i32>,
    pub status: String,
}

/// Split order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitOrderResponse {
    pub status: String,
    pub split_size: Option<i32>,
    pub total_quantity: Option<i32>,
    pub results: Option<Vec<SplitOrderResult>>,
    pub message: Option<String>,
}

/// Modify order request
#[derive(Debug, Clone, Serialize)]
pub struct ModifyOrderRequest {
    pub apikey: String,
    pub orderid: String,
    pub strategy: String,
    pub symbol: String,
    pub action: String,
    pub exchange: String,
    pub pricetype: String,
    pub product: String,
    pub quantity: String,
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosed_quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
}

/// Cancel order request
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    pub apikey: String,
    pub orderid: String,
    pub strategy: String,
}

/// Cancel all orders request
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrderRequest {
    pub apikey: String,
    pub strategy: String,
}

/// Cancel all orders response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAllOrderResponse {
    pub status: String,
    pub message: Option<String>,
    pub canceled_orders: Option<Vec<String>>,
    pub failed_cancellations: Option<Vec<String>>,
}

/// Close position request
#[derive(Debug, Clone, Serialize)]
pub struct ClosePositionRequest {
    pub apikey: String,
    pub strategy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbolgroup: Option<String>,
}

/// Order status request
#[derive(Debug, Clone, Serialize)]
pub struct OrderStatusRequest {
    pub apikey: String,
    pub orderid: String,
    pub strategy: String,
}

/// Order status data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusData {
    pub action: Option<String>,
    pub average_price: Option<f64>,
    pub exchange: Option<String>,
    pub order_status: Option<String>,
    pub orderid: Option<String>,
    pub price: Option<f64>,
    pub pricetype: Option<String>,
    pub product: Option<String>,
    pub quantity: Option<String>,
    pub symbol: Option<String>,
    pub timestamp: Option<String>,
    pub trigger_price: Option<f64>,
}

/// Order status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusResponse {
    pub status: String,
    pub data: Option<OrderStatusData>,
    pub message: Option<String>,
}

/// Open position request
#[derive(Debug, Clone, Serialize)]
pub struct OpenPositionRequest {
    pub apikey: String,
    pub strategy: String,
    pub symbol: String,
    pub exchange: String,
    pub product: String,
}

/// Open position response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenPositionResponse {
    pub status: String,
    pub quantity: Option<String>,
    pub message: Option<String>,
}

// ============================================================================
// Data Types
// ============================================================================

/// Quotes request
#[derive(Debug, Clone, Serialize)]
pub struct QuotesRequest {
    pub apikey: String,
    pub symbol: String,
    pub exchange: String,
}

/// Quotes data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesData {
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub ltp: Option<f64>,
    pub ask: Option<f64>,
    pub bid: Option<f64>,
    pub prev_close: Option<f64>,
    pub volume: Option<i64>,
    pub oi: Option<i64>,
}

/// Quotes response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesResponse {
    pub status: String,
    pub data: Option<QuotesData>,
    pub message: Option<String>,
}

/// Multi-quotes symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiQuotesSymbol {
    pub symbol: String,
    pub exchange: String,
}

impl MultiQuotesSymbol {
    /// Create a new multi-quotes symbol
    pub fn new(symbol: &str, exchange: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
        }
    }
}

/// Multi-quotes request
#[derive(Debug, Clone, Serialize)]
pub struct MultiQuotesRequest {
    pub apikey: String,
    pub symbols: Vec<MultiQuotesSymbol>,
}

/// Multi-quotes result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiQuotesResult {
    pub symbol: String,
    pub exchange: String,
    pub data: Option<QuotesData>,
}

/// Multi-quotes response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiQuotesResponse {
    pub status: String,
    pub results: Option<Vec<MultiQuotesResult>>,
    pub message: Option<String>,
}

/// Depth request
#[derive(Debug, Clone, Serialize)]
pub struct DepthRequest {
    pub apikey: String,
    pub symbol: String,
    pub exchange: String,
}

/// Depth level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthLevel {
    pub price: f64,
    pub quantity: i64,
}

/// Depth data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthData {
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub ltp: Option<f64>,
    pub ltq: Option<i64>,
    pub prev_close: Option<f64>,
    pub volume: Option<i64>,
    pub oi: Option<i64>,
    pub totalbuyqty: Option<i64>,
    pub totalsellqty: Option<i64>,
    pub asks: Option<Vec<DepthLevel>>,
    pub bids: Option<Vec<DepthLevel>>,
}

/// Depth response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthResponse {
    pub status: String,
    pub data: Option<DepthData>,
    pub message: Option<String>,
}

/// History request
#[derive(Debug, Clone, Serialize)]
pub struct HistoryRequest {
    pub apikey: String,
    pub symbol: String,
    pub exchange: String,
    pub interval: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// History candle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryCandle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

/// Intervals request
#[derive(Debug, Clone, Serialize)]
pub struct IntervalsRequest {
    pub apikey: String,
}

/// Intervals data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntervalsData {
    pub months: Vec<String>,
    pub weeks: Vec<String>,
    pub days: Vec<String>,
    pub hours: Vec<String>,
    pub minutes: Vec<String>,
    pub seconds: Vec<String>,
}

/// Intervals response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntervalsResponse {
    pub status: String,
    pub data: Option<IntervalsData>,
    pub message: Option<String>,
}

/// Option chain request
#[derive(Debug, Clone, Serialize)]
pub struct OptionChainRequest {
    pub apikey: String,
    pub underlying: String,
    pub exchange: String,
    pub expiry_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_count: Option<i32>,
}

/// Option chain strike data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionStrikeData {
    pub symbol: Option<String>,
    pub label: Option<String>,
    pub ltp: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub prev_close: Option<f64>,
    pub volume: Option<i64>,
    pub oi: Option<i64>,
    pub lotsize: Option<i32>,
    pub tick_size: Option<f64>,
}

/// Option chain strike
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChainStrike {
    pub strike: f64,
    pub ce: Option<OptionStrikeData>,
    pub pe: Option<OptionStrikeData>,
}

/// Option chain response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChainResponse {
    pub status: String,
    pub underlying: Option<String>,
    pub underlying_ltp: Option<f64>,
    pub expiry_date: Option<String>,
    pub atm_strike: Option<f64>,
    pub chain: Option<Vec<OptionChainStrike>>,
    pub message: Option<String>,
}

/// Symbol request
#[derive(Debug, Clone, Serialize)]
pub struct SymbolRequest {
    pub apikey: String,
    pub symbol: String,
    pub exchange: String,
}

/// Symbol data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolData {
    pub brexchange: Option<String>,
    pub brsymbol: Option<String>,
    pub exchange: Option<String>,
    pub expiry: Option<String>,
    pub freeze_qty: Option<i32>,
    pub id: Option<i64>,
    pub instrumenttype: Option<String>,
    pub lotsize: Option<i32>,
    pub name: Option<String>,
    pub strike: Option<f64>,
    pub symbol: Option<String>,
    pub tick_size: Option<f64>,
    pub token: Option<String>,
}

/// Symbol response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolResponse {
    pub status: String,
    pub data: Option<SymbolData>,
    pub message: Option<String>,
}

/// Search request
#[derive(Debug, Clone, Serialize)]
pub struct SearchRequest {
    pub apikey: String,
    pub query: String,
    pub exchange: String,
}

/// Search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub status: String,
    pub data: Option<Vec<SymbolData>>,
    pub message: Option<String>,
}

/// Option symbol request
#[derive(Debug, Clone, Serialize)]
pub struct OptionSymbolRequest {
    pub apikey: String,
    pub underlying: String,
    pub exchange: String,
    pub expiry_date: String,
    pub offset: String,
    pub option_type: String,
}

/// Option symbol response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionSymbolResponse {
    pub status: String,
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub lotsize: Option<i32>,
    pub tick_size: Option<f64>,
    pub freeze_qty: Option<i32>,
    pub underlying_ltp: Option<f64>,
    pub message: Option<String>,
}

/// Synthetic future request
#[derive(Debug, Clone, Serialize)]
pub struct SyntheticFutureRequest {
    pub apikey: String,
    pub underlying: String,
    pub exchange: String,
    pub expiry_date: String,
}

/// Synthetic future response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntheticFutureResponse {
    pub status: String,
    pub atm_strike: Option<f64>,
    pub expiry: Option<String>,
    pub synthetic_future_price: Option<f64>,
    pub underlying: Option<String>,
    pub underlying_ltp: Option<f64>,
    pub message: Option<String>,
}

/// Option Greeks request
#[derive(Debug, Clone, Serialize)]
pub struct OptionGreeksRequest {
    pub apikey: String,
    pub symbol: String,
    pub exchange: String,
    pub interest_rate: f64,
    pub underlying_symbol: String,
    pub underlying_exchange: String,
}

/// Greeks data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreeksData {
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
    pub rho: Option<f64>,
}

/// Option Greeks response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionGreeksResponse {
    pub status: String,
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub underlying: Option<String>,
    pub option_type: Option<String>,
    pub strike: Option<f64>,
    pub expiry_date: Option<String>,
    pub days_to_expiry: Option<f64>,
    pub spot_price: Option<f64>,
    pub option_price: Option<f64>,
    pub implied_volatility: Option<f64>,
    pub interest_rate: Option<f64>,
    pub greeks: Option<GreeksData>,
    pub message: Option<String>,
}

/// Expiry request
#[derive(Debug, Clone, Serialize)]
pub struct ExpiryRequest {
    pub apikey: String,
    pub symbol: String,
    pub exchange: String,
    pub instrumenttype: String,
}

/// Expiry response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiryResponse {
    pub status: String,
    pub data: Option<Vec<String>>,
    pub message: Option<String>,
}

/// Instruments request
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentsRequest {
    pub apikey: String,
    pub exchange: String,
}

/// Instruments response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentsResponse {
    pub status: String,
    pub data: Option<Vec<SymbolData>>,
    pub message: Option<String>,
}

// ============================================================================
// Account Types
// ============================================================================

/// Funds request
#[derive(Debug, Clone, Serialize)]
pub struct FundsRequest {
    pub apikey: String,
}

/// Funds data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundsData {
    pub availablecash: Option<String>,
    pub collateral: Option<String>,
    pub m2mrealized: Option<String>,
    pub m2munrealized: Option<String>,
    pub utiliseddebits: Option<String>,
}

/// Funds response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundsResponse {
    pub status: String,
    pub data: Option<FundsData>,
    pub message: Option<String>,
}

/// Margin position
#[derive(Debug, Clone, Serialize)]
pub struct MarginPosition {
    pub symbol: String,
    pub exchange: String,
    pub action: String,
    pub product: String,
    pub pricetype: String,
    pub quantity: String,
}

impl MarginPosition {
    /// Create a new margin position
    ///
    /// # Example
    /// ```rust
    /// use openalgo::MarginPosition;
    /// let pos = MarginPosition::new("NIFTY24DEC24000CE", "NFO", "BUY", "MIS", "MARKET", "50");
    /// ```
    pub fn new(symbol: &str, exchange: &str, action: &str, product: &str, pricetype: &str, quantity: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            action: action.to_string(),
            product: product.to_string(),
            pricetype: pricetype.to_string(),
            quantity: quantity.to_string(),
        }
    }
}

/// Margin request
#[derive(Debug, Clone, Serialize)]
pub struct MarginRequest {
    pub apikey: String,
    pub positions: Vec<MarginPosition>,
}

/// Margin data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginData {
    pub total_margin_required: Option<f64>,
    pub span_margin: Option<f64>,
    pub exposure_margin: Option<f64>,
}

/// Margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginResponse {
    pub status: String,
    pub data: Option<MarginData>,
    pub message: Option<String>,
}

/// Orderbook request
#[derive(Debug, Clone, Serialize)]
pub struct OrderbookRequest {
    pub apikey: String,
}

/// Order in orderbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookOrder {
    pub action: Option<String>,
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub orderid: Option<String>,
    pub product: Option<String>,
    pub quantity: Option<String>,
    pub price: Option<f64>,
    pub pricetype: Option<String>,
    pub order_status: Option<String>,
    pub trigger_price: Option<f64>,
    pub timestamp: Option<String>,
}

/// Orderbook statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookStatistics {
    pub total_buy_orders: Option<f64>,
    pub total_sell_orders: Option<f64>,
    pub total_completed_orders: Option<f64>,
    pub total_open_orders: Option<f64>,
    pub total_rejected_orders: Option<f64>,
}

/// Orderbook data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookData {
    pub orders: Option<Vec<OrderbookOrder>>,
    pub statistics: Option<OrderbookStatistics>,
}

/// Orderbook response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookResponse {
    pub status: String,
    pub data: Option<OrderbookData>,
    pub message: Option<String>,
}

/// Tradebook request
#[derive(Debug, Clone, Serialize)]
pub struct TradebookRequest {
    pub apikey: String,
}

/// Trade in tradebook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradebookTrade {
    pub action: Option<String>,
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub orderid: Option<String>,
    pub product: Option<String>,
    pub quantity: Option<f64>,
    pub average_price: Option<f64>,
    pub timestamp: Option<String>,
    pub trade_value: Option<f64>,
}

/// Tradebook response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradebookResponse {
    pub status: String,
    pub data: Option<Vec<TradebookTrade>>,
    pub message: Option<String>,
}

/// Positionbook request
#[derive(Debug, Clone, Serialize)]
pub struct PositionbookRequest {
    pub apikey: String,
}

/// Position in positionbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionbookPosition {
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub product: Option<String>,
    pub quantity: Option<String>,
    pub average_price: Option<String>,
    pub ltp: Option<String>,
    pub pnl: Option<String>,
}

/// Positionbook response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionbookResponse {
    pub status: String,
    pub data: Option<Vec<PositionbookPosition>>,
    pub message: Option<String>,
}

/// Holdings request
#[derive(Debug, Clone, Serialize)]
pub struct HoldingsRequest {
    pub apikey: String,
}

/// Holding item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingItem {
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub product: Option<String>,
    pub quantity: Option<i32>,
    pub pnl: Option<f64>,
    pub pnlpercent: Option<f64>,
}

/// Holdings statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsStatistics {
    pub totalholdingvalue: Option<f64>,
    pub totalinvvalue: Option<f64>,
    pub totalprofitandloss: Option<f64>,
    pub totalpnlpercentage: Option<f64>,
}

/// Holdings data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsData {
    pub holdings: Option<Vec<HoldingItem>>,
    pub statistics: Option<HoldingsStatistics>,
}

/// Holdings response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsResponse {
    pub status: String,
    pub data: Option<HoldingsData>,
    pub message: Option<String>,
}

/// Holidays request
#[derive(Debug, Clone, Serialize)]
pub struct HolidaysRequest {
    pub apikey: String,
    pub year: i32,
}

/// Open exchange timing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenExchangeTiming {
    pub exchange: String,
    pub start_time: i64,
    pub end_time: i64,
}

/// Holiday item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayItem {
    pub date: String,
    pub description: String,
    pub holiday_type: String,
    pub closed_exchanges: Vec<String>,
    pub open_exchanges: Vec<OpenExchangeTiming>,
}

/// Holidays response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidaysResponse {
    pub status: String,
    pub data: Option<Vec<HolidayItem>>,
    pub message: Option<String>,
}

/// Timings request
#[derive(Debug, Clone, Serialize)]
pub struct TimingsRequest {
    pub apikey: String,
    pub date: String,
}

/// Exchange timing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeTiming {
    pub exchange: String,
    pub start_time: i64,
    pub end_time: i64,
}

/// Timings response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingsResponse {
    pub status: String,
    pub data: Option<Vec<ExchangeTiming>>,
    pub message: Option<String>,
}

/// Telegram request
#[derive(Debug, Clone, Serialize)]
pub struct TelegramRequest {
    pub apikey: String,
    pub username: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// Telegram response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramResponse {
    pub status: String,
    pub message: Option<String>,
}

// ============================================================================
// Analyzer Types
// ============================================================================

/// Analyzer status request
#[derive(Debug, Clone, Serialize)]
pub struct AnalyzerStatusRequest {
    pub apikey: String,
}

/// Analyzer status data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerStatusData {
    pub analyze_mode: Option<bool>,
    pub mode: Option<String>,
    pub total_logs: Option<i32>,
}

/// Analyzer status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerStatusResponse {
    pub status: String,
    pub data: Option<AnalyzerStatusData>,
    pub message: Option<String>,
}

/// Analyzer toggle request
#[derive(Debug, Clone, Serialize)]
pub struct AnalyzerToggleRequest {
    pub apikey: String,
    pub mode: bool,
}

/// Analyzer toggle data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerToggleData {
    pub analyze_mode: Option<bool>,
    pub mode: Option<String>,
    pub total_logs: Option<i32>,
    pub message: Option<String>,
}

/// Analyzer toggle response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerToggleResponse {
    pub status: String,
    pub data: Option<AnalyzerToggleData>,
    pub message: Option<String>,
}

// ============================================================================
// WebSocket Types
// ============================================================================

/// WebSocket instrument for subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsInstrument {
    pub exchange: String,
    pub symbol: String,
}

impl WsInstrument {
    /// Create a new WebSocket instrument
    ///
    /// # Example
    /// ```rust
    /// use openalgo::WsInstrument;
    /// let instrument = WsInstrument::new("NSE", "RELIANCE");
    /// ```
    pub fn new(exchange: &str, symbol: &str) -> Self {
        Self {
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
        }
    }
}

/// WebSocket authentication message
#[derive(Debug, Clone, Serialize)]
pub struct WsAuthMessage {
    pub action: String,
    pub api_key: String,
}

/// WebSocket subscribe/unsubscribe message
#[derive(Debug, Clone, Serialize)]
pub struct WsSubscribeMessage {
    pub action: String,
    pub mode: String,
    pub symbols: Vec<WsInstrument>,
}

/// WebSocket LTP data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsLtpData {
    pub exchange: Option<String>,
    pub symbol: Option<String>,
    pub ltp: Option<f64>,
    pub timestamp: Option<i64>,
}

/// WebSocket Quote data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsQuoteData {
    pub exchange: Option<String>,
    pub symbol: Option<String>,
    pub ltp: Option<f64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<i64>,
    pub timestamp: Option<i64>,
}

/// WebSocket Depth data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsDepthData {
    pub exchange: Option<String>,
    pub symbol: Option<String>,
    pub ltp: Option<f64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<i64>,
    pub bids: Option<Vec<DepthLevel>>,
    pub asks: Option<Vec<DepthLevel>>,
    pub timestamp: Option<i64>,
}

/// WebSocket market data message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMarketDataMessage {
    #[serde(rename = "type")]
    pub msg_type: Option<String>,
    pub mode: Option<i32>,
    pub data: Option<serde_json::Value>,
}
