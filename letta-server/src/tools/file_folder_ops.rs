//! File and Folder Management Hub
//!
//! Provides unified interface for file session management and folder operations.
//! Implements 8 operations:
//! - File Sessions: list_files, open_file, close_file, close_all_files
//! - Folders: list_folders, attach_folder, detach_folder, list_agents_in_folder

use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use tracing::{error, info};
use turbomcp::McpError;

/// File/folder operation request
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FileFolderRequest {
    /// Operation to perform
    pub operation: String,

    /// Agent ID (required for agent-specific operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// File ID (required for open_file, close_file)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// Folder ID (required for attach/detach/list_agents_in_folder)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,

    /// Ignored parameter (for MCP client compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
}

/// File metadata
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FileMetadata {
    pub id: String,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<String>,
}

/// Folder metadata
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FolderMetadata {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_count: Option<i32>,
}

/// Agent reference
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AgentReference {
    pub id: String,
}

/// File/folder operation response
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FileFolderResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,

    // Common fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,

    // Operation-specific fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileMetadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evicted_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<FolderMetadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_state: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<AgentReference>>,
}

/// Handle letta_file_folder_ops tool requests
pub async fn handle_file_folder_ops(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<String, McpError> {
    let operation = request.operation.as_str();
    info!(operation = %operation, "Executing file/folder operation");

    let response = match operation {
        "list_files" => handle_list_files(client, request).await?,
        "open_file" => handle_open_file(client, request).await?,
        "close_file" => handle_close_file(client, request).await?,
        "close_all_files" => handle_close_all_files(client, request).await?,
        "list_folders" => handle_list_folders(client, request).await?,
        "attach_folder" => handle_attach_folder(client, request).await?,
        "detach_folder" => handle_detach_folder(client, request).await?,
        "list_agents_in_folder" => handle_list_agents_in_folder(client, request).await?,
        _ => {
            error!(operation = %operation, "Unknown operation");
            Err(McpError::invalid_request(format!(
                "Unknown operation: {}",
                operation
            )))?
        }
    };

    Ok(serde_json::to_string_pretty(&response)?)
}

/// List files for an agent
async fn handle_list_files(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<FileFolderResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    // Use SDK to list agent files
    let result = client
        .agents()
        .files(letta_agent_id)
        .list()
        .await
        .map_err(|e| McpError::internal(format!("Failed to list files: {}", e)))?;

    let files: Vec<FileMetadata> = result
        .files
        .into_iter()
        .map(|f| FileMetadata {
            id: f.id.to_string(),
            filename: f.filename.clone(),
            size: Some(f.size),
            mime_type: Some(f.mime_type.clone()),
            is_open: Some(f.is_open),
            opened_at: f.opened_at.clone(),
        })
        .collect();

    let count = files.len();

    Ok(FileFolderResponse {
        success: true,
        operation: "list_files".to_string(),
        message: format!("Found {} files", count),
        agent_id: Some(agent_id),
        files: Some(files),
        file_id: None,
        folder_id: None,
        opened: None,
        evicted_files: None,
        closed: None,
        closed_count: None,
        closed_files: None,
        folders: None,
        attached: None,
        detached: None,
        agent_state: None,
        agent_ids: None,
        agents: None,
    })
}

/// Open a file for an agent
async fn handle_open_file(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<FileFolderResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required".to_string()))?;
    let file_id = request
        .file_id
        .ok_or_else(|| McpError::invalid_request("file_id is required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_file_id = letta::types::LettaId::from_str(&file_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid file_id: {}", e)))?;

    // Use SDK to open file - returns array of evicted file IDs
    let evicted = client
        .agents()
        .files(letta_agent_id)
        .open(&letta_file_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to open file: {}", e)))?;

    Ok(FileFolderResponse {
        success: true,
        operation: "open_file".to_string(),
        message: "File opened successfully".to_string(),
        agent_id: Some(agent_id),
        file_id: Some(file_id),
        opened: Some(true),
        evicted_files: Some(evicted),
        folder_id: None,
        files: None,
        closed: None,
        closed_count: None,
        closed_files: None,
        folders: None,
        attached: None,
        detached: None,
        agent_state: None,
        agent_ids: None,
        agents: None,
    })
}

/// Close a specific file
async fn handle_close_file(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<FileFolderResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required".to_string()))?;
    let file_id = request
        .file_id
        .ok_or_else(|| McpError::invalid_request("file_id is required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_file_id = letta::types::LettaId::from_str(&file_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid file_id: {}", e)))?;

    // Use SDK to close file
    client
        .agents()
        .files(letta_agent_id)
        .close(&letta_file_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to close file: {}", e)))?;

    Ok(FileFolderResponse {
        success: true,
        operation: "close_file".to_string(),
        message: "File closed successfully".to_string(),
        agent_id: Some(agent_id),
        file_id: Some(file_id),
        closed: Some(true),
        folder_id: None,
        files: None,
        opened: None,
        evicted_files: None,
        closed_count: None,
        closed_files: None,
        folders: None,
        attached: None,
        detached: None,
        agent_state: None,
        agent_ids: None,
        agents: None,
    })
}

/// Close all files for an agent
async fn handle_close_all_files(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<FileFolderResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;

    // Use SDK to close all files - returns array of closed file IDs
    let closed = client
        .agents()
        .files(letta_agent_id)
        .close_all()
        .await
        .map_err(|e| McpError::internal(format!("Failed to close all files: {}", e)))?;

    let count = closed.len();

    Ok(FileFolderResponse {
        success: true,
        operation: "close_all_files".to_string(),
        message: format!("Closed {} files", count),
        agent_id: Some(agent_id),
        closed_count: Some(count),
        closed_files: Some(closed),
        file_id: None,
        folder_id: None,
        files: None,
        opened: None,
        evicted_files: None,
        closed: None,
        folders: None,
        attached: None,
        detached: None,
        agent_state: None,
        agent_ids: None,
        agents: None,
    })
}

/// List all folders
async fn handle_list_folders(
    client: &LettaClient,
    _request: FileFolderRequest,
) -> Result<FileFolderResponse, McpError> {
    // Use SDK to list folders
    let result = client
        .folders()
        .list(None)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list folders: {}", e)))?;

    let folders: Vec<FolderMetadata> = result
        .into_iter()
        .map(|f| FolderMetadata {
            id: f.id.to_string(),
            name: f.name.clone(),
            description: f.description.clone(),
            file_count: None, // Not included in SDK response
            agent_count: None, // Not included in SDK response
        })
        .collect();

    let count = folders.len();

    Ok(FileFolderResponse {
        success: true,
        operation: "list_folders".to_string(),
        message: format!("Found {} folders", count),
        folders: Some(folders),
        agent_id: None,
        file_id: None,
        folder_id: None,
        files: None,
        opened: None,
        evicted_files: None,
        closed: None,
        closed_count: None,
        closed_files: None,
        attached: None,
        detached: None,
        agent_state: None,
        agent_ids: None,
        agents: None,
    })
}

/// Attach folder to agent
async fn handle_attach_folder(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<FileFolderResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required".to_string()))?;
    let folder_id = request
        .folder_id
        .ok_or_else(|| McpError::invalid_request("folder_id is required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_folder_id = letta::types::LettaId::from_str(&folder_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid folder_id: {}", e)))?;

    // Use SDK to attach folder - returns AgentState
    let agent_state = client
        .folders()
        .agent(letta_agent_id)
        .attach(&letta_folder_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to attach folder: {}", e)))?;

    Ok(FileFolderResponse {
        success: true,
        operation: "attach_folder".to_string(),
        message: "Folder attached to agent successfully".to_string(),
        agent_id: Some(agent_id),
        folder_id: Some(folder_id),
        attached: Some(true),
        agent_state: Some(serde_json::to_value(&agent_state).unwrap_or(Value::Null)),
        file_id: None,
        files: None,
        opened: None,
        evicted_files: None,
        closed: None,
        closed_count: None,
        closed_files: None,
        folders: None,
        detached: None,
        agent_ids: None,
        agents: None,
    })
}

/// Detach folder from agent
async fn handle_detach_folder(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<FileFolderResponse, McpError> {
    let agent_id = request
        .agent_id
        .ok_or_else(|| McpError::invalid_request("agent_id is required".to_string()))?;
    let folder_id = request
        .folder_id
        .ok_or_else(|| McpError::invalid_request("folder_id is required".to_string()))?;

    let letta_agent_id = letta::types::LettaId::from_str(&agent_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid agent_id: {}", e)))?;
    let letta_folder_id = letta::types::LettaId::from_str(&folder_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid folder_id: {}", e)))?;

    // Use SDK to detach folder - returns AgentState
    let agent_state = client
        .folders()
        .agent(letta_agent_id)
        .detach(&letta_folder_id)
        .await
        .map_err(|e| McpError::internal(format!("Failed to detach folder: {}", e)))?;

    Ok(FileFolderResponse {
        success: true,
        operation: "detach_folder".to_string(),
        message: "Folder detached from agent successfully".to_string(),
        agent_id: Some(agent_id),
        folder_id: Some(folder_id),
        detached: Some(true),
        agent_state: Some(serde_json::to_value(&agent_state).unwrap_or(Value::Null)),
        file_id: None,
        files: None,
        opened: None,
        evicted_files: None,
        closed: None,
        closed_count: None,
        closed_files: None,
        folders: None,
        attached: None,
        agent_ids: None,
        agents: None,
    })
}

/// List agents in a specific folder
async fn handle_list_agents_in_folder(
    client: &LettaClient,
    request: FileFolderRequest,
) -> Result<FileFolderResponse, McpError> {
    let folder_id = request
        .folder_id
        .ok_or_else(|| McpError::invalid_request("folder_id is required".to_string()))?;

    let letta_folder_id = letta::types::LettaId::from_str(&folder_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid folder_id: {}", e)))?;

    // Use SDK to list agents in folder - returns Vec<String>
    let agent_ids = client
        .folders()
        .list_agents(&letta_folder_id)
        .await
        .map_err(|e| {
            McpError::internal(format!("Failed to list agents in folder: {}", e))
        })?;

    let agents: Vec<AgentReference> = agent_ids
        .iter()
        .map(|id| AgentReference { id: id.clone() })
        .collect();

    let count = agent_ids.len();

    Ok(FileFolderResponse {
        success: true,
        operation: "list_agents_in_folder".to_string(),
        message: format!("Found {} agents in folder", count),
        folder_id: Some(folder_id),
        agent_ids: Some(agent_ids),
        agents: Some(agents),
        agent_id: None,
        file_id: None,
        files: None,
        opened: None,
        evicted_files: None,
        closed: None,
        closed_count: None,
        closed_files: None,
        folders: None,
        attached: None,
        detached: None,
        agent_state: None,
    })
}
