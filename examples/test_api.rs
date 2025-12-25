//! Test API endpoints with real API key

use openalgo::OpenAlgo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with API key
    let client = OpenAlgo::new("b7403124093d1f561fd3dd4666bcc78d49ab1d95568cd3851085e816a74b490c");

    // Test 1: Get Quotes
    println!("=== Testing Quotes ===");
    match client.quotes("RELIANCE", "NSE").await {
        Ok(result) => println!("Quotes: {:?}", result),
        Err(e) => println!("Quotes Error: {:?}", e),
    }

    // Test 2: Get History with date range
    println!("\n=== Testing History (with date range) ===");
    match client.history_range("RELIANCE", "NSE", "D", "2024-12-01", "2024-12-25").await {
        Ok(result) => println!("History Range: {:?}", result),
        Err(e) => println!("History Range Error: {:?}", e),
    }

    // Test 3: Send Telegram
    println!("\n=== Testing Telegram ===");
    match client.telegram("rajandran", "Hello from OpenAlgo Rust SDK!").await {
        Ok(result) => println!("Telegram: {:?}", result),
        Err(e) => println!("Telegram Error: {:?}", e),
    }

    // Test 4: Get Holidays
    println!("\n=== Testing Holidays ===");
    match client.holidays(2024).await {
        Ok(result) => println!("Holidays: {:?}", result),
        Err(e) => println!("Holidays Error: {:?}", e),
    }

    // Test 5: Get Timings
    println!("\n=== Testing Timings ===");
    match client.timings("2024-12-25").await {
        Ok(result) => println!("Timings: {:?}", result),
        Err(e) => println!("Timings Error: {:?}", e),
    }

    Ok(())
}
