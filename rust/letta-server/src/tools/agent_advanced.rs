//! Agent Advanced Operations Tool
//!
//! Consolidated tool for all advanced agent operations using discriminator pattern.
//! This maintains backward compatibility with the Node.js implementation.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use turbomcp::McpError;
use letta_types::{Message, Pagination, StandardResponse};
use letta::LettaClient;

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
    pub tool_ids: Option<Value>,

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

    // Use Letta SDK's cursor-based pagination
    // Note: SDK uses cursor-based pagination (before/after), not offset
    let params = letta::types::ListAgentsParams {
        limit: pagination.limit.map(|l| l as u32),
        ..Default::default()
    };

    // Call SDK method
    let agents = client
        .agents()
        .list(Some(params))
        .await
        .map_err(|e| McpError::internal(format!("Failed to list agents: {}", e)))?;

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

    // Build the agent request with SDK types
    let mut agent_request = letta::types::CreateAgentRequest {
        name: Some(name),
        ..Default::default()
    };

    // Add optional fields if provided
    if let Some(system) = request.system {
        agent_request.system = Some(system);
    }

    // For complex types, parse from JSON Value to SDK types
    if let Some(llm_config_value) = request.llm_config {
        let llm_config: letta::types::LLMConfig = serde_json::from_value(llm_config_value)
            .map_err(|e| McpError::invalid_request(format!("Invalid llm_config: {}", e)))?;
        agent_request.llm_config = Some(llm_config);
    }

    if let Some(embedding_config_value) = request.embedding_config {
        let embedding_config: letta::types::EmbeddingConfig = serde_json::from_value(embedding_config_value)
            .map_err(|e| McpError::invalid_request(format!("Invalid embedding_config: {}", e)))?;
        agent_request.embedding_config = Some(embedding_config);
    }

    if let Some(tool_ids_value) = request.tool_ids {
        let tool_ids: Vec<letta::types::LettaId> = serde_json::from_value(tool_ids_value)
            .map_err(|e| McpError::invalid_request(format!("Invalid tool_ids: {}", e)))?;
        agent_request.tool_ids = Some(tool_ids);
    }

    let agent = client
        .agents()
        .create(agent_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to create agent: {}", e)))?;

    Ok(StandardResponse::success(
        "create",
        serde_json::to_value(agent)?,
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

    // Parse agent_id as LettaId
    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let agent = client
        .agents()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get agent: {}", e)))?;

    Ok(StandardResponse::success(
        "get",
        serde_json::to_value(agent)?,
        "Agent retrieved successfully",
    ))
}

async fn handle_update_agent(
    _client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    // TODO: The Letta SDK v0.1.2 doesn't expose an agent update method.
    // Updates are typically done through specific endpoints (memory, tools, etc.)
    // For now, return a not implemented error
    Err(McpError::internal(
        "Agent update operation not yet implemented in SDK v0.1.2. \
         Please use specific update operations (memory, tools, etc.)".to_string(),
    ))
}

async fn handle_delete_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required for delete operation".to_string()))?;

    // Parse agent_id as LettaId
    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    client
        .agents()
        .delete(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to delete agent: {}", e)))?;

    Ok(StandardResponse::success_no_data(
        "delete",
        format!("Agent {} deleted successfully", letta_id),
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

    // Parse agent_id as LettaId
    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Convert Message structs to MessageCreate (SDK type)
    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    // Build the request (no stream field in CreateMessagesRequest)
    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    // For streaming, we'd use client.messages().create_stream() instead
    // For now, use non-streaming create
    let response = client
        .messages()
        .create(&letta_id, messages_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to send message: {}", e)))?;

    Ok(StandardResponse::success(
        "send_message",
        serde_json::to_value(response)?,
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
