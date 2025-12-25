//! HTTP client for OpenAlgo API.

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

/// Errors that can occur when using the OpenAlgo API
#[derive(Error, Debug)]
pub enum OpenAlgoError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("URL parse error: {0}")]
    UrlError(#[from] url::ParseError),
}

/// HTTP client for making API requests
pub struct OpenAlgoClient {
    pub api_key: String,
    pub host: String,
    pub version: String,
    pub ws_url: String,
    pub http_client: Client,
}

impl OpenAlgoClient {
    /// Create a new OpenAlgo client
    pub fn new(api_key: &str, host: &str, version: &str, ws_url: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            host: host.trim_end_matches('/').to_string(),
            version: version.to_string(),
            ws_url: ws_url.to_string(),
            http_client: Client::new(),
        }
    }

    /// Build the full API URL for an endpoint
    pub fn build_url(&self, endpoint: &str) -> String {
        format!("{}/api/{}/{}", self.host, self.version, endpoint)
    }

    /// Make a POST request to the API
    pub async fn post<T, R>(&self, endpoint: &str, body: &T) -> Result<R, OpenAlgoError>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(endpoint);

        let response = self.http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(OpenAlgoError::ApiError(format!(
                "HTTP {} - {}",
                status, text
            )));
        }

        let result: R = serde_json::from_str(&text)?;
        Ok(result)
    }

    /// Make a GET request to the API
    pub async fn get<R>(&self, endpoint: &str, query_params: &[(&str, &str)]) -> Result<R, OpenAlgoError>
    where
        R: DeserializeOwned,
    {
        let url = self.build_url(endpoint);

        let response = self.http_client
            .get(&url)
            .header("Content-Type", "application/json")
            .query(query_params)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(OpenAlgoError::ApiError(format!(
                "HTTP {} - {}",
                status, text
            )));
        }

        let result: R = serde_json::from_str(&text)?;
        Ok(result)
    }
}
