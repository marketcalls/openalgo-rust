//! Example: Account Operations
//!
//! This example demonstrates how to access account information using the OpenAlgo SDK.

use openalgo::{OpenAlgo, MarginPosition};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client - simple!
    let api_key = std::env::var("OPENALGO_API_KEY").unwrap_or_else(|_| "your_api_key".to_string());
    let client = OpenAlgo::new(&api_key);

    // Example 1: Get account funds
    println!("=== Funds ===");
    let result = client.funds().await?;
    println!("Funds: {:?}", result);

    // Example 2: Get orderbook
    println!("\n=== Orderbook ===");
    let result = client.orderbook().await?;
    println!("Orderbook: {:?}", result);

    // Example 3: Get tradebook
    println!("\n=== Tradebook ===");
    let result = client.tradebook().await?;
    println!("Tradebook: {:?}", result);

    // Example 4: Get positionbook
    println!("\n=== Positionbook ===");
    let result = client.positionbook().await?;
    println!("Positionbook: {:?}", result);

    // Example 5: Get holdings
    println!("\n=== Holdings ===");
    let result = client.holdings().await?;
    println!("Holdings: {:?}", result);

    // Example 6: Get margin requirement
    println!("\n=== Margin ===");
    let positions = vec![
        MarginPosition::new("NIFTY24DEC24000CE", "NFO", "BUY", "MIS", "MARKET", "50"),
        MarginPosition::new("NIFTY24DEC24100CE", "NFO", "SELL", "MIS", "MARKET", "50"),
    ];
    let result = client.margin(positions).await?;
    println!("Margin: {:?}", result);

    // Example 7: Get holidays
    println!("\n=== Holidays ===");
    let result = client.holidays(2024).await?;
    println!("Holidays: {:?}", result);

    // Example 8: Get exchange timings
    println!("\n=== Timings ===");
    let result = client.timings("2024-12-25").await?;
    println!("Timings: {:?}", result);

    // Example 9: Send Telegram message
    println!("\n=== Telegram ===");
    let result = client.telegram("your_username", "Hello from OpenAlgo Rust SDK!").await?;
    println!("Telegram: {:?}", result);

    // Example 10: Get analyzer status
    println!("\n=== Analyzer Status ===");
    let result = client.analyzer_status().await?;
    println!("Analyzer Status: {:?}", result);

    // Example 11: Toggle analyzer mode
    println!("\n=== Toggle Analyzer ===");
    let result = client.analyzer_toggle(true).await?;
    println!("Analyzer Toggle: {:?}", result);

    Ok(())
}
