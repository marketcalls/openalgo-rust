//! Analyzer API module for OpenAlgo.

use crate::client::{OpenAlgoClient, OpenAlgoError};
use crate::types::*;
use std::sync::Arc;

/// Analyzer API client
pub struct AnalyzerAPI {
    client: Arc<OpenAlgoClient>,
}

impl AnalyzerAPI {
    /// Create a new Analyzer API client
    pub fn new(client: Arc<OpenAlgoClient>) -> Self {
        Self { client }
    }

    /// Get analyzer status
    pub async fn status(&self) -> Result<AnalyzerStatusResponse, OpenAlgoError> {
        let request = AnalyzerStatusRequest {
            apikey: self.client.api_key.clone(),
        };

        self.client.post("analyzer", &request).await
    }

    /// Toggle analyzer mode
    ///
    /// # Arguments
    ///
    /// * `mode` - Enable (true) or disable (false) analyzer mode
    pub async fn toggle(
        &self,
        mode: bool,
    ) -> Result<AnalyzerToggleResponse, OpenAlgoError> {
        let request = AnalyzerToggleRequest {
            apikey: self.client.api_key.clone(),
            mode,
        };

        self.client.post("analyzer/toggle", &request).await
    }
}
