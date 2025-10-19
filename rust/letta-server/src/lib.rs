//! Letta MCP Server - Rust Implementation
//!
//! This is a Rust implementation of the Letta MCP Server using TurboMCP framework.
//! It maintains backward compatibility with the Node.js implementation while providing
//! better performance and type safety.

use std::sync::Arc;
use turbomcp::prelude::*;

mod client;
mod tools;

use client::LettaClient;
use tools::agent_advanced;

/// Main Letta MCP Server
#[derive(Clone)]
pub struct LettaServer {
    client: Arc<LettaClient>,
}

#[turbomcp::server(
    name = "letta-mcp-server",
    version = "2.0.1",
    description = "MCP server for Letta AI - comprehensive API coverage with 7 consolidated tools (87 operations)"
)]
impl LettaServer {
    /// Create a new Letta MCP Server instance
    pub fn new(base_url: String, password: String) -> Self {
        tracing::info!("Initializing Letta MCP Server");
        tracing::info!("Base URL: {}", base_url);

        Self {
            client: Arc::new(LettaClient::new(base_url, password)),
        }
    }

    // ========================================
    // CONSOLIDATED TOOL 1: Agent Advanced
    // ========================================

    /// Advanced agent operations hub
    ///
    /// Supports 22 operations via discriminator pattern:
    /// - CRUD: list, create, get, update, delete
    /// - Tools: list_tools
    /// - Messaging: send_message
    /// - Management: export, import, clone, get_config, bulk_delete
    /// - Advanced: context, reset_messages, summarize, stream
    /// - Async: async_message, cancel_message
    /// - Utility: preview_payload, search_messages, get_message, count
    #[tool]
    async fn letta_agent_advanced(
        &self,
        request: agent_advanced::AgentAdvancedRequest,
    ) -> McpResult<String> {
        agent_advanced::handle_agent_advanced(&self.client, request).await
    }

    // ========================================
    // CONSOLIDATED TOOL 2: Memory Unified
    // ========================================

    // TODO: Implement letta_memory_unified
    // 15 operations: list_blocks, create_block, get_block, update_block, attach_block,
    //                list_passages, create_passage, update_passage, delete_passage, etc.

    // ========================================
    // CONSOLIDATED TOOL 3: Tool Manager
    // ========================================

    // TODO: Implement letta_tool_manager
    // 13 operations: attach, create, list, get, update, delete, bulk_attach, etc.

    // ========================================
    // CONSOLIDATED TOOL 4: MCP Operations
    // ========================================

    // TODO: Implement letta_mcp_ops
    // 10 operations: list_servers, list_tools, register_tool, etc.

    // ========================================
    // CONSOLIDATED TOOL 5: Source Manager
    // ========================================

    // TODO: Implement letta_source_manager
    // 15 operations: create, list, attach, upload, etc.

    // ========================================
    // CONSOLIDATED TOOL 6: Job Monitor
    // ========================================

    // TODO: Implement letta_job_monitor
    // 4 operations: list, get, cancel, list_active

    // ========================================
    // CONSOLIDATED TOOL 7: File/Folder Ops
    // ========================================

    // TODO: Implement letta_file_folder_ops
    // 8 operations: upload, list_files, create_folder, etc.
}
