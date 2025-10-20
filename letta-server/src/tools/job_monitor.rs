//! Job Monitor Operations
//!
//! Consolidated tool for job monitoring operations.

use letta::LettaClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use tracing::info;
use turbomcp::McpError;

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum JobOperation {
    List,
    Get,
    Cancel,
    ListActive,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JobMonitorRequest {
    /// The operation to perform (list, get, cancel, list_active)
    pub operation: JobOperation,

    /// Job ID (required for get and cancel operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,

    /// Ignored parameter for MCP client compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_heartbeat: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct JobMonitorResponse {
    pub success: bool,
    pub operation: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
}

pub async fn handle_job_monitor(
    client: &LettaClient,
    request: JobMonitorRequest,
) -> Result<String, McpError> {
    let operation_str = format!("{:?}", request.operation).to_lowercase();
    info!(operation = %operation_str, "Executing job operation");

    let response = match request.operation {
        JobOperation::List => handle_list_jobs(client, request).await?,
        JobOperation::Get => handle_get_job(client, request).await?,
        JobOperation::Cancel => handle_cancel_job(client, request).await?,
        JobOperation::ListActive => handle_list_active_jobs(client, request).await?,
    };

    Ok(serde_json::to_string_pretty(&response)?)
}

async fn handle_list_jobs(client: &LettaClient, _request: JobMonitorRequest) -> Result<JobMonitorResponse, McpError> {
    let jobs = client.jobs().list(None, None, None).await
        .map_err(|e| McpError::internal(format!("Failed to list jobs: {}", e)))?;

    Ok(JobMonitorResponse {
        success: true,
        operation: "list".to_string(),
        message: format!("Found {} jobs", jobs.len()),
        data: Some(serde_json::to_value(&jobs)?),
        count: Some(jobs.len()),
    })
}

async fn handle_get_job(client: &LettaClient, request: JobMonitorRequest) -> Result<JobMonitorResponse, McpError> {
    let job_id = request.job_id.ok_or_else(|| McpError::invalid_request("job_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&job_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid job_id: {}", e)))?;
    
    let job = client.jobs().get(&letta_id).await
        .map_err(|e| McpError::internal(format!("Failed to get job: {}", e)))?;
    
    Ok(JobMonitorResponse {
        success: true,
        operation: "get".to_string(),
        message: "Job retrieved successfully".to_string(),
        data: Some(serde_json::to_value(job)?),
        count: None,
    })
}

async fn handle_cancel_job(client: &LettaClient, request: JobMonitorRequest) -> Result<JobMonitorResponse, McpError> {
    let job_id = request.job_id.ok_or_else(|| McpError::invalid_request("job_id required".to_string()))?;
    let letta_id = letta::types::LettaId::from_str(&job_id)
        .map_err(|e| McpError::invalid_request(format!("Invalid job_id: {}", e)))?;
    
    client.jobs().delete(&letta_id).await
        .map_err(|e| McpError::internal(format!("Failed to cancel job: {}", e)))?;
    
    Ok(JobMonitorResponse {
        success: true,
        operation: "cancel".to_string(),
        message: "Job cancelled successfully".to_string(),
        data: None,
        count: None,
    })
}

async fn handle_list_active_jobs(client: &LettaClient, _request: JobMonitorRequest) -> Result<JobMonitorResponse, McpError> {
    // Use list_active endpoint
    let jobs = client.jobs().list_active(None, None).await
        .map_err(|e| McpError::internal(format!("Failed to list active jobs: {}", e)))?;

    Ok(JobMonitorResponse {
        success: true,
        operation: "list_active".to_string(),
        message: format!("Found {} active jobs", jobs.len()),
        data: Some(serde_json::to_value(&jobs)?),
        count: Some(jobs.len()),
    })
}
