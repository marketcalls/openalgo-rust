//! Example: Market Data
//!
//! This example demonstrates how to fetch market data using the OpenAlgo SDK.

use openalgo::OpenAlgo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client - simple!
    let api_key = std::env::var("OPENALGO_API_KEY").unwrap_or_else(|_| "your_api_key".to_string());
    let client = OpenAlgo::new(&api_key);

    // Example 1: Get quotes for a symbol
    println!("=== Quotes ===");
    let result = client.quotes("RELIANCE", "NSE").await?;
    println!("Quotes: {:?}", result);

    // Example 2: Get quotes for multiple symbols
    println!("\n=== Multi Quotes ===");
    let result = client.multi_quotes(&[
        ("RELIANCE", "NSE"),
        ("TCS", "NSE"),
        ("INFY", "NSE"),
    ]).await?;
    println!("Multi Quotes: {:?}", result);

    // Example 3: Get market depth
    println!("\n=== Market Depth ===");
    let result = client.depth("RELIANCE", "NSE").await?;
    println!("Depth: {:?}", result);

    // Example 4: Get historical data (simple)
    println!("\n=== Historical Data ===");
    let result = client.history("RELIANCE", "NSE", "5m").await?;
    println!("History: {:?}", result);

    // Example 5: Get historical data with date range
    println!("\n=== Historical Data (Date Range) ===");
    let result = client.history_range("RELIANCE", "NSE", "5m", "2024-01-01", "2024-01-31").await?;
    println!("History Range: {:?}", result);

    // Example 6: Get available intervals
    println!("\n=== Intervals ===");
    let result = client.intervals().await?;
    println!("Intervals: {:?}", result);

    // Example 7: Get symbol info
    println!("\n=== Symbol Info ===");
    let result = client.symbol("RELIANCE", "NSE").await?;
    println!("Symbol: {:?}", result);

    // Example 8: Search symbols
    println!("\n=== Search ===");
    let result = client.search("RELI", "NSE").await?;
    println!("Search Results: {:?}", result);

    // Example 9: Get expiry dates
    println!("\n=== Expiry Dates ===");
    let result = client.expiry("NIFTY", "NFO", "OPT").await?;
    println!("Expiry Dates: {:?}", result);

    // Example 10: Get option chain
    println!("\n=== Option Chain ===");
    let result = client.option_chain("NIFTY", "NFO", "241226").await?;
    println!("Option Chain: {:?}", result);

    // Example 11: Get option symbol
    println!("\n=== Option Symbol ===");
    let result = client.option_symbol("NIFTY", "NFO", "241226", "0", "CE").await?;
    println!("Option Symbol: {:?}", result);

    // Example 12: Get synthetic future price
    println!("\n=== Synthetic Future ===");
    let result = client.synthetic_future("NIFTY", "NFO", "241226").await?;
    println!("Synthetic Future: {:?}", result);

    // Example 13: Get option Greeks
    println!("\n=== Option Greeks ===");
    let result = client.option_greeks(
        "NIFTY24DEC24000CE",
        "NFO",
        6.5,
        "NIFTY",
        "NSE",
    ).await?;
    println!("Option Greeks: {:?}", result);

    // Example 14: Get instruments
    println!("\n=== Instruments ===");
    let result = client.instruments("NSE").await?;
    println!("Total Instruments: {:?}", result.status);

    Ok(())
}
