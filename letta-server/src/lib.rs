//! Letta MCP Server - Rust Implementation
//!
//! This is a Rust implementation of the Letta MCP Server using TurboMCP framework.
//! It maintains backward compatibility with the Node.js implementation while providing
//! better performance and type safety.

use std::sync::Arc;
use turbomcp::prelude::*;
use serde_json::Value;

mod tools;

// Use official Letta SDK
use letta::{auth::AuthConfig, ClientConfig, LettaClient};
use tools::{agent_advanced, file_folder_ops, job_monitor, mcp_ops, memory_unified, source_manager, tool_manager};

/// Main Letta MCP Server
#[derive(Clone)]
pub struct LettaServer {
    client: Arc<LettaClient>,
}

#[turbomcp::server(
    name = "letta-mcp-server",
    version = "2.0.1",
    description = "MCP server for Letta AI - comprehensive API coverage with 7 consolidated tools (97 operations)"
)]
impl LettaServer {
    /// Create a new Letta MCP Server instance
    pub fn new(base_url: String, password: String) -> Result<Self, Box<dyn std::error::Error>> {
        tracing::info!("Initializing Letta MCP Server");
        tracing::info!("Base URL: {}", base_url);

        // Configure the official Letta SDK client
        let config = ClientConfig::new(&base_url)?
            .auth(AuthConfig::bearer(&password));

        let client = LettaClient::new(config)?;
        tracing::info!("Letta SDK client initialized successfully");

        Ok(Self {
            client: Arc::new(client),
        })
    }

    // ========================================
    // CONSOLIDATED TOOL 1: Agent Advanced
    // ========================================

    #[tool(description = "Advanced agent operations hub - Supports 22 operations including CRUD (list, create, get, update, delete), tools (list_tools), messaging (send_message), management (export, import, clone, get_config, bulk_delete), advanced (context, reset_messages, summarize, stream), async (async_message, cancel_message), and utility (preview_payload, search_messages, get_message, count) operations.")]
    async fn letta_agent_advanced(
        &self,
        request: agent_advanced::AgentAdvancedRequest,
    ) -> McpResult<String> {
        // Call handler directly - TurboMCP will auto-detect flattening and use schemars schema
        agent_advanced::handle_agent_advanced(&self.client, request).await
    }

    // ========================================
    // CONSOLIDATED TOOL 2: Memory Unified
    // ========================================

    #[tool(description = "Unified Memory Operations Hub - Provides unified interface for all memory operations. Supports 15 operations including core memory (get_core_memory, update_core_memory), blocks (get_block_by_label, list_blocks, create_block, get_block, update_block, attach_block, detach_block, list_agents_using_block), and archival memory (search_archival, list_passages, create_passage, update_passage, delete_passage).")]
    async fn letta_memory_unified(
        &self,
        request: memory_unified::MemoryUnifiedRequest,
    ) -> McpResult<String> {
        // Call handler directly - TurboMCP will auto-detect flattening and use schemars schema
        memory_unified::handle_memory_unified(&self.client, request).await
    }

    // ========================================
    // CONSOLIDATED TOOL 3: Tool Manager
    // ========================================

    #[tool(description = "Tool Manager Operations Hub - Provides unified interface for tool management operations. Supports 13 operations including CRUD (list, get, create, update, delete, upsert), agent operations (attach, detach, bulk_attach), and advanced operations (generate_from_prompt, generate_schema, run_from_source, add_base_tools).")]
    async fn letta_tool_manager(
        &self,
        request: tool_manager::ToolManagerRequest,
    ) -> McpResult<String> {
        // Call handler directly - TurboMCP will auto-detect flattening and use schemars schema
        tool_manager::handle_tool_manager(&self.client, request).await
    }

    // ========================================
    // CONSOLIDATED TOOL 4: Source Manager
    // ========================================

    #[tool(description = "Source Manager Operations Hub - Provides unified interface for source management operations. Supports 15 operations including CRUD (list, get, create, update, delete, count), agent operations (attach, detach, list_attached), file operations (upload, delete_files, list_files), and folder operations (list_folders, get_folder_contents, list_agents_using).")]
    async fn letta_source_manager(
        &self,
        request: source_manager::SourceManagerRequest,
    ) -> McpResult<String> {
        // Call handler directly - TurboMCP will auto-detect flattening and use schemars schema
        source_manager::handle_source_manager(&self.client, request).await
    }

    // ========================================
    // CONSOLIDATED TOOL 5: Job Monitor
    // ========================================

    #[tool(description = "Job Monitor Operations Hub - Provides unified interface for job monitoring operations. Supports 4 operations: list (all jobs), get (specific job), cancel (job cancellation), and list_active (active jobs only).")]
    async fn letta_job_monitor(
        &self,
        request: job_monitor::JobMonitorRequest,
    ) -> McpResult<String> {
        // Call handler directly - TurboMCP will auto-detect flattening and use schemars schema
        job_monitor::handle_job_monitor(&self.client, request).await
    }

    // ========================================
    // CONSOLIDATED TOOL 6: File/Folder Ops
    // ========================================

    #[tool(description = "File and Folder Management Hub - Provides unified interface for file session management and folder operations. Supports 8 operations including file sessions (list_files, open_file, close_file, close_all_files) and folder operations (list_folders, attach_folder, detach_folder, list_agents_in_folder).")]
    async fn letta_file_folder_ops(
        &self,
        request: file_folder_ops::FileFolderRequest,
    ) -> McpResult<String> {
        // Call handler directly - TurboMCP will auto-detect flattening and use schemars schema
        file_folder_ops::handle_file_folder_ops(&self.client, request).await
    }

    // ========================================
    // CONSOLIDATED TOOL 7: MCP Operations
    // ========================================

    #[tool(description = "MCP Server Operations Hub - Unified tool for complete MCP server lifecycle management. Supports 10 operations: add, update, delete, test, connect, resync (server management) and list_servers, list_tools, register_tool, execute (tool operations).")]
    async fn letta_mcp_ops(
        &self,
        request: mcp_ops::McpOpsRequest,
    ) -> McpResult<String> {
        // Call handler directly - TurboMCP will auto-detect flattening and use schemars schema
        mcp_ops::handle_mcp_ops(&self.client, request).await
    }
}

// Custom HTTP runner implementation with permissive security for development
#[cfg(feature = "http")]
impl LettaServer {
    /// Run HTTP server with custom security configuration
    pub async fn run_http_custom(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        use turbomcp_transport::streamable_http_v2::{StreamableHttpConfigBuilder, run_server};
        use std::sync::Arc;
        use std::time::Duration;

        // Create permissive HTTP config for development
        let config = StreamableHttpConfigBuilder::new()
            .with_bind_address(addr)
            .allow_any_origin(true)  // Allow any origin in development mode
            .allow_localhost(true)
            .with_rate_limit(1_000_000, Duration::from_secs(60))  // Very high limit for development
            .build();

        // Run the HTTP server with custom config
        run_server(config, Arc::new(self.clone())).await?;
        Ok(())
    }
}
