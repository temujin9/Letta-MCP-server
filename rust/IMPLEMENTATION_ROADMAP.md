# Rust MCP Server Implementation Roadmap

**Status:** SDK Complete (91% coverage) → Now Implementing Tool Handlers
**Date:** 2025-10-18
**Goal:** Port all 7 consolidated MCP tools from Node.js to Rust using TurboMCP

---

## Executive Summary

**Current State:**
- ✅ **SDK Enhanced:** 79/87 endpoints covered (91%)
- ✅ **TurboMCP Framework:** Integrated and configured
- ⚠️ **Tool Handlers:** 1/7 tools partially implemented
- ❌ **HTTP Transport:** Not yet ported to TurboMCP

**Remaining Work:**
- 6.5 tools to implement (87 operations total)
- HTTP transport layer for production
- End-to-end testing and validation

---

## SDK Coverage Status

### ✅ Complete Categories (100%)
1. **File Sessions** (4/4) - Agent file management with LRU
2. **Folder Operations** (11/11) - CRUD, uploads, agent relationships

### ✅ Excellent Categories (93%+)
3. **Memory Operations** (14/15) - Core memory, blocks, passages
4. **Source Operations** (13/15) - CRUD, files, agent attachments
5. **Message Operations** (7/8) - Create, stream, cancel, preview, search

### ⚠️ Good Categories (64%)
6. **Agent Operations** (14/22) - CRUD, tools, messages, context

### ❌ Not Applicable
7. **MCP Operations** (0/10) - Custom Node.js implementation
8. **Tool Operations** (~10/13) - Needs verification
9. **Job Operations** (~3/4) - Needs verification

---

## Tool Implementation Matrix

| Tool | Operations | Node.js LOC | Status | SDK Support | Priority |
|------|-----------|-------------|--------|-------------|----------|
| **letta_agent_advanced** | 22 | 850 | 🟡 Partial (6/22) | ✅ 14/22 | 🔴 **High** |
| **letta_file_folder_ops** | 8 | 320 | ❌ Not Started | ✅ 8/8 | 🔴 **High** |
| **letta_memory_unified** | 15 | 580 | ❌ Not Started | ✅ 14/15 | 🟠 **Medium** |
| **letta_tool_manager** | 13 | 490 | ❌ Not Started | ✅ ~10/13 | 🟠 **Medium** |
| **letta_source_manager** | 15 | 560 | ❌ Not Started | ✅ 13/15 | 🟢 **Low** |
| **letta_job_monitor** | 4 | 180 | ❌ Not Started | ✅ ~3/4 | 🟢 **Low** |
| **letta_mcp_ops** | 10 | 420 | ❌ Not Started | ❌ Custom | ⚪ **Deferred** |
| **TOTAL** | **87** | **3,400** | **6.9%** | **85%** | - |

---

## Detailed Implementation Plan

### Phase 1: Critical Tools (Week 1) 🔴

#### 1.1 Complete `letta_agent_advanced` (16 operations remaining)

**Already Implemented (6):**
- ✅ create_agent
- ✅ get_agent
- ✅ list_agents
- ✅ delete_agent
- ✅ attach_tools
- ✅ detach_tools

**Need to Implement (16):**

**Tier 1 - SDK Supported (8):**
```rust
// Using new SDK methods
AgentOperation::Update => {
    client.agents().update(&agent_id, request).await?
}

AgentOperation::GetContext => {
    client.agents().get_context(&agent_id).await?
}

AgentOperation::ResetMessages => {
    client.agents().reset_messages(&agent_id).await?
}

AgentOperation::AttachSource => {
    client.agents().sources(agent_id).attach(&source_id).await?
}

AgentOperation::DetachSource => {
    client.agents().sources(agent_id).detach(&source_id).await?
}

AgentOperation::GetMemory => {
    client.agents().memory(&agent_id).await?
}

AgentOperation::UpdateMemory => {
    client.agents().memory().update(&agent_id, request).await?
}

AgentOperation::ListMemoryBlocks => {
    client.agents().blocks(&agent_id).list().await?
}
```

**Tier 2 - Needs Investigation (8):**
```rust
// Check if SDK has these endpoints
AgentOperation::Count           // GET /v1/agents/count
AgentOperation::ListSources     // GET /v1/agents/{id}/sources
AgentOperation::SendMessage     // POST /v1/agents/{id}/messages
AgentOperation::StreamMessage   // POST /v1/agents/{id}/messages/stream
AgentOperation::GetMessages     // GET /v1/agents/{id}/messages
AgentOperation::Export          // GET /v1/agents/{id}/export
AgentOperation::Import          // POST /v1/agents/import
AgentOperation::BulkDelete      // DELETE /v1/agents (with filters)
```

**Estimated Effort:** 2-3 days
**Complexity:** Medium (most have SDK support)
**Files:** `letta-server/src/tools/agent_advanced.rs`

---

#### 1.2 Implement `letta_file_folder_ops` (8 operations)

**All SDK Supported ✅**

```rust
// File Session Operations (4)
match operation {
    "list_files" => {
        let files = client.agents().files(agent_id).list().await?;
        // Return paginated file list with open status
    }

    "open_file" => {
        let evicted = client.agents().files(agent_id).open(&file_id).await?;
        // Returns array of evicted file IDs (LRU behavior)
    }

    "close_file" => {
        client.agents().files(agent_id).close(&file_id).await?;
        // Success response
    }

    "close_all_files" => {
        let closed = client.agents().files(agent_id).close_all().await?;
        // Returns array of closed file IDs
    }

    // Folder Operations (4)
    "list_folders" => {
        let folders = client.folders().list(None).await?;
        // Return all folders in organization
    }

    "attach_folder" => {
        let state = client.folders().agent(agent_id).attach(&folder_id).await?;
        // Returns updated agent state
    }

    "detach_folder" => {
        let state = client.folders().agent(agent_id).detach(&folder_id).await?;
        // Returns updated agent state
    }

    "list_agents_in_folder" => {
        let agents = client.folders().list_agents(&folder_id).await?;
        // Returns array of agent IDs
    }
}
```

**Estimated Effort:** 1 day
**Complexity:** Low (all SDK methods ready)
**Files:** `letta-server/src/tools/file_folder_ops.rs` (NEW)

---

### Phase 2: Memory & Tools (Week 2) 🟠

#### 2.1 Implement `letta_memory_unified` (15 operations)

**SDK Coverage: 14/15 (93%)**

**Core Memory (2):**
```rust
"get_core_memory" => {
    client.agents().memory(&agent_id).await?
}

"update_core_memory" => {
    client.agents().memory().update(&agent_id, request).await?
}
```

**Memory Blocks (6):**
```rust
"list_blocks" => {
    client.agents().blocks(&agent_id).list().await?
}

"create_block" => {
    client.blocks().create(request).await?
}

"get_block" => {
    client.blocks().get(&block_id).await?
}

"update_block" => {
    client.blocks().update(&block_id, request).await?
}

"attach_block" => {
    client.agents().blocks(&agent_id).attach(&block_id).await?
}

"detach_block" => {
    client.agents().blocks(&agent_id).detach(&block_id).await?
}
```

**Archival/Passages (7):**
```rust
"search_archival" => {
    client.agents().passages(&agent_id).search(request).await?
}

"list_passages" => {
    client.agents().passages(&agent_id).list(params).await?
}

"create_passage" => {
    client.agents().passages(&agent_id).create(request).await?
}

"update_passage" => {
    client.agents().passages(&agent_id).update(&passage_id, request).await?
}

"delete_passage" => {
    client.agents().passages(&agent_id).delete(&passage_id).await?
}

"get_passage" => {
    client.agents().passages(&agent_id).get(&passage_id).await?
}

// Missing: bulk operations (may need custom implementation)
"bulk_create_passages" => {
    // Loop and create individually or use batch API if available
}
```

**Estimated Effort:** 2 days
**Complexity:** Medium (needs passage type handling)
**Files:** `letta-server/src/tools/memory_unified.rs` (NEW)

---

#### 2.2 Implement `letta_tool_manager` (13 operations)

**SDK Coverage: ~10/13 (77%)**

**Basic CRUD (5):**
```rust
"list_tools" => {
    client.tools().list(params).await?
}

"create_tool" => {
    client.tools().create(request).await?
}

"get_tool" => {
    client.tools().get(&tool_id).await?
}

"update_tool" => {
    client.tools().update(&tool_id, request).await?
}

"delete_tool" => {
    client.tools().delete(&tool_id).await?
}
```

**MCP Integration (8):**
```rust
// Need to verify SDK support for these
"list_mcp_servers" => {
    // GET /v1/tools/mcp/servers
}

"list_mcp_tools" => {
    // GET /v1/tools/mcp/servers/{server}/tools
}

"add_mcp_tool" => {
    // POST /v1/tools/mcp/servers/{server}/{tool}
}

"resync_mcp_server" => {
    // POST /v1/tools/mcp/servers/{server}/resync
}

// Additional operations
"count_tools"
"add_base_tools"
"run_tool"
"generate_schema"
```

**Estimated Effort:** 2-3 days
**Complexity:** Medium-High (MCP integration needs verification)
**Files:** `letta-server/src/tools/tool_manager.rs` (NEW)

---

### Phase 3: Sources & Jobs (Week 3) 🟢

#### 3.1 Implement `letta_source_manager` (15 operations)

**SDK Coverage: 13/15 (87%)**

**CRUD Operations (6):**
```rust
"list_sources" => client.sources().list().await?
"create_source" => client.sources().create(request).await?
"get_source" => client.sources().get(&source_id).await?
"update_source" => client.sources().update(&source_id, request).await?
"delete_source" => client.sources().delete(&source_id).await?
"count_sources" => client.sources().count().await?
```

**File Operations (4):**
```rust
"upload_file" => {
    client.sources().upload_file(&source_id, name, data, mime).await?
}

"list_files" => {
    client.sources().list_files(&source_id, params).await?
}

"get_file" => {
    client.sources().get_file(&source_id, &file_id, params).await?
}

"delete_file" => {
    client.sources().delete_file(&source_id, &file_id).await?
}
```

**Passage Operations (3):**
```rust
"list_passages" => {
    client.sources().list_passages(&source_id, params).await?
}

"attach_to_agent" => {
    client.sources().agent_sources(agent_id).attach(&source_id).await?
}

"detach_from_agent" => {
    client.sources().agent_sources(agent_id).detach(&source_id).await?
}
```

**Missing (2):**
- Process source endpoint (may not exist in v0.12.1)
- Get processing jobs (may be in jobs API)

**Estimated Effort:** 2 days
**Complexity:** Low-Medium
**Files:** `letta-server/src/tools/source_manager.rs` (NEW)

---

#### 3.2 Implement `letta_job_monitor` (4 operations)

**SDK Coverage: ~3/4 (75%)**

```rust
"list_jobs" => {
    client.jobs().list(params).await?
}

"get_job" => {
    client.jobs().get(&job_id).await?
}

"list_active_jobs" => {
    client.jobs().list_active().await?
}

"cancel_job" => {
    // May need to check if SDK has this
    client.jobs().cancel(&job_id).await?
}
```

**Estimated Effort:** 0.5 days
**Complexity:** Low
**Files:** `letta-server/src/tools/job_monitor.rs` (NEW)

---

### Phase 4: HTTP Transport (Week 4) 🔵

#### 4.1 Port HTTP Transport to TurboMCP

**Current:** Custom Node.js implementation
**Target:** TurboMCP Rust framework

**Reference Implementation:**
- Node.js: `src/transports/http-transport.js` (450 LOC)
- Features needed:
  - Streamable HTTP with SSE fallback
  - Session management
  - CORS headers
  - Health endpoint
  - Graceful shutdown

**TurboMCP Approach:**
```rust
use turbomcp::server::{Server, Transport};
use axum::{Router, routing::post};

async fn run_http_server(server: Server) -> Result<()> {
    let app = Router::new()
        .route("/mcp", post(handle_mcp_request))
        .route("/health", get(health_check))
        .layer(/* CORS, session middleware */);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

**Estimated Effort:** 3-4 days
**Complexity:** Medium (TurboMCP learning curve)
**Files:** `letta-server/src/transports/http.rs` (NEW)

---

### Phase 5: MCP Operations (Deferred) ⚪

#### 5.1 Implement `letta_mcp_ops` (10 operations)

**Status:** Custom Node.js implementation
**Decision:** Defer until Phases 1-4 complete

**Rationale:**
- No SDK support (custom MCP server management)
- Requires MCP-to-MCP communication
- Complex authentication flows
- Can function without this initially

**Operations:**
- list_mcp_servers
- connect_mcp_server
- disconnect_mcp_server
- get_mcp_server_status
- list_mcp_server_tools
- execute_mcp_tool
- test_mcp_connection
- add_mcp_server_config
- remove_mcp_server_config
- update_mcp_server_config

**Estimated Effort:** 5-7 days (when prioritized)
**Complexity:** High

---

## Implementation Strategy

### Recommended Order

**Week 1: Critical Path**
1. ✅ Day 1-2: Complete `letta_file_folder_ops` (8 operations)
2. ✅ Day 3-5: Complete `letta_agent_advanced` (16 operations)

**Week 2: Core Functionality**
3. ✅ Day 6-7: Implement `letta_memory_unified` (15 operations)
4. ✅ Day 8-10: Implement `letta_tool_manager` (13 operations)

**Week 3: Extended Features**
5. ✅ Day 11-12: Implement `letta_source_manager` (15 operations)
6. ✅ Day 13: Implement `letta_job_monitor` (4 operations)

**Week 4: Transport & Testing**
7. ✅ Day 14-17: Port HTTP transport to TurboMCP
8. ✅ Day 18-20: End-to-end testing and bug fixes

**Future: Advanced Features**
9. ⚪ TBD: Implement `letta_mcp_ops` (10 operations)

---

## Technical Considerations

### Type Safety
- Use SDK types for all request/response
- Define Zod-equivalent schemas using `serde`
- Leverage Rust's type system for compile-time guarantees

### Error Handling
- Unified error handling via `LettaError`
- Map HTTP status codes to MCP error codes
- Preserve error context for debugging

### Tool Pattern
```rust
// Standard handler structure
pub async fn handle_tool_name(
    server: &Server,
    request: ToolRequest
) -> Result<ToolResponse, LettaError> {
    let operation = request.operation;

    match operation.as_str() {
        "operation_one" => handle_operation_one(server, request).await,
        "operation_two" => handle_operation_two(server, request).await,
        _ => Err(LettaError::InvalidOperation(operation)),
    }
}

// Individual operation handler
async fn handle_operation_one(
    server: &Server,
    request: ToolRequest
) -> Result<ToolResponse, LettaError> {
    let client = &server.letta_client;
    let result = client.some_api().some_method(&request.param).await?;

    Ok(ToolResponse::success(result))
}
```

### Testing Strategy
1. Unit tests for each operation handler
2. Integration tests with mock Letta server
3. End-to-end tests against real Letta instance
4. Performance benchmarks vs Node.js implementation

---

## Success Metrics

### Functional Completeness
- [ ] 71/87 operations implemented (82%)
- [ ] All critical tools functional (agent, file, memory)
- [ ] HTTP transport working with sessions
- [ ] Zero compilation errors/warnings

### Performance Targets
- [ ] Startup time < 500ms (Node.js: ~2s)
- [ ] Memory usage < 50MB idle (Node.js: ~80MB)
- [ ] Request latency < Node.js (target: -30%)
- [ ] Handle 100 concurrent requests

### Quality Metrics
- [ ] 80%+ test coverage
- [ ] Zero clippy warnings
- [ ] All SDK operations documented
- [ ] Backward compatible with Node.js clients

---

## Risk Assessment

### High Risk ⚠️
1. **TurboMCP HTTP Transport** - New framework, limited examples
   - Mitigation: Study turbomcp examples, incremental implementation

2. **MCP Tool Integration** - Complex authentication flows
   - Mitigation: Defer to later phase, verify SDK support first

### Medium Risk ⚠️
3. **Missing SDK Endpoints** - Some operations may not have SDK support
   - Mitigation: Fall back to axios-style HTTP calls as needed

4. **Type Compatibility** - SDK types may not match Node.js exactly
   - Mitigation: Create adapter types, comprehensive testing

### Low Risk ✅
5. **File/Folder Operations** - SDK fully supports all endpoints
6. **Memory Operations** - SDK nearly complete (14/15)

---

## Dependencies

### External
- ✅ `letta` SDK (forked, enhanced to 91%)
- ✅ `turbomcp` v2.0.0-rc.3
- ✅ `tokio` async runtime
- ✅ `axum` web framework (for HTTP transport)

### Internal
- ✅ SDK integration complete
- ✅ TurboMCP server framework initialized
- ⏳ Tool registration system
- ⏳ HTTP transport layer

---

## Next Actions

### Immediate (Today)
1. Create `letta-server/src/tools/file_folder_ops.rs`
2. Implement all 8 file/folder operations
3. Register tool with TurboMCP server
4. Write basic unit tests

### This Week
1. Complete remaining `agent_advanced` operations
2. Verify all SDK methods work as expected
3. Set up integration test framework
4. Document API usage patterns

### Next Week
1. Implement memory and tool manager
2. Port HTTP transport to TurboMCP
3. Create comprehensive test suite
4. Performance benchmarking

---

## Resources

### Documentation
- **Letta OpenAPI Spec:** `/tmp/letta-rs/openapi.json`
- **SDK API Docs:** https://github.com/oculairmedia/letta-rs
- **TurboMCP Docs:** https://docs.rs/turbomcp
- **Node.js Reference:** `/opt/stacks/letta-MCP-server/src/tools/`

### Reference Implementations
- **Node.js Agent Tool:** `src/tools/agents/letta-agent-advanced.js`
- **Node.js File Tool:** `src/tools/files/letta-file-folder-ops.js`
- **Node.js Memory Tool:** `src/tools/memory/letta-memory-unified.js`

---

**Status:** Ready to proceed with Phase 1 implementation
**Confidence:** High (SDK support confirmed for 85% of operations)
**Timeline:** 3-4 weeks to full production readiness (excluding MCP ops)
