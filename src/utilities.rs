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
    /// Mirrors Python's `holidays(year=None)`: `year` is optional (2020-2050);
    /// when omitted the server defaults to the current year.
    ///
    /// # Arguments
    ///
    /// * `year` - Year to get holidays for. `None` uses the server's default (current year).
    pub async fn holidays(
        &self,
        year: Option<i32>,
    ) -> Result<HolidaysResponse, OpenAlgoError> {
        let request = HolidaysRequest {
            apikey: self.client.api_key.clone(),
            year,
        };

        self.client.post("market/holidays", &request).await
    }

    /// Get exchange timings
    ///
    /// Mirrors Python's `timings(date=None)`: `date` is optional; when omitted
    /// this defaults client-side to today's date (Asia/Kolkata) in `YYYY-MM-DD`
    /// format, same as the Python SDK's `datetime.now()` default.
    ///
    /// # Arguments
    ///
    /// * `date` - Date to get timings for (YYYY-MM-DD format). `None` uses today's date.
    pub async fn timings(
        &self,
        date: Option<&str>,
    ) -> Result<TimingsResponse, OpenAlgoError> {
        let date = match date {
            Some(d) => d.to_string(),
            None => today_ist_date_string(),
        };

        let request = TimingsRequest {
            apikey: self.client.api_key.clone(),
            date,
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

/// Today's date in `YYYY-MM-DD`, approximated in Asia/Kolkata (UTC+5:30) since
/// that is OpenAlgo's operating timezone. Used as the client-side default for
/// [`UtilitiesAPI::timings`] when no date is supplied, mirroring Python's
/// `datetime.now()` default. Implemented without a chrono dependency using
/// Howard Hinnant's `civil_from_days` algorithm.
fn today_ist_date_string() -> String {
    let unix_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let ist_secs = unix_secs + 19_800; // UTC+5:30
    let days = ist_secs.div_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Convert a day count since the Unix epoch (1970-01-01) into a (year, month, day)
/// civil calendar date. Public-domain algorithm by Howard Hinnant:
/// <https://howardhinnant.github.io/date_algorithms.html#civil_from_days>
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
