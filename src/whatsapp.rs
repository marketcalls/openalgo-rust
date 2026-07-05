//! WhatsApp API module for OpenAlgo.
//!
//! Surface mirror of Python's `WhatsAppAPI.whatsapp()` — send WhatsApp
//! notifications through the OpenAlgo paired device (text, image, or document,
//! to self / a single number / a small broadcast / a linked username).
//!
//! Prerequisites (same as Python):
//! 1. WhatsApp device must be paired from the OpenAlgo web UI's `/whatsapp` page.
//! 2. The bot must be connected (auto-reconnects on server boot).
//! 3. A valid, active OpenAlgo API key.
//!
//! Pairing itself is deliberately not exposed via the API — a leaked API key
//! must not be able to re-pair the device. This module only wraps the public
//! `whatsapp/notify` REST endpoint.

use crate::client::{OpenAlgoClient, OpenAlgoError};
use crate::types::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Recipient selection for a WhatsApp message. Exactly one form is sent to the
/// server, matching Python's precedence: `username` > `phones` > `phone` > self.
#[derive(Debug, Clone, Default)]
pub enum WhatsAppRecipient {
    /// Send to the paired device's own number (the operator). Default.
    #[default]
    SelfDevice,
    /// Single E.164 digit string, e.g. `"919876543210"`.
    Phone(String),
    /// Up to 5 E.164 digit strings (broadcast). Anything beyond 5 is dropped
    /// server-side as a WhatsApp ToS-safety guardrail.
    Phones(Vec<String>),
    /// OpenAlgo username — resolves via the linked-users table on the server.
    Username(String),
}

/// Optional payload fields for [`WhatsAppAPI::whatsapp`].
#[derive(Debug, Clone, Default)]
pub struct WhatsAppOptions {
    /// Plain text body. Max 4096 characters.
    pub message: Option<String>,
    /// Server-local path to an image file. Caption falls back to `message` if unset.
    pub image: Option<String>,
    /// Server-local path to a document (PDF, CSV, ...).
    pub document: Option<String>,
    /// Caption for image / follow-up text for document.
    pub caption: Option<String>,
    /// Override the document's display name on the recipient's device.
    pub filename: Option<String>,
    /// When `true` (the default if unset), block until WhatsApp confirms
    /// delivery and return a per-recipient report. Set to `false` for
    /// fire-and-forget (returns a generic "queued" acknowledgement).
    pub wait_for_delivery: Option<bool>,
    /// Extra fields forwarded verbatim, mirroring Python's `**kwargs`.
    pub extra: HashMap<String, serde_json::Value>,
}

/// WhatsApp API client
pub struct WhatsAppAPI {
    client: Arc<OpenAlgoClient>,
}

impl WhatsAppAPI {
    /// Create a new WhatsApp API client
    pub fn new(client: Arc<OpenAlgoClient>) -> Self {
        Self { client }
    }

    /// Send a WhatsApp message via the OpenAlgo paired device.
    ///
    /// # Example
    /// ```rust,no_run
    /// use openalgo::whatsapp::{WhatsAppRecipient, WhatsAppOptions};
    /// # async fn run(api: &openalgo::whatsapp::WhatsAppAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// // Send to self — simplest case
    /// api.whatsapp(
    ///     WhatsAppRecipient::SelfDevice,
    ///     WhatsAppOptions { message: Some("Build #482 deployed".to_string()), ..Default::default() },
    /// ).await?;
    ///
    /// // Send to a single number
    /// api.whatsapp(
    ///     WhatsAppRecipient::Phone("919876543210".to_string()),
    ///     WhatsAppOptions { message: Some("Are you free for a call?".to_string()), ..Default::default() },
    /// ).await?;
    /// # Ok(()) }
    /// ```
    pub async fn whatsapp(
        &self,
        recipient: WhatsAppRecipient,
        options: WhatsAppOptions,
    ) -> Result<WhatsAppResponse, OpenAlgoError> {
        let mut request = WhatsAppRequest {
            apikey: self.client.api_key.clone(),
            username: None,
            phone: None,
            phones: None,
            self_: None,
            message: options.message,
            image_path: options.image,
            document_path: options.document,
            caption: options.caption,
            filename: options.filename,
            wait_for_delivery: options.wait_for_delivery.unwrap_or(true),
            extra: options.extra,
        };

        match recipient {
            WhatsAppRecipient::Username(username) => request.username = Some(username),
            WhatsAppRecipient::Phones(phones) => request.phones = Some(phones),
            WhatsAppRecipient::Phone(phone) => request.phone = Some(phone),
            WhatsAppRecipient::SelfDevice => request.self_ = Some(true),
        }

        self.client.post("whatsapp/notify", &request).await
    }

    /// Convenience: send a plain text message to the paired device (self).
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn run(api: &openalgo::whatsapp::WhatsAppAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let result = api.whatsapp_message("Build #482 deployed. P&L: +1.2%").await?;
    /// # Ok(()) }
    /// ```
    pub async fn whatsapp_message(&self, message: &str) -> Result<WhatsAppResponse, OpenAlgoError> {
        self.whatsapp(
            WhatsAppRecipient::SelfDevice,
            WhatsAppOptions {
                message: Some(message.to_string()),
                ..Default::default()
            },
        )
        .await
    }

    /// Convenience: send a plain text message to a single phone number.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn run(api: &openalgo::whatsapp::WhatsAppAPI) -> Result<(), Box<dyn std::error::Error>> {
    /// let result = api.whatsapp_to("919876543210", "Stop-loss hit on BANKNIFTY!").await?;
    /// # Ok(()) }
    /// ```
    pub async fn whatsapp_to(&self, to: &str, message: &str) -> Result<WhatsAppResponse, OpenAlgoError> {
        self.whatsapp(
            WhatsAppRecipient::Phone(to.to_string()),
            WhatsAppOptions {
                message: Some(message.to_string()),
                ..Default::default()
            },
        )
        .await
    }
}
