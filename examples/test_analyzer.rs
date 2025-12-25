//! Test Analyzer API endpoints

use openalgo::OpenAlgo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with API key
    let client = OpenAlgo::new("b7403124093d1f561fd3dd4666bcc78d49ab1d95568cd3851085e816a74b490c");

    // Test 1: Get Analyzer Status
    println!("=== Testing Analyzer Status ===");
    match client.analyzer_status().await {
        Ok(result) => println!("Analyzer Status: {:?}", result),
        Err(e) => println!("Analyzer Status Error: {:?}", e),
    }

    // Test 2: Toggle Analyzer ON
    println!("\n=== Testing Analyzer Toggle (ON) ===");
    match client.analyzer_toggle(true).await {
        Ok(result) => println!("Analyzer Toggle ON: {:?}", result),
        Err(e) => println!("Analyzer Toggle ON Error: {:?}", e),
    }

    // Test 3: Get Analyzer Status again
    println!("\n=== Testing Analyzer Status (after toggle ON) ===");
    match client.analyzer_status().await {
        Ok(result) => println!("Analyzer Status: {:?}", result),
        Err(e) => println!("Analyzer Status Error: {:?}", e),
    }

    // Test 4: Toggle Analyzer OFF
    println!("\n=== Testing Analyzer Toggle (OFF) ===");
    match client.analyzer_toggle(false).await {
        Ok(result) => println!("Analyzer Toggle OFF: {:?}", result),
        Err(e) => println!("Analyzer Toggle OFF Error: {:?}", e),
    }

    // Test 5: Get Analyzer Status again
    println!("\n=== Testing Analyzer Status (after toggle OFF) ===");
    match client.analyzer_status().await {
        Ok(result) => println!("Analyzer Status: {:?}", result),
        Err(e) => println!("Analyzer Status Error: {:?}", e),
    }

    Ok(())
}
