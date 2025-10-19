//! Agent Advanced Operations Tool
//!
//! Consolidated tool for all advanced agent operations using discriminator pattern.
//! This maintains backward compatibility with the Node.js implementation.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use turbomcp::McpError;
use turbomcp_macros::FlattenTool;
use letta_types::{Message, Pagination, StandardResponse};
use letta::LettaClient;

/// Agent operation discriminator
#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
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
#[derive(Debug, Deserialize, schemars::JsonSchema, FlattenTool)]
pub struct BulkDeleteFilters {
    /// Filter agents by name pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name_filter: Option<String>,

    /// Filter agents by tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_tag_filter: Option<String>,

    /// Specific agent IDs to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

/// Search filters for messages
#[derive(Debug, Deserialize, schemars::JsonSchema, FlattenTool)]
pub struct SearchFilters {
    /// Filter messages after this date (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    /// Filter messages before this date (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    /// Filter messages by role (user, assistant, system)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Agent advanced request - all parameters are optional except operation
#[derive(Debug, Deserialize, schemars::JsonSchema, FlattenTool)]
pub struct AgentAdvancedRequest {
    /// The operation to perform (list, create, get, update, delete, send_message, etc.)
    pub operation: AgentOperation,

    /// Agent ID (required for get, update, delete, and message operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// Agent name (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Agent description (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// System prompt for the agent (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// LLM configuration object (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_config: Option<Value>,

    /// Embedding model configuration (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<Value>,

    /// Tool IDs to attach to agent (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Value>,

    /// Pagination settings (for list operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,

    /// Messages to send to agent (for send_message operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,

    /// Enable streaming response (for send_message operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Filters for bulk delete operation (agent_name_filter, agent_tag_filter, agent_ids)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<BulkDeleteFilters>,

    /// Search query text (for search_messages operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Search filters (for search_messages operation: start_date, end_date, role)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filters: Option<SearchFilters>,

    /// Agent export data (for import operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_data: Option<Value>,

    /// Update data object (for update operation)
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
        AgentOperation::ListTools => handle_list_tools(client, request).await?,
        AgentOperation::Export => handle_export_agent(client, request).await?,
        AgentOperation::Import => handle_import_agent(client, request).await?,
        AgentOperation::Clone => handle_clone_agent(client, request).await?,
        AgentOperation::GetConfig => handle_get_config(client, request).await?,
        AgentOperation::BulkDelete => handle_bulk_delete(client, request).await?,
        AgentOperation::Context => handle_get_context(client, request).await?,
        AgentOperation::ResetMessages => handle_reset_messages(client, request).await?,
        AgentOperation::Summarize => handle_summarize(client, request).await?,
        AgentOperation::Stream => handle_stream(client, request).await?,
        AgentOperation::AsyncMessage => handle_async_message(client, request).await?,
        AgentOperation::CancelMessage => handle_cancel_message(client, request).await?,
        AgentOperation::PreviewPayload => handle_preview_payload(client, request).await?,
        AgentOperation::SearchMessages => handle_search_messages(client, request).await?,
        AgentOperation::GetMessage => handle_get_message(client, request).await?,
        AgentOperation::Count => handle_count(client, request).await?,
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

async fn handle_list_tools(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for list_tools operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let tools = client
        .memory()
        .list_agent_tools(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list agent tools: {}", e)))?;

    Ok(StandardResponse::success(
        "list_tools",
        serde_json::to_value(&tools)?,
        format!("Found {} tools", tools.len()),
    ))
}

async fn handle_export_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for export operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let export_json = client
        .agents()
        .export_file(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to export agent: {}", e)))?;

    Ok(StandardResponse::success(
        "export",
        serde_json::json!({ "export_data": export_json }),
        "Agent exported successfully",
    ))
}

async fn handle_import_agent(
    _client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    // Import requires file upload which is not directly supported in MCP tools
    // Would need special handling with multipart form data
    Err(McpError::internal(
        "Import operation not yet implemented - requires file upload support".to_string(),
    ))
}

async fn handle_clone_agent(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for clone operation".to_string())
    })?;
    let new_name = request.name.ok_or_else(|| {
        McpError::invalid_request("name is required for clone operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Get source agent
    let source_agent = client
        .agents()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get source agent: {}", e)))?;

    // Create cloned agent with new name
    let clone_request = letta::types::CreateAgentRequest {
        name: Some(new_name.clone()),
        description: source_agent.description.clone(),
        system: source_agent.system.clone(),
        llm_config: source_agent.llm_config.clone(),
        embedding_config: source_agent.embedding_config.clone(),
        ..Default::default()
    };

    let new_agent = client
        .agents()
        .create(clone_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to create cloned agent: {}", e)))?;

    Ok(StandardResponse::success(
        "clone",
        serde_json::json!({
            "source_agent_id": agent_id,
            "new_agent_id": new_agent.id.to_string(),
            "new_agent_name": new_name
        }),
        "Agent cloned successfully",
    ))
}

async fn handle_get_config(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get_config operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let agent = client
        .agents()
        .get(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get agent: {}", e)))?;

    // Get agent tools (may fail if not accessible)
    let tools = client.memory().list_agent_tools(&letta_id).await.ok();

    Ok(StandardResponse::success(
        "get_config",
        serde_json::json!({
            "name": agent.name,
            "description": agent.description,
            "system": agent.system,
            "llm_config": agent.llm_config,
            "embedding_config": agent.embedding_config,
            "tools": tools.unwrap_or_default(),
            "created_at": agent.created_at,
        }),
        "Agent configuration retrieved successfully",
    ))
}

async fn handle_bulk_delete(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let filters = request.filters.ok_or_else(|| {
        McpError::invalid_request("filters are required for bulk_delete operation".to_string())
    })?;

    // List all agents
    let agents = client
        .agents()
        .list(None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list agents: {}", e)))?;

    // Filter agents based on criteria
    let mut to_delete: Vec<letta::types::LettaId> = Vec::new();

    for agent in agents {
        let mut should_delete = false;

        if let Some(ref name_filter) = filters.agent_name_filter {
            if agent.name.contains(name_filter) {
                should_delete = true;
            }
        }

        if let Some(ref ids) = filters.agent_ids {
            if ids.contains(&agent.id.to_string()) {
                should_delete = true;
            }
        }

        if should_delete {
            to_delete.push(agent.id);
        }
    }

    // Delete each agent
    let mut deleted_count = 0;
    for agent_id in &to_delete {
        if client.agents().delete(agent_id).await.is_ok() {
            deleted_count += 1;
        }
    }

    Ok(StandardResponse::success(
        "bulk_delete",
        serde_json::json!({
            "deleted_count": deleted_count,
            "failed_count": to_delete.len() - deleted_count
        }),
        format!("Deleted {} agents", deleted_count),
    ))
}

async fn handle_get_context(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for context operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    let context = client
        .agents()
        .get_context(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to get context: {}", e)))?;

    Ok(StandardResponse::success(
        "context",
        context,
        "Context retrieved successfully",
    ))
}

async fn handle_reset_messages(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for reset_messages operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    client
        .agents()
        .reset_messages(&letta_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to reset messages: {}", e)))?;

    Ok(StandardResponse::success_no_data(
        "reset_messages",
        "Messages reset successfully",
    ))
}

async fn handle_summarize(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for summarize operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Default to 10 max messages if not specified
    let max_message_length = 10u32;

    let agent_state = client
        .agents()
        .summarize_agent_conversation(&letta_id, max_message_length)
        .await
        .map_err(|e| McpError::internal(format!("Failed to summarize conversation: {}", e)))?;

    Ok(StandardResponse::success(
        "summarize",
        serde_json::to_value(agent_state)?,
        "Conversation summarized successfully",
    ))
}

async fn handle_stream(
    _client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    // Streaming requires special handling and is not directly compatible with MCP tool responses
    Err(McpError::internal(
        "Stream operation not supported in MCP tool context - use async_message instead"
            .to_string(),
    ))
}

async fn handle_async_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for async_message operation".to_string())
    })?;
    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request("messages are required for async_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Convert to MessageCreate types
    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    let run_id = client
        .messages()
        .create_async(&letta_id, messages_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to create async message: {}", e)))?;

    Ok(StandardResponse::success(
        "async_message",
        serde_json::json!({ "run_id": run_id }),
        "Async message created successfully",
    ))
}

async fn handle_cancel_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for cancel_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Note: SDK cancel takes Option<CancelAgentRunRequest>
    // For now, pass None to cancel the most recent run
    // TODO: Add run_id to request structure to cancel specific runs
    client
        .messages()
        .cancel(&letta_id, None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to cancel message: {}", e)))?;

    Ok(StandardResponse::success_no_data(
        "cancel_message",
        "Message cancelled successfully",
    ))
}

async fn handle_preview_payload(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request(
            "agent_id is required for preview_payload operation".to_string(),
        )
    })?;
    let messages = request.messages.ok_or_else(|| {
        McpError::invalid_request(
            "messages are required for preview_payload operation".to_string(),
        )
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // Convert to MessageCreate types
    let message_creates: Vec<letta::types::MessageCreate> = messages
        .into_iter()
        .map(|m| letta::types::MessageCreate::user(&m.content))
        .collect();

    let messages_request = letta::types::CreateMessagesRequest {
        messages: message_creates,
        ..Default::default()
    };

    let preview = client
        .messages()
        .preview(&letta_id, messages_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to preview payload: {}", e)))?;

    Ok(StandardResponse::success(
        "preview_payload",
        serde_json::to_value(preview)?,
        "Payload preview generated successfully",
    ))
}

async fn handle_search_messages(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let query = request.query.ok_or_else(|| {
        McpError::invalid_request("query is required for search_messages operation".to_string())
    })?;

    let search_request = letta::types::MessageSearchRequest {
        query: Some(query),
        ..Default::default()
    };

    let results = client
        .messages()
        .search(search_request)
        .await
        .map_err(|e| McpError::internal(format!("Failed to search messages: {}", e)))?;

    Ok(StandardResponse::success(
        "search_messages",
        serde_json::to_value(&results)?,
        format!("Found {} messages", results.len()),
    ))
}

async fn handle_get_message(
    client: &LettaClient,
    request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| {
        McpError::invalid_request("agent_id is required for get_message operation".to_string())
    })?;

    let letta_id: letta::types::LettaId = agent_id
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id format: {}", e)))?;

    // List all messages and find the one we want
    // Note: SDK doesn't have a direct get_message endpoint, so we list and filter
    let messages = client
        .messages()
        .list(&letta_id, None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list messages: {}", e)))?;

    Ok(StandardResponse::success(
        "get_message",
        serde_json::to_value(&messages)?,
        format!("Retrieved {} messages (filter client-side)", messages.len()),
    ))
}

async fn handle_count(
    client: &LettaClient,
    _request: AgentAdvancedRequest,
) -> Result<StandardResponse, McpError> {
    let count = client
        .agents()
        .count()
        .await
        .map_err(|e| McpError::internal(format!("Failed to count agents: {}", e)))?;

    Ok(StandardResponse::success(
        "count",
        serde_json::json!({ "count": count }),
        format!("Total agents: {}", count),
    ))
}
