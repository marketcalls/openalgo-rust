//! WebSocket module for OpenAlgo real-time data.

use crate::client::OpenAlgoError;
use crate::types::*;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

/// WebSocket data types
#[derive(Debug, Clone)]
pub enum WsData {
    Ltp(WsLtpData),
    Quote(WsQuoteData),
    Depth(WsDepthData),
    Connected,
    Disconnected,
    Error(String),
}

/// Local snapshot cache keyed by `"EXCHANGE:SYMBOL"`, populated as market data
/// arrives on the WebSocket. Mirrors Python `FeedAPI`'s `ltp_data` / `quotes_data`
/// / `depth_data` dicts.
type SnapshotCache = Arc<Mutex<HashMap<String, serde_json::Value>>>;

/// OpenAlgo WebSocket client for real-time market data
pub struct OpenAlgoWebSocket {
    api_key: String,
    ws_url: String,
    ltp_cache: SnapshotCache,
    quotes_cache: SnapshotCache,
    depth_cache: SnapshotCache,
}

impl OpenAlgoWebSocket {
    /// Create a new WebSocket client
    pub fn new(api_key: &str, ws_url: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            ws_url: ws_url.to_string(),
            ltp_cache: Arc::new(Mutex::new(HashMap::new())),
            quotes_cache: Arc::new(Mutex::new(HashMap::new())),
            depth_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Connect to WebSocket server and return channels for communication
    ///
    /// Returns a tuple of (sender for commands, receiver for data)
    pub async fn connect(
        &self,
    ) -> Result<
        (
            mpsc::Sender<WsCommand>,
            mpsc::Receiver<WsData>,
        ),
        OpenAlgoError,
    > {
        let url = Url::parse(&self.ws_url)?;
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))?;

        let (mut write, mut read) = ws_stream.split();

        // Send authentication message
        let auth_msg = WsAuthMessage {
            action: "authenticate".to_string(),
            api_key: self.api_key.clone(),
        };
        let auth_json = serde_json::to_string(&auth_msg)?;
        write
            .send(Message::Text(auth_json))
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))?;

        // Create channels
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<WsCommand>(32);
        let (data_tx, data_rx) = mpsc::channel::<WsData>(128);

        // Spawn reader task
        let data_tx_clone = data_tx.clone();
        let ltp_cache = Arc::clone(&self.ltp_cache);
        let quotes_cache = Arc::clone(&self.quotes_cache);
        let depth_cache = Arc::clone(&self.depth_cache);
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(raw) = serde_json::from_str::<serde_json::Value>(&text) {
                            update_snapshot_cache(&raw, &ltp_cache, &quotes_cache, &depth_cache);

                            if let Ok(market_data) =
                                serde_json::from_value::<WsMarketDataMessage>(raw)
                            {
                                let ws_data = parse_market_data(market_data);
                                let _ = data_tx_clone.send(ws_data).await;
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        let _ = data_tx_clone.send(WsData::Disconnected).await;
                        break;
                    }
                    Err(e) => {
                        let _ = data_tx_clone.send(WsData::Error(e.to_string())).await;
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Spawn writer task
        tokio::spawn(async move {
            while let Some(cmd) = cmd_rx.recv().await {
                let msg = match cmd {
                    WsCommand::SubscribeLtp(instruments) => {
                        create_subscribe_message("subscribe", "ltp", instruments)
                    }
                    WsCommand::UnsubscribeLtp(instruments) => {
                        create_subscribe_message("unsubscribe", "ltp", instruments)
                    }
                    WsCommand::SubscribeQuote(instruments) => {
                        create_subscribe_message("subscribe", "quote", instruments)
                    }
                    WsCommand::UnsubscribeQuote(instruments) => {
                        create_subscribe_message("unsubscribe", "quote", instruments)
                    }
                    WsCommand::SubscribeDepth(instruments) => {
                        create_subscribe_message("subscribe", "depth", instruments)
                    }
                    WsCommand::UnsubscribeDepth(instruments) => {
                        create_subscribe_message("unsubscribe", "depth", instruments)
                    }
                    WsCommand::Disconnect => {
                        let _ = write.close().await;
                        break;
                    }
                };

                if let Some(json) = msg {
                    let _ = write.send(Message::Text(json)).await;
                }
            }
        });

        // Send connected notification
        let _ = data_tx.send(WsData::Connected).await;

        Ok((cmd_tx, data_rx))
    }

    /// Get the latest cached LTP snapshot, optionally filtered by exchange and/or symbol.
    ///
    /// Populated as `market_data` messages arrive after calling [`OpenAlgoWebSocket::connect`]
    /// and subscribing. Mirrors Python's `FeedAPI.get_ltp()` nested shape:
    /// `{"ltp": {"EXCHANGE": {"SYMBOL": {"timestamp": ..., "ltp": ...}}}}`.
    pub fn get_ltp(&self, exchange: Option<&str>, symbol: Option<&str>) -> serde_json::Value {
        let mut ltp_obj = serde_json::Map::new();
        let cache = self.ltp_cache.lock().unwrap_or_else(|e| e.into_inner());
        for (key, data) in cache.iter() {
            if let Some((ex, sym)) = key.split_once(':') {
                if exchange.is_some_and(|f| f != ex) {
                    continue;
                }
                if symbol.is_some_and(|f| f != sym) {
                    continue;
                }
                insert_nested(&mut ltp_obj, ex, sym, data.clone());
            }
        }
        serde_json::json!({ "ltp": ltp_obj })
    }

    /// Get the latest cached Quote snapshot, optionally filtered by exchange and/or symbol.
    ///
    /// Mirrors Python's `FeedAPI.get_quotes()` nested shape:
    /// `{"quote": {"EXCHANGE": {"SYMBOL": {"open", "high", "low", "close", "ltp", "volume", ...}}}}`.
    pub fn get_quotes(&self, exchange: Option<&str>, symbol: Option<&str>) -> serde_json::Value {
        let mut quote_obj = serde_json::Map::new();
        let cache = self.quotes_cache.lock().unwrap_or_else(|e| e.into_inner());
        for (key, data) in cache.iter() {
            if let Some((ex, sym)) = key.split_once(':') {
                if exchange.is_some_and(|f| f != ex) {
                    continue;
                }
                if symbol.is_some_and(|f| f != sym) {
                    continue;
                }
                insert_nested(&mut quote_obj, ex, sym, data.clone());
            }
        }
        serde_json::json!({ "quote": quote_obj })
    }

    /// Get the latest cached Market Depth snapshot, optionally filtered by exchange and/or symbol.
    ///
    /// Mirrors Python's `FeedAPI.get_depth()` nested shape, with `buyBook`/`sellBook`
    /// padded to 5 levels: `{"depth": {"EXCHANGE": {"SYMBOL": {"timestamp", "ltp",
    /// "buyBook": {"1": {"price", "qty", "orders"}, ...}, "sellBook": {...}}}}}`.
    pub fn get_depth(&self, exchange: Option<&str>, symbol: Option<&str>) -> serde_json::Value {
        let mut depth_obj = serde_json::Map::new();
        let cache = self.depth_cache.lock().unwrap_or_else(|e| e.into_inner());
        for (key, data) in cache.iter() {
            if let Some((ex, sym)) = key.split_once(':') {
                if exchange.is_some_and(|f| f != ex) {
                    continue;
                }
                if symbol.is_some_and(|f| f != sym) {
                    continue;
                }

                let empty: Vec<serde_json::Value> = Vec::new();
                let buy_levels = data
                    .get("depth")
                    .and_then(|d| d.get("buy"))
                    .and_then(|v| v.as_array())
                    .unwrap_or(&empty);
                let sell_levels = data
                    .get("depth")
                    .and_then(|d| d.get("sell"))
                    .and_then(|v| v.as_array())
                    .unwrap_or(&empty);

                let entry = serde_json::json!({
                    "timestamp": data.get("timestamp").cloned().unwrap_or(serde_json::json!(0)),
                    "ltp": data.get("ltp").cloned().unwrap_or(serde_json::json!(0)),
                    "buyBook": build_depth_book(buy_levels),
                    "sellBook": build_depth_book(sell_levels),
                });
                insert_nested(&mut depth_obj, ex, sym, entry);
            }
        }
        serde_json::json!({ "depth": depth_obj })
    }
}

/// Insert `value` into `obj[exchange][symbol]`, creating the exchange-level
/// object on first use. Shared by the `get_ltp` / `get_quotes` / `get_depth` snapshot getters.
fn insert_nested(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    exchange: &str,
    symbol: &str,
    value: serde_json::Value,
) {
    let ex_entry = obj
        .entry(exchange.to_string())
        .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
    if let serde_json::Value::Object(m) = ex_entry {
        m.insert(symbol.to_string(), value);
    }
}

/// Build a 5-level `{"1": {"price", "qty", "orders"}, ..., "5": {...}}` depth
/// book from a raw array of `{"price", "quantity", "orders"}` levels, padding
/// missing levels with zeros — matching Python's `get_depth()` transform.
fn build_depth_book(levels: &[serde_json::Value]) -> serde_json::Value {
    let mut book = serde_json::Map::new();
    for i in 0..5 {
        let level = levels.get(i);
        let price = level
            .and_then(|l| l.get("price"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let qty = level
            .and_then(|l| l.get("quantity"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let orders = level
            .and_then(|l| l.get("orders"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        book.insert(
            (i + 1).to_string(),
            serde_json::json!({ "price": price, "qty": qty, "orders": orders }),
        );
    }
    serde_json::Value::Object(book)
}

/// Update the LTP/Quote/Depth snapshot caches from a raw `market_data` WebSocket
/// message, mirroring Python's `FeedAPI._process_message()`. No-ops for any
/// other message type (auth, subscribe acks, etc).
fn update_snapshot_cache(
    raw: &serde_json::Value,
    ltp_cache: &SnapshotCache,
    quotes_cache: &SnapshotCache,
    depth_cache: &SnapshotCache,
) {
    if raw.get("type").and_then(|v| v.as_str()) != Some("market_data") {
        return;
    }

    let (Some(exchange), Some(symbol)) = (
        raw.get("exchange").and_then(|v| v.as_str()),
        raw.get("symbol").and_then(|v| v.as_str()),
    ) else {
        return;
    };

    let mode = raw.get("mode").and_then(|v| v.as_i64()).unwrap_or(0);
    let data = raw.get("data").cloned().unwrap_or(serde_json::Value::Null);
    let key = format!("{}:{}", exchange, symbol);
    let now_ms = || {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    };
    let timestamp = data
        .get("timestamp")
        .and_then(|v| v.as_i64())
        .unwrap_or_else(now_ms);

    match mode {
        1 => {
            let entry = serde_json::json!({
                "ltp": data.get("ltp").cloned().unwrap_or(serde_json::json!(0)),
                "timestamp": timestamp,
            });
            ltp_cache.lock().unwrap_or_else(|e| e.into_inner()).insert(key, entry);
        }
        2 => {
            let entry = serde_json::json!({
                "open": data.get("open").cloned().unwrap_or(serde_json::json!(0)),
                "high": data.get("high").cloned().unwrap_or(serde_json::json!(0)),
                "low": data.get("low").cloned().unwrap_or(serde_json::json!(0)),
                "close": data.get("close").cloned().unwrap_or(serde_json::json!(0)),
                "ltp": data.get("ltp").cloned().unwrap_or(serde_json::json!(0)),
                "volume": data.get("volume").cloned().unwrap_or(serde_json::json!(0)),
                "last_trade_quantity": data.get("last_trade_quantity").cloned().unwrap_or(serde_json::json!(0)),
                "avg_trade_price": data.get("avg_trade_price").cloned().unwrap_or(serde_json::json!(0)),
                "change": data.get("change").cloned().unwrap_or(serde_json::json!(0)),
                "change_percent": data.get("change_percent").cloned().unwrap_or(serde_json::json!(0)),
                "timestamp": timestamp,
            });
            quotes_cache.lock().unwrap_or_else(|e| e.into_inner()).insert(key, entry);
        }
        3 => {
            let entry = serde_json::json!({
                "ltp": data.get("ltp").cloned().unwrap_or(serde_json::json!(0)),
                "timestamp": timestamp,
                "depth": data.get("depth").cloned().unwrap_or_else(|| serde_json::json!({"buy": [], "sell": []})),
            });
            depth_cache.lock().unwrap_or_else(|e| e.into_inner()).insert(key, entry);
        }
        _ => {}
    }
}

/// WebSocket commands
#[derive(Debug, Clone)]
pub enum WsCommand {
    SubscribeLtp(Vec<WsInstrument>),
    UnsubscribeLtp(Vec<WsInstrument>),
    SubscribeQuote(Vec<WsInstrument>),
    UnsubscribeQuote(Vec<WsInstrument>),
    SubscribeDepth(Vec<WsInstrument>),
    UnsubscribeDepth(Vec<WsInstrument>),
    Disconnect,
}

fn create_subscribe_message(
    action: &str,
    mode: &str,
    instruments: Vec<WsInstrument>,
) -> Option<String> {
    let msg = WsSubscribeMessage {
        action: action.to_string(),
        mode: mode.to_string(),
        symbols: instruments,
    };
    serde_json::to_string(&msg).ok()
}

fn parse_market_data(msg: WsMarketDataMessage) -> WsData {
    let mode = msg.mode.unwrap_or(0);

    match mode {
        1 => {
            // LTP mode
            if let Some(data) = msg.data {
                if let Ok(ltp_data) = serde_json::from_value::<WsLtpData>(data) {
                    return WsData::Ltp(ltp_data);
                }
            }
            WsData::Error("Failed to parse LTP data".to_string())
        }
        2 => {
            // Quote mode
            if let Some(data) = msg.data {
                if let Ok(quote_data) = serde_json::from_value::<WsQuoteData>(data) {
                    return WsData::Quote(quote_data);
                }
            }
            WsData::Error("Failed to parse Quote data".to_string())
        }
        3 => {
            // Depth mode
            if let Some(data) = msg.data {
                if let Ok(depth_data) = serde_json::from_value::<WsDepthData>(data) {
                    return WsData::Depth(depth_data);
                }
            }
            WsData::Error("Failed to parse Depth data".to_string())
        }
        _ => WsData::Error(format!("Unknown mode: {}", mode)),
    }
}

/// Helper struct for easy WebSocket subscriptions
pub struct WsSubscriber {
    cmd_tx: mpsc::Sender<WsCommand>,
}

impl WsSubscriber {
    /// Create a new subscriber from command sender
    pub fn new(cmd_tx: mpsc::Sender<WsCommand>) -> Self {
        Self { cmd_tx }
    }

    /// Subscribe to LTP updates
    pub async fn subscribe_ltp(&self, instruments: Vec<WsInstrument>) -> Result<(), OpenAlgoError> {
        self.cmd_tx
            .send(WsCommand::SubscribeLtp(instruments))
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))
    }

    /// Unsubscribe from LTP updates
    pub async fn unsubscribe_ltp(&self, instruments: Vec<WsInstrument>) -> Result<(), OpenAlgoError> {
        self.cmd_tx
            .send(WsCommand::UnsubscribeLtp(instruments))
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))
    }

    /// Subscribe to Quote updates
    pub async fn subscribe_quote(&self, instruments: Vec<WsInstrument>) -> Result<(), OpenAlgoError> {
        self.cmd_tx
            .send(WsCommand::SubscribeQuote(instruments))
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))
    }

    /// Unsubscribe from Quote updates
    pub async fn unsubscribe_quote(&self, instruments: Vec<WsInstrument>) -> Result<(), OpenAlgoError> {
        self.cmd_tx
            .send(WsCommand::UnsubscribeQuote(instruments))
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))
    }

    /// Subscribe to Depth updates
    pub async fn subscribe_depth(&self, instruments: Vec<WsInstrument>) -> Result<(), OpenAlgoError> {
        self.cmd_tx
            .send(WsCommand::SubscribeDepth(instruments))
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))
    }

    /// Unsubscribe from Depth updates
    pub async fn unsubscribe_depth(&self, instruments: Vec<WsInstrument>) -> Result<(), OpenAlgoError> {
        self.cmd_tx
            .send(WsCommand::UnsubscribeDepth(instruments))
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))
    }

    /// Disconnect from WebSocket
    pub async fn disconnect(&self) -> Result<(), OpenAlgoError> {
        self.cmd_tx
            .send(WsCommand::Disconnect)
            .await
            .map_err(|e| OpenAlgoError::WebSocketError(e.to_string()))
    }
}
