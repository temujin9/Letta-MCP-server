//! MCP Operations
//!
//! Consolidated tool for MCP server lifecycle management.

use letta::{LettaClient, types::tool::{McpServerConfig, TestMcpServerRequest, UpdateMcpServerRequest}};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use turbomcp::McpError;

// Import JsonValue wrapper from agent_advanced
use super::agent_advanced::JsonValue;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum McpOperation {
    Add,
    Update,
    Delete,
    Test,
    Connect,
    Resync,
    Execute,
    ListServers,
    ListTools,
    RegisterTool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct McpOpsRequest {
    /// The operation to perform (add, update, delete, test, connect, resync, execute, list_servers, list_tools, register_tool)
    pub operation: McpOperation,

    /// MCP server name (required for most operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,

    /// MCP server configuration object (for add/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_config: Option<JsonValue>,

    /// Tool name (for execute and register_tool operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    /// Tool execution arguments (for execute operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_args: Option<JsonValue>,

    /// OAuth configuration object (for add/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_config: Option<JsonValue>,

    /// Pagination settings (for list operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<JsonValue>,

    /// Ignored parameter for MCP client compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct McpOpsResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
}

pub async fn handle_mcp_ops(
    client: &LettaClient,
    request: McpOpsRequest,
) -> Result<String, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing MCP operation");

    let response = match request.operation {
        McpOperation::Add => handle_add_server(client, request).await?,
        McpOperation::Update => handle_update_server(client, request).await?,
        McpOperation::Delete => handle_delete_server(client, request).await?,
        McpOperation::Test => handle_test_server(client, request).await?,
        McpOperation::Connect => handle_connect_server(client, request).await?,
        McpOperation::Resync => handle_resync_server(client, request).await?,
        McpOperation::Execute => handle_execute_tool(client, request).await?,
        McpOperation::ListServers => handle_list_servers(client, request).await?,
        McpOperation::ListTools => handle_list_tools(client, request).await?,
        McpOperation::RegisterTool => handle_register_tool(client, request).await?,
    };

    Ok(serde_json::to_string_pretty(&response)?)
}

async fn handle_add_server(client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_config_json = request.server_config.ok_or_else(|| McpError::invalid_request("server_config required".to_string()))?;

    // Deserialize Value to McpServerConfig
    let server_config: McpServerConfig = serde_json::from_value(server_config_json.0)
        .map_err(|e| McpError::invalid_request(format!("Invalid server_config: {}", e)))?;

    let result = client.tools().add_mcp_server(server_config).await
        .map_err(|e| McpError::internal(format!("Failed to add MCP server: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "add".to_string(),
        message: "MCP server added successfully".to_string(),
        data: Some(serde_json::to_value(&result)?),
        servers: None,
        tools: None,
        server_name: None,
        tool_name: None,
    })
}

async fn handle_update_server(client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_name = request.server_name.ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;
    let server_config_json = request.server_config.ok_or_else(|| McpError::invalid_request("server_config required".to_string()))?;

    // Deserialize Value to UpdateMcpServerRequest
    let update_request: UpdateMcpServerRequest = serde_json::from_value(server_config_json.0)
        .map_err(|e| McpError::invalid_request(format!("Invalid server_config: {}", e)))?;

    let result = client.tools().update_mcp_server(&server_name, update_request).await
        .map_err(|e| McpError::internal(format!("Failed to update MCP server: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "update".to_string(),
        message: "MCP server updated successfully".to_string(),
        data: Some(serde_json::to_value(&result)?),
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: None,
    })
}

async fn handle_delete_server(client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_name = request.server_name.ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;

    let result = client.tools().delete_mcp_server(&server_name).await
        .map_err(|e| McpError::internal(format!("Failed to delete MCP server: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "delete".to_string(),
        message: "MCP server deleted successfully".to_string(),
        data: Some(serde_json::to_value(&result)?),
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: None,
    })
}

async fn handle_test_server(client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_config_json = request.server_config.ok_or_else(|| McpError::invalid_request("server_config required".to_string()))?;

    // Deserialize Value to McpServerConfig for the flattened TestMcpServerRequest
    let config: McpServerConfig = serde_json::from_value(server_config_json.0)
        .map_err(|e| McpError::invalid_request(format!("Invalid server_config: {}", e)))?;

    let test_request = TestMcpServerRequest { config };

    let start_time = std::time::Instant::now();
    let result = client.tools().test_mcp_server(test_request).await
        .map_err(|e| McpError::internal(format!("Failed to test MCP server: {}", e)))?;
    let latency = start_time.elapsed().as_millis() as i64;

    let mut test_result = serde_json::Map::new();
    test_result.insert("connected".to_string(), Value::Bool(true));
    test_result.insert("latency_ms".to_string(), Value::Number(latency.into()));
    if let Value::Object(result_obj) = serde_json::to_value(&result)? {
        for (k, v) in result_obj {
            test_result.insert(k, v);
        }
    }

    Ok(McpOpsResponse {
        success: true,
        operation: "test".to_string(),
        message: "MCP server connection successful".to_string(),
        data: Some(Value::Object(test_result)),
        servers: None,
        tools: None,
        server_name: None,
        tool_name: None,
    })
}

async fn handle_connect_server(_client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_name = request.server_name.ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;

    // TODO: Implement when SDK adds connect_mcp_server support
    // For now, return a placeholder response
    Ok(McpOpsResponse {
        success: false,
        operation: "connect".to_string(),
        message: "Connect operation not yet implemented in Rust SDK".to_string(),
        data: None,
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: None,
    })
}

async fn handle_resync_server(_client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_name = request.server_name.ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;

    // TODO: Implement when SDK adds resync support
    // For now, return a placeholder response
    Ok(McpOpsResponse {
        success: false,
        operation: "resync".to_string(),
        message: "Resync operation not yet implemented in Rust SDK".to_string(),
        data: None,
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: None,
    })
}

async fn handle_execute_tool(_client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_name = request.server_name.ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;
    let tool_name = request.tool_name.ok_or_else(|| McpError::invalid_request("tool_name required".to_string()))?;

    // TODO: Implement when SDK adds tool execution support
    // For now, return a placeholder response
    Ok(McpOpsResponse {
        success: false,
        operation: "execute".to_string(),
        message: "Execute operation not yet implemented in Rust SDK".to_string(),
        data: None,
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: Some(tool_name),
    })
}

async fn handle_list_servers(client: &LettaClient, _request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let result = client.tools().list_mcp_servers().await
        .map_err(|e| McpError::internal(format!("Failed to list MCP servers: {}", e)))?;

    // SDK returns object with server names as keys
    let servers_list: Vec<Value> = if let Value::Object(servers_map) = serde_json::to_value(&result)? {
        servers_map.into_iter().map(|(name, config)| {
            let mut server = serde_json::Map::new();
            server.insert("name".to_string(), Value::String(name));
            if let Value::Object(config_obj) = config {
                for (k, v) in config_obj {
                    server.insert(k, v);
                }
            }
            Value::Object(server)
        }).collect()
    } else {
        vec![]
    };

    let count = servers_list.len();

    Ok(McpOpsResponse {
        success: true,
        operation: "list_servers".to_string(),
        message: format!("Found {} MCP servers", count),
        data: None,
        servers: Some(servers_list),
        tools: None,
        server_name: None,
        tool_name: None,
    })
}

async fn handle_list_tools(client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_name = request.server_name.ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;

    let result = client.tools().list_mcp_tools_by_server(&server_name).await
        .map_err(|e| McpError::internal(format!("Failed to list MCP tools: {}", e)))?;

    let tools_list: Vec<Value> = if let Value::Array(arr) = serde_json::to_value(&result)? {
        arr.into_iter().map(|tool| {
            if let Value::Object(mut tool_obj) = tool {
                let mut simplified = serde_json::Map::new();
                if let Some(name) = tool_obj.remove("name") {
                    simplified.insert("name".to_string(), name);
                }
                if let Some(description) = tool_obj.remove("description") {
                    simplified.insert("description".to_string(), description);
                }
                if let Some(schema) = tool_obj.remove("schema").or_else(|| tool_obj.remove("inputSchema")) {
                    simplified.insert("schema".to_string(), schema);
                }
                Value::Object(simplified)
            } else {
                tool
            }
        }).collect()
    } else if let Value::Object(obj) = serde_json::to_value(&result)? {
        if let Some(Value::Array(tools)) = obj.get("tools") {
            tools.clone()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let count = tools_list.len();

    Ok(McpOpsResponse {
        success: true,
        operation: "list_tools".to_string(),
        message: format!("Found {} tools on server {}", count, server_name),
        data: None,
        servers: None,
        tools: Some(tools_list),
        server_name: Some(server_name),
        tool_name: None,
    })
}

async fn handle_register_tool(client: &LettaClient, request: McpOpsRequest) -> Result<McpOpsResponse, McpError> {
    let server_name = request.server_name.ok_or_else(|| McpError::invalid_request("server_name required".to_string()))?;
    let tool_name = request.tool_name.ok_or_else(|| McpError::invalid_request("tool_name required".to_string()))?;

    let result = client.tools().add_mcp_tool(&server_name, &tool_name).await
        .map_err(|e| McpError::internal(format!("Failed to register MCP tool: {}", e)))?;

    Ok(McpOpsResponse {
        success: true,
        operation: "register_tool".to_string(),
        message: format!("Tool {} from {} registered successfully in Letta", tool_name, server_name),
        data: Some(serde_json::to_value(&result)?),
        servers: None,
        tools: None,
        server_name: Some(server_name),
        tool_name: Some(tool_name),
    })
}
