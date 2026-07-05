//! Order API module for OpenAlgo.

use crate::client::{OpenAlgoClient, OpenAlgoError};
use crate::types::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Order API client
pub struct OrderAPI {
    client: Arc<OpenAlgoClient>,
}

impl OrderAPI {
    /// Create a new Order API client
    pub fn new(client: Arc<OpenAlgoClient>) -> Self {
        Self { client }
    }

    /// Place an order (simple form)
    ///
    /// * `disclosed_quantity` - Optional disclosed quantity, mirroring Python's `**kwargs`.
    #[allow(clippy::too_many_arguments)]
    pub async fn place_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        pricetype: &str,
        product: &str,
        quantity: &str,
        disclosed_quantity: Option<&str>,
    ) -> Result<OrderResponse, OpenAlgoError> {
        let request = PlaceOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            symbol: symbol.to_string(),
            action: action.to_string(),
            exchange: exchange.to_string(),
            pricetype: pricetype.to_string(),
            product: product.to_string(),
            quantity: quantity.to_string(),
            price: None,
            trigger_price: None,
            disclosed_quantity: disclosed_quantity.map(|s| s.to_string()),
        };

        self.client.post("placeorder", &request).await
    }

    /// Place a limit order with price
    ///
    /// * `disclosed_quantity` - Optional disclosed quantity, mirroring Python's `**kwargs`.
    #[allow(clippy::too_many_arguments)]
    pub async fn place_limit_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        product: &str,
        quantity: &str,
        price: &str,
        disclosed_quantity: Option<&str>,
    ) -> Result<OrderResponse, OpenAlgoError> {
        let request = PlaceOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            symbol: symbol.to_string(),
            action: action.to_string(),
            exchange: exchange.to_string(),
            pricetype: "LIMIT".to_string(),
            product: product.to_string(),
            quantity: quantity.to_string(),
            price: Some(price.to_string()),
            trigger_price: None,
            disclosed_quantity: disclosed_quantity.map(|s| s.to_string()),
        };

        self.client.post("placeorder", &request).await
    }

    /// Place a stop-loss order
    ///
    /// * `disclosed_quantity` - Optional disclosed quantity, mirroring Python's `**kwargs`.
    #[allow(clippy::too_many_arguments)]
    pub async fn place_sl_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        product: &str,
        quantity: &str,
        price: &str,
        trigger_price: &str,
        disclosed_quantity: Option<&str>,
    ) -> Result<OrderResponse, OpenAlgoError> {
        let request = PlaceOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            symbol: symbol.to_string(),
            action: action.to_string(),
            exchange: exchange.to_string(),
            pricetype: "SL".to_string(),
            product: product.to_string(),
            quantity: quantity.to_string(),
            price: Some(price.to_string()),
            trigger_price: Some(trigger_price.to_string()),
            disclosed_quantity: disclosed_quantity.map(|s| s.to_string()),
        };

        self.client.post("placeorder", &request).await
    }

    /// Place a smart order
    pub async fn place_smart_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        pricetype: &str,
        product: &str,
        quantity: &str,
        position_size: &str,
    ) -> Result<OrderResponse, OpenAlgoError> {
        let request = PlaceSmartOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            symbol: symbol.to_string(),
            action: action.to_string(),
            exchange: exchange.to_string(),
            pricetype: pricetype.to_string(),
            product: product.to_string(),
            quantity: quantity.to_string(),
            position_size: position_size.to_string(),
        };

        self.client.post("placesmartorder", &request).await
    }

    /// Place an options order by auto-resolving the symbol from underlying + offset
    ///
    /// Mirrors Python's `optionsorder()`. Note there is no `splitsize` parameter
    /// on this endpoint (that was a mismatched invention in an earlier version of
    /// this SDK) — use [`OrderAPI::split_order`] separately if you need order
    /// splitting.
    ///
    /// * `expiry_date` - Optional; resolvable when `underlying` already embeds an
    ///   expiry (e.g. `NIFTY28OCT25FUT`).
    /// * `strike_int` - Deprecated in Python, kept for parity. Optional.
    /// * `extra` - Extra broker-specific fields forwarded verbatim, mirroring
    ///   Python's `**kwargs` (e.g. `price` for LIMIT orders, `trigger_price` for
    ///   SL/SL-M orders, `disclosed_quantity`).
    #[allow(clippy::too_many_arguments)]
    pub async fn options_order(
        &self,
        strategy: &str,
        underlying: &str,
        exchange: &str,
        offset: &str,
        option_type: &str,
        action: &str,
        quantity: &str,
        pricetype: &str,
        product: &str,
        expiry_date: Option<&str>,
        strike_int: Option<i32>,
        extra: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<OptionsOrderResponse, OpenAlgoError> {
        let request = OptionsOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            underlying: underlying.to_string(),
            exchange: exchange.to_string(),
            expiry_date: expiry_date.map(|s| s.to_string()),
            offset: offset.to_string(),
            option_type: option_type.to_string(),
            action: action.to_string(),
            quantity: quantity.to_string(),
            pricetype: pricetype.to_string(),
            product: product.to_string(),
            strike_int: strike_int.map(|v| v.to_string()),
            extra: extra.unwrap_or_default(),
        };

        self.client.post("optionsorder", &request).await
    }

    /// Place a multi-leg options order
    pub async fn options_multi_order(
        &self,
        strategy: &str,
        underlying: &str,
        exchange: &str,
        expiry_date: &str,
        legs: Vec<OptionsLeg>,
    ) -> Result<OptionsMultiOrderResponse, OpenAlgoError> {
        let request = OptionsMultiOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            underlying: underlying.to_string(),
            exchange: exchange.to_string(),
            expiry_date: Some(expiry_date.to_string()),
            legs,
        };

        self.client.post("optionsmultiorder", &request).await
    }

    /// Place basket orders
    pub async fn basket_order(
        &self,
        strategy: &str,
        orders: Vec<BasketOrderItem>,
    ) -> Result<BasketOrderResponse, OpenAlgoError> {
        let request = BasketOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            orders,
        };

        self.client.post("basketorder", &request).await
    }

    /// Place split orders
    pub async fn split_order(
        &self,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        quantity: i32,
        splitsize: i32,
        pricetype: &str,
        product: &str,
    ) -> Result<SplitOrderResponse, OpenAlgoError> {
        let request = SplitOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            symbol: symbol.to_string(),
            action: action.to_string(),
            exchange: exchange.to_string(),
            quantity,
            splitsize,
            pricetype: pricetype.to_string(),
            product: product.to_string(),
        };

        self.client.post("splitorder", &request).await
    }

    /// Modify an existing order
    ///
    /// * `disclosed_quantity` - Optional disclosed quantity.
    /// * `trigger_price` - Optional trigger price (for SL / SL-M modifications).
    /// * `extra` - Extra broker-specific fields forwarded verbatim, mirroring Python's `**kwargs`.
    #[allow(clippy::too_many_arguments)]
    pub async fn modify_order(
        &self,
        orderid: &str,
        strategy: &str,
        symbol: &str,
        action: &str,
        exchange: &str,
        pricetype: &str,
        product: &str,
        quantity: &str,
        price: &str,
        disclosed_quantity: Option<&str>,
        trigger_price: Option<&str>,
        extra: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<OrderResponse, OpenAlgoError> {
        let request = ModifyOrderRequest {
            apikey: self.client.api_key.clone(),
            orderid: orderid.to_string(),
            strategy: strategy.to_string(),
            symbol: symbol.to_string(),
            action: action.to_string(),
            exchange: exchange.to_string(),
            pricetype: pricetype.to_string(),
            product: product.to_string(),
            quantity: quantity.to_string(),
            price: price.to_string(),
            disclosed_quantity: disclosed_quantity.map(|s| s.to_string()),
            trigger_price: trigger_price.map(|s| s.to_string()),
            extra: extra.unwrap_or_default(),
        };

        self.client.post("modifyorder", &request).await
    }

    /// Cancel an order
    pub async fn cancel_order(
        &self,
        orderid: &str,
        strategy: &str,
    ) -> Result<OrderResponse, OpenAlgoError> {
        let request = CancelOrderRequest {
            apikey: self.client.api_key.clone(),
            orderid: orderid.to_string(),
            strategy: strategy.to_string(),
        };

        self.client.post("cancelorder", &request).await
    }

    /// Cancel all orders
    pub async fn cancel_all_order(
        &self,
        strategy: &str,
    ) -> Result<CancelAllOrderResponse, OpenAlgoError> {
        let request = CancelAllOrderRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
        };

        self.client.post("cancelallorder", &request).await
    }

    /// Close all positions
    pub async fn close_position(
        &self,
        strategy: &str,
    ) -> Result<StatusResponse, OpenAlgoError> {
        let request = ClosePositionRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            product: None,
            symbolgroup: None,
        };

        self.client.post("closeposition", &request).await
    }

    /// Get order status
    pub async fn order_status(
        &self,
        orderid: &str,
        strategy: &str,
    ) -> Result<OrderStatusResponse, OpenAlgoError> {
        let request = OrderStatusRequest {
            apikey: self.client.api_key.clone(),
            orderid: orderid.to_string(),
            strategy: strategy.to_string(),
        };

        self.client.post("orderstatus", &request).await
    }

    /// Get open position
    pub async fn open_position(
        &self,
        strategy: &str,
        symbol: &str,
        exchange: &str,
        product: &str,
    ) -> Result<OpenPositionResponse, OpenAlgoError> {
        let request = OpenPositionRequest {
            apikey: self.client.api_key.clone(),
            strategy: strategy.to_string(),
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            product: product.to_string(),
        };

        self.client.post("openposition", &request).await
    }
}
