//! Utilities API module for OpenAlgo.
//!
//! This module contains utility functions like holidays, timings, and telegram messaging.

use crate::client::{OpenAlgoClient, OpenAlgoError};
use crate::types::*;
use std::sync::Arc;

/// Utilities API client
pub struct UtilitiesAPI {
    client: Arc<OpenAlgoClient>,
}

impl UtilitiesAPI {
    /// Create a new Utilities API client
    pub fn new(client: Arc<OpenAlgoClient>) -> Self {
        Self { client }
    }

    /// Get market holidays
    ///
    /// # Arguments
    ///
    /// * `year` - Year to get holidays for
    pub async fn holidays(
        &self,
        year: i32,
    ) -> Result<HolidaysResponse, OpenAlgoError> {
        let request = HolidaysRequest {
            apikey: self.client.api_key.clone(),
            year,
        };

        self.client.post("market/holidays", &request).await
    }

    /// Get exchange timings
    ///
    /// # Arguments
    ///
    /// * `date` - Date to get timings for (YYYY-MM-DD format)
    pub async fn timings(
        &self,
        date: &str,
    ) -> Result<TimingsResponse, OpenAlgoError> {
        let request = TimingsRequest {
            apikey: self.client.api_key.clone(),
            date: date.to_string(),
        };

        self.client.post("market/timings", &request).await
    }

    /// Send Telegram message with default priority (5)
    ///
    /// # Arguments
    ///
    /// * `username` - Telegram username
    /// * `message` - Message to send
    pub async fn telegram(
        &self,
        username: &str,
        message: &str,
    ) -> Result<TelegramResponse, OpenAlgoError> {
        self.telegram_priority(username, message, 5).await
    }

    /// Send Telegram message with custom priority
    ///
    /// # Arguments
    ///
    /// * `username` - Telegram username
    /// * `message` - Message to send
    /// * `priority` - Message priority (1-10)
    pub async fn telegram_priority(
        &self,
        username: &str,
        message: &str,
        priority: i32,
    ) -> Result<TelegramResponse, OpenAlgoError> {
        let request = TelegramRequest {
            apikey: self.client.api_key.clone(),
            username: username.to_string(),
            message: message.to_string(),
            priority: Some(priority),
        };

        self.client.post("telegram/notify", &request).await
    }
}
