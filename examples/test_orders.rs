//! Test order endpoints with real API key

use openalgo::{OpenAlgo, BasketOrderItem, OptionsLeg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with API key
    let client = OpenAlgo::new("b7403124093d1f561fd3dd4666bcc78d49ab1d95568cd3851085e816a74b490c");

    // Test 1: Basket Order
    // BasketOrderItem::new(symbol, exchange, action, quantity, pricetype, product)
    println!("=== Testing Basket Order ===");
    let basket_items = vec![
        BasketOrderItem::new("RELIANCE", "NSE", "BUY", 1, "MARKET", "MIS"),
        BasketOrderItem::new("TCS", "NSE", "BUY", 1, "MARKET", "MIS"),
    ];

    match client.basket_order("Test", basket_items).await {
        Ok(result) => println!("Basket Order: {:?}", result),
        Err(e) => println!("Basket Order Error: {:?}", e),
    }

    // Test 2: Split Order
    // split_order(strategy, symbol, action, exchange, quantity, splitsize, pricetype, product)
    println!("\n=== Testing Split Order ===");
    match client.split_order(
        "Test",
        "RELIANCE",
        "BUY",
        "NSE",
        10,     // quantity as i32
        2,      // splitsize as i32
        "MARKET",
        "MIS",
    ).await {
        Ok(result) => println!("Split Order: {:?}", result),
        Err(e) => println!("Split Order Error: {:?}", e),
    }

    // Test 3: Options Order (ATM Call with 26DEC24 expiry)
    println!("\n=== Testing Options Order (ATM Call) ===");
    match client.options_order(
        "Test",
        "NIFTY",
        "NFO",
        "241226",   // 26DEC24
        "0",        // ATM
        "CE",       // Call
        "BUY",
        "50",       // Lot size
        "MARKET",
        "MIS",
        "25",       // Quantity
    ).await {
        Ok(result) => println!("Options Order: {:?}", result),
        Err(e) => println!("Options Order Error: {:?}", e),
    }

    // Test 4: Options Multi-Leg Order (Bull Call Spread)
    println!("\n=== Testing Options Multi-Leg (Bull Call Spread) ===");
    let legs = vec![
        OptionsLeg::new("0", "CE", "BUY", "50"),   // Buy ATM Call
        OptionsLeg::new("2", "CE", "SELL", "50"),  // Sell OTM Call
    ];

    match client.options_multi_order(
        "Test",
        "NIFTY",
        "NFO",
        "241226",   // 26DEC24 expiry
        legs,
    ).await {
        Ok(result) => println!("Multi-Leg Options: {:?}", result),
        Err(e) => println!("Multi-Leg Options Error: {:?}", e),
    }

    Ok(())
}
