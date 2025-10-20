//! Source Manager Operations
//!
//! Consolidated tool for source management operations.

use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use tracing::info;
use turbomcp::McpError;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceOperation {
    List,
    Get,
    Create,
    Update,
    Delete,
    Attach,
    Detach,
    ListAttached,
    Upload,
    DeleteFiles,
    ListFiles,
    Count,
    ListAgentsUsing,
    ListFolders,
    GetFolderContents,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SourceManagerRequest {
    /// The operation to perform (list, get, create, update, delete, attach, detach, etc.)
    pub operation: SourceOperation,

    /// Source ID (required for get, update, delete, attach, detach operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,

    /// Agent ID (required for attach, detach, list_attached operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// Source name (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Source description (for create/update operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// File ID (for file-related operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// File name (for upload operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,

    /// File data as base64 encoded string (for upload operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,

    /// File content type/MIME type (for upload operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// Maximum number of results to return (for list operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Whether to include file content in response (for list_files operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_content: Option<bool>,

    /// Ignored parameter for MCP client compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SourceManagerResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
}

pub async fn handle_source_manager(
    client: &LettaClient,
    request: SourceManagerRequest,
) -> Result<String, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing source operation");

    let response = match request.operation {
        SourceOperation::List => handle_list_sources(client, request).await?,
        SourceOperation::Get => handle_get_source(client, request).await?,
        SourceOperation::Create => handle_create_source(client, request).await?,
        SourceOperation::Update => handle_update_source(client, request).await?,
        SourceOperation::Delete => handle_delete_source(client, request).await?,
        SourceOperation::Attach => handle_attach_source(client, request).await?,
        SourceOperation::Detach => handle_detach_source(client, request).await?,
        SourceOperation::Count => handle_count_sources(client, request).await?,
        SourceOperation::ListAttached => handle_list_attached(client, request).await?,
        SourceOperation::ListFiles => handle_list_files(client, request).await?,
        SourceOperation::Upload => handle_upload_file(client, request).await?,
        SourceOperation::DeleteFiles => handle_delete_file(client, request).await?,
        SourceOperation::ListFolders => Err(McpError::internal("Folder operations belong in letta_file_folder_ops tool".to_string()))?,
        SourceOperation::GetFolderContents => Err(McpError::internal("Folder operations belong in letta_file_folder_ops tool".to_string()))?,
        SourceOperation::ListAgentsUsing => handle_list_agents_using(client, request).await?,
    };

    Ok(serde_json::to_string_pretty(&response)?)
}

async fn handle_list_sources(client: &LettaClient, _request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let sources = client.sources().list().await
        .map_err(|e| McpError::internal(format!("Failed to list sources: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "list".to_string(),
        message: format!("Found {} sources", sources.len()),
        data: Some(serde_json::to_value(&sources)?),
        count: Some(sources.len()),
    })
}

async fn handle_get_source(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;
    
    let source = client.sources().get(&letta_id).await
        .map_err(|e| McpError::internal(format!("Failed to get source: {}", e)))?;
    
    Ok(SourceManagerResponse {
        success: true,
        operation: "get".to_string(),
        message: "Source retrieved successfully".to_string(),
        data: Some(serde_json::to_value(source)?),
        count: None,
    })
}

async fn handle_create_source(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let name = request.name.ok_or_else(|| McpError::invalid_request("name required".to_string()))?;

    let create_request = if let Some(desc) = request.description {
        letta::types::source::CreateSourceRequest::builder()
            .name(name)
            .description(desc)
            .build()
    } else {
        letta::types::source::CreateSourceRequest::builder()
            .name(name)
            .build()
    };

    let source = client.sources().create(create_request).await
        .map_err(|e| McpError::internal(format!("Failed to create source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "create".to_string(),
        message: "Source created successfully".to_string(),
        data: Some(serde_json::to_value(source)?),
        count: None,
    })
}

async fn handle_update_source(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;
    
    let update_request = letta::types::source::UpdateSourceRequest {
        name: request.name,
        description: request.description,
        ..Default::default()
    };
    
    let source = client.sources().update(&letta_id, update_request).await
        .map_err(|e| McpError::internal(format!("Failed to update source: {}", e)))?;
    
    Ok(SourceManagerResponse {
        success: true,
        operation: "update".to_string(),
        message: "Source updated successfully".to_string(),
        data: Some(serde_json::to_value(source)?),
        count: None,
    })
}

async fn handle_delete_source(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;
    
    client.sources().delete(&letta_id).await
        .map_err(|e| McpError::internal(format!("Failed to delete source: {}", e)))?;
    
    Ok(SourceManagerResponse {
        success: true,
        operation: "delete".to_string(),
        message: "Source deleted successfully".to_string(),
        data: None,
        count: None,
    })
}

async fn handle_attach_source(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_source_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    let agent_state = client.sources().agent_sources(letta_agent_id).attach(&letta_source_id).await
        .map_err(|e| McpError::internal(format!("Failed to attach source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "attach".to_string(),
        message: "Source attached successfully".to_string(),
        data: Some(serde_json::to_value(agent_state)?),
        count: None,
    })
}

async fn handle_detach_source(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_source_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    let agent_state = client.sources().agent_sources(letta_agent_id).detach(&letta_source_id).await
        .map_err(|e| McpError::internal(format!("Failed to detach source: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "detach".to_string(),
        message: "Source detached successfully".to_string(),
        data: Some(serde_json::to_value(agent_state)?),
        count: None,
    })
}

async fn handle_count_sources(client: &LettaClient, _request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let count = client.sources().count().await
        .map_err(|e| McpError::internal(format!("Failed to count sources: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "count".to_string(),
        message: format!("Total sources: {}", count),
        data: Some(serde_json::json!({"count": count})),
        count: Some(count as usize),
    })
}

async fn handle_list_attached(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let agent_id = request.agent_id.ok_or_else(|| McpError::invalid_request("agent_id required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    let sources = client.sources().agent_sources(letta_agent_id).list().await
        .map_err(|e| McpError::internal(format!("Failed to list attached sources: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "list_attached".to_string(),
        message: format!("Found {} attached sources", sources.len()),
        data: Some(serde_json::to_value(&sources)?),
        count: Some(sources.len()),
    })
}

async fn handle_list_files(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    let params = if request.limit.is_some() || request.include_content.is_some() {
        Some(letta::types::source::ListFilesParams {
            limit: request.limit,
            after: None,
            include_content: request.include_content,
        })
    } else {
        None
    };

    let files = client.sources().list_files(&letta_id, params).await
        .map_err(|e| McpError::internal(format!("Failed to list files: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "list_files".to_string(),
        message: format!("Found {} files", files.len()),
        data: Some(serde_json::to_value(&files)?),
        count: Some(files.len()),
    })
}

async fn handle_upload_file(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let file_name = request.file_name.ok_or_else(|| McpError::invalid_request("file_name required".to_string()))?;
    let file_data_b64 = request.file_data.ok_or_else(|| McpError::invalid_request("file_data required (base64 encoded)".to_string()))?;

    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    // Decode base64 file data
    use base64::{Engine as _, engine::general_purpose};
    let file_bytes = general_purpose::STANDARD.decode(&file_data_b64)
        .map_err(|e| McpError::invalid_request(format!("Invalid base64 file_data: {}", e)))?;

    let response = client.sources().upload_file(
        &letta_id,
        file_name.clone(),
        bytes::Bytes::from(file_bytes),
        request.content_type,
    ).await
        .map_err(|e| McpError::internal(format!("Failed to upload file: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "upload".to_string(),
        message: format!("File '{}' uploaded successfully", file_name),
        data: Some(serde_json::to_value(&response)?),
        count: None,
    })
}

async fn handle_delete_file(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let file_id = request.file_id.ok_or_else(|| McpError::invalid_request("file_id required".to_string()))?;

    let letta_source_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;
    let letta_file_id = letta::types::LettaId::from_str(&file_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid file_id: {}", e)))?;

    client.sources().delete_file(&letta_source_id, &letta_file_id).await
        .map_err(|e| McpError::internal(format!("Failed to delete file: {}", e)))?;

    Ok(SourceManagerResponse {
        success: true,
        operation: "delete_files".to_string(),
        message: "File deleted successfully".to_string(),
        data: None,
        count: None,
    })
}

async fn handle_list_agents_using(client: &LettaClient, request: SourceManagerRequest) -> Result<SourceManagerResponse, McpError> {
    let source_id = request.source_id.ok_or_else(|| McpError::invalid_request("source_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&source_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid source_id: {}", e)))?;

    // Get all agents and filter by those using this source
    let agents = client.agents().list(None).await
        .map_err(|e| McpError::internal(format!("Failed to list agents: {}", e)))?;

    // Filter agents that have this source attached
    let mut agents_using = Vec::new();
    for agent in agents {
        // Check if this agent has the source attached
        let sources = client.sources().agent_sources(agent.id.clone()).list().await
            .map_err(|e| McpError::internal(format!("Failed to check agent sources: {}", e)))?;

        for source in sources {
            if let Some(sid) = &source.id {
                if sid == &letta_id {
                    agents_using.push(agent.clone());
                    break;
                }
            }
        }
    }

    Ok(SourceManagerResponse {
        success: true,
        operation: "list_agents_using".to_string(),
        message: format!("Found {} agents using this source", agents_using.len()),
        data: Some(serde_json::to_value(&agents_using)?),
        count: Some(agents_using.len()),
    })
}
