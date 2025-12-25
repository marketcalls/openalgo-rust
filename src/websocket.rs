//! WebSocket module for OpenAlgo real-time data.

use crate::client::OpenAlgoError;
use crate::types::*;
use futures_util::{SinkExt, StreamExt};
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

/// OpenAlgo WebSocket client for real-time market data
pub struct OpenAlgoWebSocket {
    api_key: String,
    ws_url: String,
}

impl OpenAlgoWebSocket {
    /// Create a new WebSocket client
    pub fn new(api_key: &str, ws_url: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            ws_url: ws_url.to_string(),
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
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(market_data) = serde_json::from_str::<WsMarketDataMessage>(&text)
                        {
                            let ws_data = parse_market_data(market_data);
                            let _ = data_tx_clone.send(ws_data).await;
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
