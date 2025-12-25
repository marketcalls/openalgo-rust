//! Example: Options Order
//!
//! This example demonstrates how to place options orders using the OpenAlgo SDK.

use openalgo::{OpenAlgo, OptionsLeg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client - simple!
    let api_key = std::env::var("OPENALGO_API_KEY").unwrap_or_else(|_| "your_api_key".to_string());
    let client = OpenAlgo::new(&api_key);

    // Example 1: Place a simple options order (ATM Call)
    println!("=== Options Order - ATM Call ===");
    let result = client.options_order(
        "Strategy1",
        "NIFTY",
        "NFO",
        "241226",
        "0",              // ATM
        "CE",
        "BUY",
        "50",
        "MARKET",
        "MIS",
        "50",
    ).await?;
    println!("Options Order Result: {:?}", result);

    // Example 2: Place an OTM Put order
    println!("\n=== Options Order - OTM Put ===");
    let result = client.options_order(
        "Strategy1",
        "BANKNIFTY",
        "NFO",
        "241226",
        "-2",             // 2 strikes below ATM
        "PE",
        "BUY",
        "30",
        "MARKET",
        "MIS",
        "15",
    ).await?;
    println!("OTM Put Order Result: {:?}", result);

    // Example 3: Place a multi-leg options order (Bull Call Spread)
    println!("\n=== Multi-Leg - Bull Call Spread ===");
    let legs = vec![
        OptionsLeg::new("0", "CE", "BUY", "50"),   // Buy ATM Call
        OptionsLeg::new("2", "CE", "SELL", "50"),  // Sell OTM Call
    ];

    let result = client.options_multi_order(
        "Strategy1",
        "NIFTY",
        "NFO",
        "241226",
        legs,
    ).await?;
    println!("Bull Call Spread Result: {:?}", result);

    // Example 4: Place an Iron Condor
    println!("\n=== Multi-Leg - Iron Condor ===");
    let legs = vec![
        OptionsLeg::new("-2", "PE", "BUY", "50"),   // Buy OTM Put
        OptionsLeg::new("-1", "PE", "SELL", "50"),  // Sell ATM Put
        OptionsLeg::new("1", "CE", "SELL", "50"),   // Sell ATM Call
        OptionsLeg::new("2", "CE", "BUY", "50"),    // Buy OTM Call
    ];

    let result = client.options_multi_order(
        "Strategy1",
        "NIFTY",
        "NFO",
        "241226",
        legs,
    ).await?;
    println!("Iron Condor Result: {:?}", result);

    // Example 5: Calendar Spread (different expiries)
    println!("\n=== Multi-Leg - Calendar Spread ===");
    let legs = vec![
        OptionsLeg::with_expiry("0", "CE", "SELL", "50", "241226"),  // Sell near expiry
        OptionsLeg::with_expiry("0", "CE", "BUY", "50", "250102"),   // Buy far expiry
    ];

    let result = client.options_multi_order(
        "Strategy1",
        "NIFTY",
        "NFO",
        "241226",
        legs,
    ).await?;
    println!("Calendar Spread Result: {:?}", result);

    Ok(())
}
