# OpenAlgo Rust SDK

A Rust wrapper for the OpenAlgo API with WebSocket support for real-time market data streaming.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openalgo = "1.0.5"
tokio = { version = "1", features = ["full"] }
```

Or install using cargo:

```bash
cargo add openalgo tokio --features tokio/full
```

## Quick Start

```rust
use openalgo::OpenAlgo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple initialization with just API key
    let client = OpenAlgo::new("your_api_key");

    // Get quotes
    let quotes = client.quotes("RELIANCE", "NSE").await?;
    println!("{:?}", quotes);

    // Place a simple market order
    let order = client.place_order("Strategy1", "RELIANCE", "BUY", "NSE", "MARKET", "MIS", "1").await?;
    println!("{:?}", order);

    Ok(())
}
```

## Configuration

```rust
// Simple initialization (uses default host and WebSocket URL)
let client = OpenAlgo::new("your_api_key");

// Custom configuration
let client = OpenAlgo::with_config(
    "your_api_key",
    "http://127.0.0.1:5000",    // Host
    "v1",                        // API Version
    "ws://127.0.0.1:8765",      // WebSocket URL
);
```

---

# Order API

## Place Order

Place a simple market order.

```rust
let order = client.place_order(
    "Strategy1",      // strategy
    "RELIANCE",       // symbol
    "BUY",            // action (BUY/SELL)
    "NSE",            // exchange (NSE/BSE/NFO/MCX/CDS/BFO)
    "MARKET",         // pricetype (MARKET/LIMIT/SL/SL-M)
    "MIS",            // product (CNC/NRML/MIS)
    "1",              // quantity
).await?;
```

**Response:**
```json
{
    "status": "success",
    "orderid": "1234567890"
}
```

## Place Limit Order

Place a limit order with price.

```rust
let order = client.place_limit_order(
    "Strategy1",      // strategy
    "RELIANCE",       // symbol
    "BUY",            // action
    "NSE",            // exchange
    "MIS",            // product
    "1",              // quantity
    "2500.00",        // price
).await?;
```

## Place Stop-Loss Order

Place a stop-loss order with trigger price.

```rust
let order = client.place_sl_order(
    "Strategy1",      // strategy
    "RELIANCE",       // symbol
    "BUY",            // action
    "NSE",            // exchange
    "MIS",            // product
    "1",              // quantity
    "2500.00",        // price
    "2490.00",        // trigger_price
).await?;
```

## Place Smart Order

Place an order with position sizing logic.

```rust
let order = client.place_smart_order(
    "Strategy1",      // strategy
    "RELIANCE",       // symbol
    "BUY",            // action
    "NSE",            // exchange
    "MARKET",         // pricetype
    "MIS",            // product
    "1",              // quantity
    "5",              // position_size
).await?;
```

## Options Order

Place an options order with automatic strike selection.

```rust
let order = client.options_order(
    "Strategy1",      // strategy
    "NIFTY",          // underlying
    "NFO",            // exchange
    "241226",         // expiry_date (YYMMDD)
    "0",              // offset (0=ATM, 1=OTM1, -1=ITM1)
    "CE",             // option_type (CE/PE)
    "BUY",            // action
    "50",             // quantity
    "MARKET",         // pricetype
    "MIS",            // product
    "50",             // splitsize
).await?;
```

**Response:**
```json
{
    "status": "success",
    "orderid": "1234567890",
    "symbol": "NIFTY24DEC24000CE",
    "exchange": "NFO",
    "offset": "0",
    "option_type": "CE",
    "underlying": "NIFTY",
    "underlying_ltp": 24000.50,
    "mode": "live"
}
```

## Options Multi-Order

Place multi-leg options orders (spreads, straddles, etc.).

```rust
use openalgo::OptionsLeg;

// Bull Call Spread
let legs = vec![
    OptionsLeg::new("0", "CE", "BUY", "50"),   // Buy ATM Call
    OptionsLeg::new("2", "CE", "SELL", "50"),  // Sell OTM Call
];

let order = client.options_multi_order(
    "Strategy1",      // strategy
    "NIFTY",          // underlying
    "NFO",            // exchange
    "241226",         // expiry_date
    legs,
).await?;
```

**Response:**
```json
{
    "status": "success",
    "underlying": "NIFTY",
    "underlying_ltp": 24000.50,
    "results": [
        {"leg": 1, "status": "success", "orderid": "1234567890", "symbol": "NIFTY24DEC24000CE"},
        {"leg": 2, "status": "success", "orderid": "1234567891", "symbol": "NIFTY24DEC24100CE"}
    ]
}
```

## Basket Order

Place multiple orders at once.

```rust
use openalgo::BasketOrderItem;

let orders = vec![
    BasketOrderItem::new("RELIANCE", "NSE", "BUY", 1, "MARKET", "MIS"),
    BasketOrderItem::new("TCS", "NSE", "BUY", 1, "MARKET", "MIS"),
];

let result = client.basket_order("Strategy1", orders).await?;
```

**Response:**
```json
{
    "status": "success",
    "results": [
        {"symbol": "RELIANCE", "status": "success", "orderid": "1234567890"},
        {"symbol": "TCS", "status": "success", "orderid": "1234567891"}
    ]
}
```

## Split Order

Split a large order into smaller chunks.

```rust
let result = client.split_order(
    "Strategy1",      // strategy
    "RELIANCE",       // symbol
    "BUY",            // action
    "NSE",            // exchange
    100,              // total quantity (i32)
    25,               // splitsize (i32)
    "MARKET",         // pricetype
    "MIS",            // product
).await?;
```

## Modify Order

Modify an existing order.

```rust
let result = client.modify_order(
    "1234567890",     // orderid
    "Strategy1",      // strategy
    "RELIANCE",       // symbol
    "BUY",            // action
    "NSE",            // exchange
    "LIMIT",          // pricetype
    "MIS",            // product
    "1",              // quantity
    "2550.00",        // price
).await?;
```

## Cancel Order

Cancel a specific order.

```rust
let result = client.cancel_order("1234567890", "Strategy1").await?;
```

## Cancel All Orders

Cancel all open orders for a strategy.

```rust
let result = client.cancel_all_order("Strategy1").await?;
```

## Close Position

Close all positions for a strategy.

```rust
let result = client.close_position("Strategy1").await?;
```

## Order Status

Get the status of an order.

```rust
let status = client.order_status("1234567890", "Strategy1").await?;
```

## Open Position

Get current open position for a symbol.

```rust
let position = client.open_position("Strategy1", "RELIANCE", "NSE", "MIS").await?;
```

---

# Data API

## Quotes

Get real-time quotes for a symbol.

```rust
let quotes = client.quotes("RELIANCE", "NSE").await?;
```

**Response:**
```json
{
    "status": "success",
    "data": {
        "ltp": 2500.50,
        "open": 2480.00,
        "high": 2510.00,
        "low": 2475.00,
        "prev_close": 2485.00,
        "volume": 1234567,
        "bid": 2500.00,
        "ask": 2500.50,
        "oi": 0
    }
}
```

## Multi Quotes

Get quotes for multiple symbols.

```rust
let quotes = client.multi_quotes(&[
    ("RELIANCE", "NSE"),
    ("TCS", "NSE"),
    ("INFY", "NSE"),
]).await?;
```

## Market Depth

Get order book depth.

```rust
let depth = client.depth("RELIANCE", "NSE").await?;
```

## History

Get historical OHLCV data.

```rust
// Simple form - latest data
let history = client.history("RELIANCE", "NSE", "5m").await?;

// With date range
let history = client.history_range("RELIANCE", "NSE", "5m", "2024-01-01", "2024-01-31").await?;
```

## Intervals

Get available intervals.

```rust
let intervals = client.intervals().await?;
```

## Symbol

Get symbol information.

```rust
let info = client.symbol("RELIANCE", "NSE").await?;
```

## Search

Search for symbols.

```rust
let results = client.search("RELI", "NSE").await?;
```

## Expiry

Get expiry dates.

```rust
let expiries = client.expiry("NIFTY", "NFO", "OPT").await?;
```

## Option Chain

Get option chain data.

```rust
let chain = client.option_chain("NIFTY", "NFO", "241226").await?;
```

## Option Symbol

Get option symbol by offset.

```rust
let symbol = client.option_symbol("NIFTY", "NFO", "241226", "0", "CE").await?;
```

## Synthetic Future

Get synthetic future price.

```rust
let future = client.synthetic_future("NIFTY", "NFO", "241226").await?;
```

## Option Greeks

Get option Greeks.

```rust
let greeks = client.option_greeks(
    "NIFTY24DEC24000CE",
    "NFO",
    6.5,              // interest_rate
    "NIFTY",          // underlying_symbol
    "NSE",            // underlying_exchange
).await?;
```

## Instruments

Get all instruments for an exchange.

```rust
let instruments = client.instruments("NSE").await?;
```

---

# Account API

## Funds

Get account funds.

```rust
let funds = client.funds().await?;
```

**Response:**
```json
{
    "status": "success",
    "data": {
        "availablecash": "100000.00",
        "collateral": "50000.00",
        "m2mrealized": "1000.00",
        "m2munrealized": "-500.00",
        "utiliseddebits": "25000.00"
    }
}
```

## Orderbook

Get all orders.

```rust
let orderbook = client.orderbook().await?;
```

## Tradebook

Get all trades.

```rust
let tradebook = client.tradebook().await?;
```

## Positionbook

Get all positions.

```rust
let positions = client.positionbook().await?;
```

## Holdings

Get holdings.

```rust
let holdings = client.holdings().await?;
```

## Margin

Get margin requirement for positions.

```rust
use openalgo::MarginPosition;

let positions = vec![
    MarginPosition::new("NIFTY24DEC24000CE", "NFO", "BUY", "MIS", "MARKET", "50"),
];
let margin = client.margin(positions).await?;
```

---

# Utilities API

## Holidays

Get market holidays.

```rust
let holidays = client.holidays(2024).await?;
```

## Timings

Get exchange timings for a date.

```rust
let timings = client.timings("2024-12-25").await?;
```

## Telegram

Send a Telegram message.

```rust
let result = client.telegram("username", "Hello from OpenAlgo!").await?;
```

---

# Analyzer API

## Status

Get analyzer status.

```rust
let status = client.analyzer_status().await?;
```

## Toggle

Toggle analyzer mode.

```rust
let result = client.analyzer_toggle(true).await?;
```

---

# WebSocket API

## Connect and Subscribe

```rust
use openalgo::{OpenAlgo, WsInstrument};
use openalgo::websocket::{WsSubscriber, WsData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAlgo::new("your_api_key");
    let ws = client.websocket();

    // Connect
    let (cmd_tx, mut data_rx) = ws.connect().await?;
    let subscriber = WsSubscriber::new(cmd_tx);

    // Define instruments
    let instruments = vec![
        WsInstrument::new("NSE", "RELIANCE"),
        WsInstrument::new("NSE", "TCS"),
    ];

    // Subscribe to LTP
    subscriber.subscribe_ltp(instruments.clone()).await?;

    // Receive data
    while let Some(data) = data_rx.recv().await {
        match data {
            WsData::Ltp(ltp) => {
                println!("LTP: {} - {}",
                    ltp.symbol.unwrap_or_default(),
                    ltp.ltp.unwrap_or_default()
                );
            }
            WsData::Quote(quote) => {
                println!("Quote: {} - LTP: {}, High: {}, Low: {}",
                    quote.symbol.unwrap_or_default(),
                    quote.ltp.unwrap_or_default(),
                    quote.high.unwrap_or_default(),
                    quote.low.unwrap_or_default()
                );
            }
            WsData::Depth(depth) => {
                println!("Depth: {} - Bids: {:?}",
                    depth.symbol.unwrap_or_default(),
                    depth.bids
                );
            }
            _ => {}
        }
    }

    Ok(())
}
```

## Subscription Modes

- **LTP Mode**: Last traded price only
- **Quote Mode**: OHLC + Volume data
- **Depth Mode**: Full order book depth

```rust
// Subscribe to different modes
subscriber.subscribe_ltp(instruments.clone()).await?;
subscriber.subscribe_quote(instruments.clone()).await?;
subscriber.subscribe_depth(instruments.clone()).await?;

// Unsubscribe
subscriber.unsubscribe_ltp(instruments.clone()).await?;

// Disconnect
subscriber.disconnect().await?;
```

---

## Running Examples

```bash
# Set your API key
export OPENALGO_API_KEY=your_api_key

# Run examples
cargo run --example place_order
cargo run --example options_order
cargo run --example quotes
cargo run --example account
cargo run --example websocket
```

## Error Handling

```rust
use openalgo::client::OpenAlgoError;

match client.quotes("RELIANCE", "NSE").await {
    Ok(result) => println!("Success: {:?}", result),
    Err(OpenAlgoError::RequestError(e)) => println!("HTTP Error: {}", e),
    Err(OpenAlgoError::ApiError(msg)) => println!("API Error: {}", msg),
    Err(OpenAlgoError::JsonError(e)) => println!("JSON Error: {}", e),
    Err(e) => println!("Other Error: {}", e),
}
```

## License

MIT License

## Links

- [OpenAlgo Documentation](https://docs.openalgo.in)
- [GitHub Repository](https://github.com/marketcalls/openalgo-rust)
- [crates.io](https://crates.io/crates/openalgo)
