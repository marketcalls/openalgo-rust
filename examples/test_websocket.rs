//! Test WebSocket endpoints

use openalgo::{OpenAlgo, WsInstrument};
use openalgo::websocket::{WsSubscriber, WsData};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with API key
    let api_key = "b7403124093d1f561fd3dd4666bcc78d49ab1d95568cd3851085e816a74b490c";
    let client = OpenAlgo::new(api_key);

    // Create WebSocket client
    let ws = client.websocket();

    // Test: Connect to WebSocket server
    println!("=== Testing WebSocket Connection ===");
    match ws.connect().await {
        Ok((cmd_tx, mut data_rx)) => {
            println!("WebSocket connected!");

            // Create subscriber
            let subscriber = WsSubscriber::new(cmd_tx);

            // Define instruments
            let instruments = vec![
                WsInstrument::new("NSE", "RELIANCE"),
                WsInstrument::new("NSE", "TCS"),
            ];

            // Wait for connection confirmation
            let timeout = tokio::time::timeout(Duration::from_secs(5), async {
                if let Some(data) = data_rx.recv().await {
                    match data {
                        WsData::Connected => {
                            println!("Connection confirmed!");
                            return true;
                        }
                        WsData::Error(e) => {
                            println!("Error: {}", e);
                            return false;
                        }
                        _ => {}
                    }
                }
                false
            });

            if let Ok(connected) = timeout.await {
                if connected {
                    // Test LTP subscription
                    println!("\n=== Testing LTP Subscribe ===");
                    match subscriber.subscribe_ltp(instruments.clone()).await {
                        Ok(_) => println!("LTP Subscribe: SUCCESS"),
                        Err(e) => println!("LTP Subscribe Error: {:?}", e),
                    }

                    // Wait a moment
                    tokio::time::sleep(Duration::from_secs(2)).await;

                    // Test Quote subscription
                    println!("\n=== Testing Quote Subscribe ===");
                    match subscriber.subscribe_quote(instruments.clone()).await {
                        Ok(_) => println!("Quote Subscribe: SUCCESS"),
                        Err(e) => println!("Quote Subscribe Error: {:?}", e),
                    }

                    // Wait a moment
                    tokio::time::sleep(Duration::from_secs(2)).await;

                    // Test Depth subscription
                    println!("\n=== Testing Depth Subscribe ===");
                    match subscriber.subscribe_depth(instruments.clone()).await {
                        Ok(_) => println!("Depth Subscribe: SUCCESS"),
                        Err(e) => println!("Depth Subscribe Error: {:?}", e),
                    }

                    // Wait a moment
                    tokio::time::sleep(Duration::from_secs(2)).await;

                    // Test unsubscribe
                    println!("\n=== Testing Unsubscribe ===");
                    match subscriber.unsubscribe_ltp(instruments.clone()).await {
                        Ok(_) => println!("LTP Unsubscribe: SUCCESS"),
                        Err(e) => println!("LTP Unsubscribe Error: {:?}", e),
                    }

                    // Receive any data that came in
                    println!("\n=== Checking for market data (5 seconds) ===");
                    let data_timeout = tokio::time::timeout(Duration::from_secs(5), async {
                        while let Some(data) = data_rx.recv().await {
                            match data {
                                WsData::Ltp(ltp) => {
                                    println!("LTP: {} {} - {}",
                                        ltp.exchange.unwrap_or_default(),
                                        ltp.symbol.unwrap_or_default(),
                                        ltp.ltp.unwrap_or_default());
                                }
                                WsData::Quote(quote) => {
                                    println!("Quote: {} {} - LTP: {}",
                                        quote.exchange.unwrap_or_default(),
                                        quote.symbol.unwrap_or_default(),
                                        quote.ltp.unwrap_or_default());
                                }
                                WsData::Depth(depth) => {
                                    println!("Depth: {} {} - LTP: {}",
                                        depth.exchange.unwrap_or_default(),
                                        depth.symbol.unwrap_or_default(),
                                        depth.ltp.unwrap_or_default());
                                }
                                WsData::Error(e) => {
                                    println!("Data Error: {}", e);
                                }
                                WsData::Disconnected => {
                                    println!("Disconnected");
                                    break;
                                }
                                _ => {}
                            }
                        }
                    });

                    let _ = data_timeout.await;
                    println!("(Market may be closed - no data received)");

                    // Disconnect
                    println!("\n=== Testing Disconnect ===");
                    match subscriber.disconnect().await {
                        Ok(_) => println!("Disconnect: SUCCESS"),
                        Err(e) => println!("Disconnect Error: {:?}", e),
                    }
                }
            } else {
                println!("Connection timeout!");
            }
        }
        Err(e) => {
            println!("WebSocket connection failed: {:?}", e);
        }
    }

    Ok(())
}
