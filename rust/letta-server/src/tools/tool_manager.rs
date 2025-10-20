//! Tool Manager Operations
//!
//! Consolidated tool for tool management operations using discriminator pattern.

use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use tracing::info;
use turbomcp::McpError;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ToolOperation {
    List,
    Get,
    Create,
    Attach,
    BulkAttach,
    Update,
    Delete,
    Upsert,
    Detach,
    GenerateFromPrompt,
    GenerateSchema,
    RunFromSource,
    AddBaseTools,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ToolManagerRequest {
    pub operation: ToolOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_json_schema: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Value>, // For run_from_source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<std::collections::HashMap<String, String>>, // For run_from_source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // For run_from_source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ToolManagerResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
}

pub async fn handle_tool_manager(
    client: &LettaClient,
    request: ToolManagerRequest,
) -> Result<ToolManagerResponse, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing tool operation");

    match request.operation {
        ToolOperation::List => handle_list_tools(client, request).await,
        ToolOperation::Get => handle_get_tool(client, request).await,
        ToolOperation::Create => handle_create_tool(client, request).await,
        ToolOperation::Attach => handle_attach_tool(client, request).await,
        ToolOperation::BulkAttach => handle_bulk_attach(client, request).await,
        ToolOperation::Update => handle_update_tool(client, request).await,
        ToolOperation::Delete => handle_delete_tool(client, request).await,
        ToolOperation::Upsert => handle_upsert_tool(client, request).await,
        ToolOperation::Detach => handle_detach_tool(client, request).await,
        ToolOperation::RunFromSource => handle_run_from_source(client, request).await,
        ToolOperation::AddBaseTools => handle_add_base_tools(client, request).await,
        ToolOperation::GenerateFromPrompt => Err(McpError::internal("generate_from_prompt not available in SDK - requires custom implementation".to_string())),
        ToolOperation::GenerateSchema => Err(McpError::internal("generate_schema not available in SDK - requires custom implementation".to_string())),
    }
}

async fn handle_list_tools(client: &LettaClient, _request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let tools = client.tools().list(None).await
        .map_err(|e| McpError::internal(format!("Failed to list tools: {}", e)))?;
    
    Ok(ToolManagerResponse {
        success: true,
        operation: "list".to_string(),
        message: format!("Found {} tools", tools.len()),
        data: Some(serde_json::to_value(&tools)?),
        count: Some(tools.len()),
    })
}

async fn handle_get_tool(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let tool_id = request.tool_id.ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;
    
    let tool = client.tools().get(&letta_id).await
        .map_err(|e| McpError::internal(format!("Failed to get tool: {}", e)))?;
    
    Ok(ToolManagerResponse {
        success: true,
        operation: "get".to_string(),
        message: "Tool retrieved successfully".to_string(),
        data: Some(serde_json::to_value(tool)?),
        count: None,
    })
}

async fn handle_create_tool(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let source_code = request.source_code.ok_or_else(|| McpError::invalid_request("source_code required".to_string()))?;

    // Parse source_type if provided
    let source_type = request.source_type.and_then(|s| {
        match s.to_lowercase().as_str() {
            "python" => Some(letta::types::tool::SourceType::Python),
            "javascript" => Some(letta::types::tool::SourceType::JavaScript),
            _ => None,
        }
    });

    let create_request = letta::types::tool::CreateToolRequest {
        source_code,
        description: request.description,
        json_schema: request.json_schema,
        args_json_schema: request.args_json_schema,
        source_type,
        tags: request.tags,
        return_char_limit: request.return_char_limit,
        pip_requirements: None,
    };

    let tool = client.tools().create(create_request).await
        .map_err(|e| McpError::internal(format!("Failed to create tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "create".to_string(),
        message: "Tool created successfully".to_string(),
        data: Some(serde_json::to_value(tool)?),
        count: None,
    })
}

async fn handle_attach_tool(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;
    let tool_id = request.tool_id.ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    
    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_tool_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;
    
    let agent_state = client.memory().attach_tool_to_agent(&letta_agent_id, &letta_tool_id).await
        .map_err(|e| McpError::internal(format!("Failed to attach tool: {}", e)))?;
    
    Ok(ToolManagerResponse {
        success: true,
        operation: "attach".to_string(),
        message: "Tool attached successfully".to_string(),
        data: Some(serde_json::to_value(agent_state)?),
        count: None,
    })
}

async fn handle_bulk_attach(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let tool_id = request.tool_id.ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    let agent_ids = request.agent_ids.ok_or_else(|| McpError::invalid_request("agent_ids required".to_string()))?;

    let letta_tool_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    let mut results = Vec::new();
    let mut errors = Vec::new();

    for agent_id in agent_ids {
        match letta::types::LettaId::from_str(&agent_id) {
            Ok(letta_agent_id) => {
                match client.memory().attach_tool_to_agent(&letta_agent_id, &letta_tool_id).await {
                    Ok(agent_state) => {
                        results.push(serde_json::json!({
                            "agent_id": agent_id,
                            "success": true,
                            "data": agent_state
                        }));
                    },
                    Err(e) => {
                        errors.push(serde_json::json!({
                            "agent_id": agent_id,
                            "success": false,
                            "error": e.to_string()
                        }));
                    }
                }
            },
            Err(e) => {
                errors.push(serde_json::json!({
                    "agent_id": agent_id,
                    "success": false,
                    "error": format!("Invalid agent_id: {}", e)
                }));
            }
        }
    }

    Ok(ToolManagerResponse {
        success: errors.is_empty(),
        operation: "bulk_attach".to_string(),
        message: format!("Attached to {} agents, {} errors", results.len(), errors.len()),
        data: Some(serde_json::json!({
            "results": results,
            "errors": errors
        })),
        count: Some(results.len()),
    })
}

async fn handle_update_tool(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let tool_id = request.tool_id.ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    let update_request = letta::types::tool::UpdateToolRequest {
        description: request.description,
        source_code: request.source_code,
        tags: request.tags,
        return_char_limit: request.return_char_limit,
        pip_requirements: None,
        metadata: None,
    };

    let tool = client.tools().update(&letta_id, update_request).await
        .map_err(|e| McpError::internal(format!("Failed to update tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "update".to_string(),
        message: "Tool updated successfully".to_string(),
        data: Some(serde_json::to_value(tool)?),
        count: None,
    })
}

async fn handle_delete_tool(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let tool_id = request.tool_id.ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;
    
    client.tools().delete(&letta_id).await
        .map_err(|e| McpError::internal(format!("Failed to delete tool: {}", e)))?;
    
    Ok(ToolManagerResponse {
        success: true,
        operation: "delete".to_string(),
        message: "Tool deleted successfully".to_string(),
        data: None,
        count: None,
    })
}

async fn handle_upsert_tool(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let source_code = request.source_code.ok_or_else(|| McpError::invalid_request("source_code required".to_string()))?;

    // Parse source_type if provided
    let source_type = request.source_type.and_then(|s| {
        match s.to_lowercase().as_str() {
            "python" => Some(letta::types::tool::SourceType::Python),
            "javascript" => Some(letta::types::tool::SourceType::JavaScript),
            _ => None,
        }
    });

    let upsert_request = letta::types::tool::CreateToolRequest {
        source_code,
        description: request.description,
        json_schema: request.json_schema,
        args_json_schema: request.args_json_schema,
        source_type,
        tags: request.tags,
        return_char_limit: request.return_char_limit,
        pip_requirements: None,
    };

    let tool = client.tools().upsert(upsert_request).await
        .map_err(|e| McpError::internal(format!("Failed to upsert tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "upsert".to_string(),
        message: "Tool upserted successfully".to_string(),
        data: Some(serde_json::to_value(tool)?),
        count: None,
    })
}

async fn handle_detach_tool(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;
    let tool_id = request.tool_id.ok_or_else(|| McpError::invalid_request("tool_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_tool_id = letta::types::LettaId::from_str(&tool_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid tool_id: {}", e)))?;

    let agent_state = client.memory().detach_tool_from_agent(&letta_agent_id, &letta_tool_id).await
        .map_err(|e| McpError::internal(format!("Failed to detach tool: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "detach".to_string(),
        message: "Tool detached successfully".to_string(),
        data: Some(serde_json::to_value(agent_state)?),
        count: None,
    })
}

async fn handle_run_from_source(client: &LettaClient, request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let source_code = request.source_code.ok_or_else(|| McpError::invalid_request("source_code required".to_string()))?;
    let args = request.args.ok_or_else(|| McpError::invalid_request("args required (JSON object)".to_string()))?;

    // Parse source_type if provided
    let source_type = request.source_type.and_then(|s| {
        match s.to_lowercase().as_str() {
            "python" => Some(letta::types::tool::SourceType::Python),
            "javascript" => Some(letta::types::tool::SourceType::JavaScript),
            _ => None,
        }
    });

    let run_request = letta::types::tool::RunToolFromSourceRequest {
        source_code,
        args,
        env_vars: request.env_vars,
        name: request.name,
        source_type,
        args_json_schema: request.args_json_schema,
        json_schema: request.json_schema,
        pip_requirements: None,
    };

    let response = client.tools().run_from_source(run_request).await
        .map_err(|e| McpError::internal(format!("Failed to run tool from source: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "run_from_source".to_string(),
        message: "Tool executed successfully".to_string(),
        data: Some(serde_json::to_value(response)?),
        count: None,
    })
}

async fn handle_add_base_tools(client: &LettaClient, _request: ToolManagerRequest) -> Result<ToolManagerResponse, McpError> {
    let tools = client.tools().upsert_base_tools().await
        .map_err(|e| McpError::internal(format!("Failed to add base tools: {}", e)))?;

    Ok(ToolManagerResponse {
        success: true,
        operation: "add_base_tools".to_string(),
        message: format!("Added {} base tools", tools.len()),
        data: Some(serde_json::to_value(&tools)?),
        count: Some(tools.len()),
    })
}
