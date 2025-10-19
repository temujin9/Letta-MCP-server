//! Agent Advanced Operations Tool
//!
//! Consolidated tool for all advanced agent operations using discriminator pattern.
//! This maintains backward compatibility with the Node.js implementation.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use turbomcp::McpError;
use letta_types::{Message, Pagination, StandardResponse};
use crate::client::LettaClient;

/// Agent operation discriminator
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentOperation {
    // CRUD operations
    List,
    Create,
    Get,
    Update,
    Delete,
    ListTools,
    SendMessage,
    Export,
    Import,
    Clone,
    GetConfig,
    BulkDelete,
    // Advanced operations
    Context,
    ResetMessages,
    Summarize,
    Stream,
    AsyncMessage,
    CancelMessage,
    PreviewPayload,
    SearchMessages,
    GetMessage,
    Count,
}

/// Bulk delete filters
#[derive(Debug, Deserialize)]
pub struct BulkDeleteFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_tag_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

/// Search filters for messages
#[derive(Debug, Deserialize)]
pub struct SearchFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Agent advanced request - all parameters are optional except operation
#[derive(Debug, Deserialize)]
pub struct AgentAdvancedRequest {
    /// The operation to perform
    pub operation: AgentOperation,

    // Common parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // Create/Update parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,

    // Pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,

    // Message parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    // Bulk delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<BulkDeleteFilters>,

    // Search parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filters: Option<SearchFilters>,

    // Export/Import
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_data: Option<Value>,

    // Update data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_data: Option<Value>,
}

/// Main handler for agent advanced operations
pub async fn handle_agent_advanced(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<String, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();

    tracing::info!("Executing agent operation: {}", operation_str);

    let response = match request.operation {
        AgentOperation::List => handle_list_agents(client, request).await?,
        AgentOperation::Create => handle_create_agent(client, request).await?,
        AgentOperation::Get => handle_get_agent(client, request).await?,
        AgentOperation::Update => handle_update_agent(client, request).await?,
        AgentOperation::Delete => handle_delete_agent(client, request).await?,
        AgentOperation::SendMessage => handle_send_message(client, request).await?,
        // TODO: Implement remaining operations
        _ => {
            return Err(McpError::invalid_request(format!(
                "Operation not yet implemented: {:?}",
                request.operation
            )))
        }
    };

    Ok(serde_json::to_string_pretty(&response)?)
}

// ===================================================
// Operation Handlers
// ===================================================

async fn handle_list_agents(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let pagination = request.pagination.unwrap_or_default();
    let limit = pagination.limit.unwrap_or(50);
    let offset = pagination.offset.unwrap_or(0);

    let agents = client.list_agents(limit, offset).await?;
    let count = agents.len();

    Ok(StandardResponse::success(
        "list",
        serde_json::to_value(agents)?,
        format!("Retrieved {} agents", count),
    ))
}

async fn handle_create_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let name = request
        .name
        .ok_or_else(|| McpError::invalid_request("name is required for create operation".to_string()))?;

    let agent = client
        .create_agent(
            name,
            request.system,
            request.llm_config,
            request.embedding_config,
            request.tool_ids,
        )
        .await?;

    Ok(StandardResponse::success(
        "create",
        agent,
        "Agent created successfully",
    ))
}

async fn handle_get_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required for get operation".to_string()))?;

    let agent = client.get_agent(&agent_id).await?;

    Ok(StandardResponse::success(
        "get",
        agent,
        "Agent retrieved successfully",
    ))
}

async fn handle_update_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required for update operation".to_string()))?;

    let update_data = request.update_data.ok_or_else(|| {
        McpError::invalid_request("update_data is required for update operation".to_string())
    })?;

    let agent = client.update_agent(agent_id, update_data).await?;

    Ok(StandardResponse::success(
        "update",
        agent,
        "Agent updated successfully",
    ))
}

async fn handle_delete_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required for delete operation".to_string()))?;

    client.delete_agent(&agent_id).await?;

    Ok(StandardResponse::success_no_data(
        "delete",
        format!("Agent {} deleted successfully", agent_id),
    ))
}

async fn handle_send_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for send_message operation".to_string())
    })?;

    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request("messages is required for send_message operation".to_string())
    })?;

    // Convert Message structs to JSON values
    let message_values: Vec<Value> = messages
        .into_iter()
        .map(|m| serde_json::to_value(m))
        .collect::<Result<Vec<_>, _>>()?;

    let response = client
        .send_message(agent_id, message_values, request.stream)
        .await?;

    Ok(StandardResponse::success(
        "send_message",
        response,
        "Message sent successfully",
    ))
}

// TODO: Implement remaining 16 operation handlers:
// - ListTools
// - Export
// - Import
// - Clone
// - GetConfig
// - BulkDelete
// - Context
// - ResetMessages
// - Summarize
// - Stream
// - AsyncMessage
// - CancelMessage
// - PreviewPayload
// - SearchMessages
// - GetMessage
// - Count
