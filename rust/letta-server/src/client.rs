//! HTTP client for communicating with Letta API
//!
//! This module provides a type-safe HTTP client for all Letta API operations.

use reqwest::{Client as HttpClient, Method, StatusCode};
use serde_json::Value;
use std::time::Duration;
use thiserror::Error;
use turbomcp::McpError;

/// Errors that can occur when communicating with the Letta API
#[derive(Error, Debug)]
pub enum LettaError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api { status: StatusCode, message: String },

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Authentication failed")]
    Unauthorized,
}

/// HTTP client for Letta API
pub struct LettaClient {
    client: HttpClient,
    base_url: String,
    password: String,
}

impl LettaClient {
    /// Create a new Letta API client
    pub fn new(base_url: String, password: String) -> Self {
        let client = HttpClient::builder()
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(90))
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url,
            password,
        }
    }

    /// Make a request to the Letta API
    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<Value>,
    ) -> Result<T, LettaError> {
        let url = format!("{}{}", self.base_url, endpoint);

        tracing::debug!("Making {} request to {}", method, endpoint);

        let mut req = self
            .client
            .request(method.clone(), &url)
            .header("Authorization", format!("Bearer {}", self.password))
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            tracing::trace!("Request body: {}", serde_json::to_string_pretty(&body)?);
            req = req.json(&body);
        }

        let response = req.send().await?;
        let status = response.status();

        tracing::debug!("Response status: {}", status);

        match status {
            StatusCode::OK | StatusCode::CREATED => {
                let data = response.json().await?;
                Ok(data)
            }
            StatusCode::NO_CONTENT => {
                // For DELETE operations that return no content
                Ok(serde_json::from_value(Value::Null)?)
            }
            StatusCode::NOT_FOUND => {
                Err(LettaError::NotFound(endpoint.to_string()))
            }
            StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => {
                let text = response.text().await?;
                Err(LettaError::InvalidRequest(text))
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(LettaError::Unauthorized)
            }
            _ => {
                let text = response.text().await.unwrap_or_else(|_| String::from("Unknown error"));
                Err(LettaError::Api {
                    status,
                    message: text,
                })
            }
        }
    }

    // ===================================================
    // AGENT OPERATIONS
    // ===================================================

    /// List all agents with pagination
    pub async fn list_agents(&self, limit: usize, offset: usize) -> Result<Vec<Value>, LettaError> {
        self.request(
            Method::GET,
            &format!("/agents?limit={}&offset={}", limit, offset),
            None,
        )
        .await
    }

    /// Create a new agent
    pub async fn create_agent(
        &self,
        name: String,
        system: Option<String>,
        llm_config: Option<Value>,
        embedding_config: Option<Value>,
        tool_ids: Option<Vec<String>>,
    ) -> Result<Value, LettaError> {
        let mut body = serde_json::json!({ "name": name });
        if let Some(s) = system {
            body["system"] = s.into();
        }
        if let Some(c) = llm_config {
            body["llm_config"] = c;
        }
        if let Some(c) = embedding_config {
            body["embedding_config"] = c;
        }
        if let Some(t) = tool_ids {
            body["tool_ids"] = t.into();
        }

        self.request(Method::POST, "/agents", Some(body)).await
    }

    /// Get agent details
    pub async fn get_agent(&self, agent_id: &str) -> Result<Value, LettaError> {
        self.request(Method::GET, &format!("/agents/{}", agent_id), None)
            .await
    }

    /// Update an agent
    pub async fn update_agent(
        &self,
        agent_id: String,
        update_data: Value,
    ) -> Result<Value, LettaError> {
        self.request(
            Method::PATCH,
            &format!("/agents/{}", agent_id),
            Some(update_data),
        )
        .await
    }

    /// Delete an agent
    pub async fn delete_agent(&self, agent_id: &str) -> Result<(), LettaError> {
        let _: Value = self
            .request(Method::DELETE, &format!("/agents/{}", agent_id), None)
            .await?;
        Ok(())
    }

    /// Send message to agent
    pub async fn send_message(
        &self,
        agent_id: String,
        messages: Vec<Value>,
        stream: Option<bool>,
    ) -> Result<Value, LettaError> {
        let mut body = serde_json::json!({
            "messages": messages
        });
        if let Some(s) = stream {
            body["stream"] = s.into();
        }

        self.request(
            Method::POST,
            &format!("/agents/{}/messages", agent_id),
            Some(body),
        )
        .await
    }

    // ===================================================
    // MEMORY OPERATIONS
    // ===================================================

    /// List memory blocks for an agent
    pub async fn list_memory_blocks(&self, agent_id: &str) -> Result<Vec<Value>, LettaError> {
        self.request(
            Method::GET,
            &format!("/agents/{}/memory/blocks", agent_id),
            None,
        )
        .await
    }

    /// Create a memory block
    pub async fn create_memory_block(
        &self,
        label: String,
        value: String,
    ) -> Result<Value, LettaError> {
        let body = serde_json::json!({
            "label": label,
            "value": value
        });

        self.request(Method::POST, "/blocks", Some(body)).await
    }

    /// Update a memory block
    pub async fn update_memory_block(
        &self,
        block_id: &str,
        value: String,
    ) -> Result<Value, LettaError> {
        let body = serde_json::json!({
            "value": value
        });

        self.request(Method::PATCH, &format!("/blocks/{}", block_id), Some(body))
            .await
    }

    // TODO: Add remaining 80+ methods for all operations
    // This includes:
    // - Remaining agent operations (export, import, clone, etc.)
    // - Tool management operations
    // - MCP operations
    // - Source management
    // - Job monitoring
    // - File/folder operations
}

/// Convert LettaError to McpError for MCP protocol compliance
impl From<LettaError> for McpError {
    fn from(err: LettaError) -> Self {
        match err {
            LettaError::NotFound(msg) => McpError::invalid_request(format!("Not found: {}", msg)),
            LettaError::InvalidRequest(msg) => McpError::invalid_request(msg),
            LettaError::Unauthorized => McpError::invalid_request("Authentication failed".to_string()),
            LettaError::Api { status, message } => {
                McpError::internal(format!("API error ({}): {}", status, message))
            }
            LettaError::Request(e) => McpError::internal(format!("Request failed: {}", e)),
            LettaError::Serialization(e) => {
                McpError::invalid_request(format!("Serialization error: {}", e))
            }
        }
    }
}
