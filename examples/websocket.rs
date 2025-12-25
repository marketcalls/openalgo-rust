//! Example: WebSocket Streaming
//!
//! This example demonstrates how to use WebSocket for real-time market data.

use openalgo::{OpenAlgo, WsInstrument, WsData};
use openalgo::websocket::WsSubscriber;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client - simple!
    let api_key = std::env::var("OPENALGO_API_KEY").unwrap_or_else(|_| "your_api_key".to_string());
    let client = OpenAlgo::new(&api_key);

    // Create WebSocket client
    let ws = client.websocket();

    // Connect to WebSocket server
    println!("Connecting to WebSocket server...");
    let (cmd_tx, mut data_rx) = ws.connect().await?;

    // Create subscriber helper
    let subscriber = WsSubscriber::new(cmd_tx);

    // Define instruments - using the simple helper
    let instruments = vec![
        WsInstrument::new("NSE", "RELIANCE"),
        WsInstrument::new("NSE", "TCS"),
        WsInstrument::new("NSE", "INFY"),
    ];

    // Wait for connection
    if let Some(WsData::Connected) = data_rx.recv().await {
        println!("Connected to WebSocket server!");
    }

    // Subscribe to LTP updates
    println!("\n=== Subscribing to LTP ===");
    subscriber.subscribe_ltp(instruments.clone()).await?;

    // Process incoming data for 10 seconds
    println!("Receiving LTP data for 10 seconds...\n");
    let timeout = tokio::time::timeout(Duration::from_secs(10), async {
        while let Some(data) = data_rx.recv().await {
            match data {
                WsData::Ltp(ltp) => {
                    println!(
                        "LTP: {} {} - {}",
                        ltp.exchange.unwrap_or_default(),
                        ltp.symbol.unwrap_or_default(),
                        ltp.ltp.unwrap_or_default()
                    );
                }
                WsData::Quote(quote) => {
                    println!(
                        "Quote: {} {} - LTP: {}, Open: {}, High: {}, Low: {}, Volume: {}",
                        quote.exchange.unwrap_or_default(),
                        quote.symbol.unwrap_or_default(),
                        quote.ltp.unwrap_or_default(),
                        quote.open.unwrap_or_default(),
                        quote.high.unwrap_or_default(),
                        quote.low.unwrap_or_default(),
                        quote.volume.unwrap_or_default()
                    );
                }
                WsData::Depth(depth) => {
                    println!(
                        "Depth: {} {} - LTP: {}, Bids: {:?}, Asks: {:?}",
                        depth.exchange.unwrap_or_default(),
                        depth.symbol.unwrap_or_default(),
                        depth.ltp.unwrap_or_default(),
                        depth.bids.as_ref().map(|b| b.len()).unwrap_or(0),
                        depth.asks.as_ref().map(|a| a.len()).unwrap_or(0)
                    );
                }
                WsData::Disconnected => {
                    println!("Disconnected from server");
                    break;
                }
                WsData::Error(e) => {
                    println!("Error: {}", e);
                }
                _ => {}
            }
        }
    });

    let _ = timeout.await;

    // Unsubscribe from LTP and subscribe to Quote
    println!("\n=== Switching to Quote mode ===");
    subscriber.unsubscribe_ltp(instruments.clone()).await?;
    subscriber.subscribe_quote(instruments.clone()).await?;

    // Process quote data for 10 seconds
    println!("Receiving Quote data for 10 seconds...\n");
    let timeout = tokio::time::timeout(Duration::from_secs(10), async {
        while let Some(data) = data_rx.recv().await {
            match data {
                WsData::Quote(quote) => {
                    println!(
                        "Quote: {} {} - LTP: {}, Open: {}, High: {}, Low: {}",
                        quote.exchange.unwrap_or_default(),
                        quote.symbol.unwrap_or_default(),
                        quote.ltp.unwrap_or_default(),
                        quote.open.unwrap_or_default(),
                        quote.high.unwrap_or_default(),
                        quote.low.unwrap_or_default()
                    );
                }
                WsData::Disconnected => {
                    println!("Disconnected from server");
                    break;
                }
                WsData::Error(e) => {
                    println!("Error: {}", e);
                }
                _ => {}
            }
        }
    });

    let _ = timeout.await;

    // Disconnect
    println!("\n=== Disconnecting ===");
    subscriber.disconnect().await?;
    println!("Disconnected from WebSocket server");

    Ok(())
}
