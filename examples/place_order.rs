//! Example: Place Order
//!
//! This example demonstrates how to place different types of orders using the OpenAlgo SDK.

use openalgo::{OpenAlgo, BasketOrderItem, OptionsLeg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client - simple!
    let api_key = std::env::var("OPENALGO_API_KEY").unwrap_or_else(|_| "your_api_key".to_string());
    let client = OpenAlgo::new(&api_key);

    // Example 1: Place a simple market order
    println!("=== Place Market Order ===");
    let result = client.place_order(
        "Strategy1",
        "RELIANCE",
        "BUY",
        "NSE",
        "MARKET",
        "MIS",
        "1",
    ).await?;
    println!("Order Result: {:?}", result);

    // Example 2: Place a limit order
    println!("\n=== Place Limit Order ===");
    let result = client.place_limit_order(
        "Strategy1",
        "RELIANCE",
        "BUY",
        "NSE",
        "MIS",
        "1",
        "2500.00",
    ).await?;
    println!("Limit Order Result: {:?}", result);

    // Example 3: Place a stop-loss order
    println!("\n=== Place Stop-Loss Order ===");
    let result = client.place_sl_order(
        "Strategy1",
        "RELIANCE",
        "BUY",
        "NSE",
        "MIS",
        "1",
        "2500.00",
        "2490.00",
    ).await?;
    println!("SL Order Result: {:?}", result);

    // Example 4: Place a smart order
    println!("\n=== Place Smart Order ===");
    let result = client.place_smart_order(
        "Strategy1",
        "RELIANCE",
        "BUY",
        "NSE",
        "MARKET",
        "MIS",
        "1",
        "5",
    ).await?;
    println!("Smart Order Result: {:?}", result);

    // Example 5: Place basket orders
    println!("\n=== Basket Order ===");
    let orders = vec![
        BasketOrderItem::new("RELIANCE", "NSE", "BUY", "1", "MARKET", "MIS"),
        BasketOrderItem::new("TCS", "NSE", "BUY", "1", "MARKET", "MIS"),
    ];
    let result = client.basket_order("Strategy1", orders).await?;
    println!("Basket Order Result: {:?}", result);

    // Example 6: Place split order
    println!("\n=== Split Order ===");
    let result = client.split_order(
        "Strategy1",
        "RELIANCE",
        "BUY",
        "NSE",
        "100",
        "25",
        "MARKET",
        "MIS",
    ).await?;
    println!("Split Order Result: {:?}", result);

    // Example 7: Modify order
    println!("\n=== Modify Order ===");
    let result = client.modify_order(
        "123456789",
        "Strategy1",
        "RELIANCE",
        "BUY",
        "NSE",
        "LIMIT",
        "MIS",
        "1",
        "2550.00",
    ).await?;
    println!("Modify Order Result: {:?}", result);

    // Example 8: Cancel order
    println!("\n=== Cancel Order ===");
    let result = client.cancel_order("123456789", "Strategy1").await?;
    println!("Cancel Order Result: {:?}", result);

    // Example 9: Cancel all orders
    println!("\n=== Cancel All Orders ===");
    let result = client.cancel_all_order("Strategy1").await?;
    println!("Cancel All Orders Result: {:?}", result);

    // Example 10: Close position
    println!("\n=== Close Position ===");
    let result = client.close_position("Strategy1").await?;
    println!("Close Position Result: {:?}", result);

    // Example 11: Get order status
    println!("\n=== Order Status ===");
    let result = client.order_status("123456789", "Strategy1").await?;
    println!("Order Status: {:?}", result);

    // Example 12: Get open position
    println!("\n=== Open Position ===");
    let result = client.open_position("Strategy1", "RELIANCE", "NSE", "MIS").await?;
    println!("Open Position: {:?}", result);

    Ok(())
}
