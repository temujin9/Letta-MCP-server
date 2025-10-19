# Letta Rust SDK API Coverage Analysis

**SDK Version:** letta v0.1.2
**Date:** 2025-10-18
**Purpose:** Determine if the Rust SDK supports all 87 operations across 7 consolidated MCP tools

## Executive Summary

✅ **Agent Operations (22)**: Partially supported - 6/22 implemented, SDK covers most needs
⚠️ **Memory Operations (15)**: Fully supported by SDK
✅ **Tool Operations (13)**: Need to verify blocks API
❌ **File/Folder Operations (8)**: **CRITICAL GAP** - Missing agent file session management
⚠️ **Source Operations (15)**: Partially supported - Different API structure
⚠️ **MCP Operations (10)**: Custom endpoints - No SDK support expected
✅ **Job Operations (4)**: Need to verify jobs API

**Overall Coverage: ~70-75% (estimated)**
**Critical Gaps: File session operations, folder operations**

---

## Detailed Analysis

### 1. Agent Advanced Operations (22 operations)

**Node.js Tool:** `letta-agent-advanced.js`

| Operation | Node.js Method | Rust SDK Method | Status | Notes |
|-----------|---------------|-----------------|--------|-------|
| list | `client.agents.list()` | `client.agents().list()` | ✅ DONE | Cursor-based pagination |
| create | `client.agents.create()` | `client.agents().create()` | ✅ DONE | Type-safe request |
| get | `client.agents.retrieve()` | `client.agents().get()` | ✅ DONE | LettaId parsing |
| update | `client.agents.update()` | ❌ NOT IN SDK | ⚠️ TODO | SDK v0.1.2 lacks this |
| delete | `client.agents.delete()` | `client.agents().delete()` | ✅ DONE | |
| list_tools | `client.agents.tools.list()` | `client.memory().list_agent_tools()` | ⚠️ PENDING | In memory API |
| send_message | `client.agents.messages.create()` | `client.messages().create()` | ✅ DONE | Non-streaming |
| export | `client.agents.export()` | `client.agents().export_file()` | ⚠️ PENDING | Returns JSON string |
| import | `client.agents.import()` | `client.agents().import_file()` | ⚠️ PENDING | Multipart form |
| clone | `axios POST /agents/{id}/clone` | ❌ NOT IN SDK | ❌ GAP | Custom HTTP needed |
| get_config | `axios GET /agents/{id}/config` | ❌ NOT IN SDK | ❌ GAP | Custom HTTP needed |
| bulk_delete | `axios POST /agents/bulk-delete` | ❌ NOT IN SDK | ❌ GAP | Custom HTTP needed |
| context | `axios GET /agents/{id}/context` | ❌ NOT IN SDK | ❌ GAP | Custom HTTP needed |
| reset_messages | `axios DELETE /agents/{id}/messages` | ❌ NOT IN SDK | ❌ GAP | Custom HTTP needed |
| summarize | `client.agents.summarize()` | `client.agents().summarize_agent_conversation()` | ⚠️ PENDING | SDK has it |
| stream | `client.messages.createStream()` | `client.messages().create_stream()` | ⚠️ PENDING | Streaming support |
| async_message | `axios POST /agents/{id}/messages/async` | ❌ NOT IN SDK | ❌ GAP | Custom HTTP needed |
| cancel_message | `axios DELETE /agents/{id}/messages/{msg_id}` | ❌ NOT IN SDK | ❌ GAP | Custom HTTP needed |
| preview_payload | `axios POST /agents/{id}/preview` | ❌ NOT IN SDK | ❌ GAP | Custom HTTP needed |
| search_messages | `client.agents.messages.search()` | ❌ NOT IN SDK | ❌ GAP | Search API missing |
| get_message | `client.agents.messages.retrieve()` | `client.messages().get()` | ⚠️ PENDING | Check signature |
| count | `client.agents.count()` | `client.agents().count()` | ⚠️ PENDING | SDK has it |

**SDK Coverage: 6/22 done, 9/22 need SDK methods, 7/22 require custom HTTP**

---

### 2. Memory Unified Operations (15 operations)

**Node.js Tool:** `letta-memory-unified.js`

| Operation | Node.js Method | Rust SDK Method | Status |
|-----------|---------------|-----------------|--------|
| get_core_memory | `client.agents.coreMemory.retrieve()` | `client.memory().get_core_memory()` | ✅ SDK |
| update_core_memory | `client.agents.blocks.modify()` | `client.memory().update_core_memory_block()` | ✅ SDK |
| get_block_by_label | `client.agents.blocks.retrieveByLabel()` | `client.memory().get_core_memory_block()` | ✅ SDK |
| list_blocks | `client.blocks.list()` | `client.blocks().list()` | ✅ SDK |
| create_block | `client.blocks.create()` | `client.blocks().create()` | ⚠️ CHECK |
| get_block | `client.blocks.retrieve()` | `client.blocks().get()` | ⚠️ CHECK |
| update_block | `client.blocks.update()` | `client.blocks().update()` | ⚠️ CHECK |
| attach_block | `client.agents.blocks.attach()` | `client.memory().attach_memory_block()` | ✅ SDK |
| detach_block | `client.agents.blocks.detach()` | `client.memory().detach_memory_block()` | ✅ SDK |
| list_agents_using_block | `client.blocks.agents.list()` | ❓ UNKNOWN | ⚠️ CHECK |
| search_archival | `client.agents.passages.search()` | `client.memory().list_archival_memory()` | ✅ SDK |
| list_passages | `client.agents.passages.list()` | `client.memory().list_archival_memory()` | ✅ SDK |
| create_passage | `client.agents.passages.create()` | `client.memory().create_archival_memory()` | ✅ SDK |
| update_passage | `client.agents.passages.modify()` | `client.memory().update_archival_memory()` | ✅ SDK |
| delete_passage | `client.agents.passages.delete()` | `client.memory().delete_archival_memory()` | ✅ SDK |

**SDK Coverage: ~90% - Need to verify blocks API methods**

---

### 3. File/Folder Operations (8 operations) 🚨 CRITICAL

**Node.js Tool:** `letta-file-folder-ops.js`

| Operation | Node.js Method | Rust SDK Method | Status |
|-----------|---------------|-----------------|--------|
| list_files | `client.agents.files.list(agent_id)` | ❌ NOT AVAILABLE | ❌ **MISSING** |
| open_file | `client.agents.files.open(agent_id, file_id)` | ❌ NOT AVAILABLE | ❌ **MISSING** |
| close_file | `client.agents.files.close(agent_id, file_id)` | ❌ NOT AVAILABLE | ❌ **MISSING** |
| close_all_files | `client.agents.files.closeAll(agent_id)` | ❌ NOT AVAILABLE | ❌ **MISSING** |
| list_folders | `client.folders.list()` | ❌ NO FOLDER API | ❌ **MISSING** |
| attach_folder | `client.agents.folders.attach(agent_id, folder_id)` | ❌ NO FOLDER API | ❌ **MISSING** |
| detach_folder | `client.agents.folders.detach(agent_id, folder_id)` | ❌ NO FOLDER API | ❌ **MISSING** |
| list_agents_in_folder | `client.folders.agents.list(folder_id)` | ❌ NO FOLDER API | ❌ **MISSING** |

**SDK Coverage: 0/8 - ZERO file/folder operations supported**

### Alternative: Source API (Not Equivalent)

The Rust SDK has a `sources()` API with file operations, but they're **source-scoped**, not **agent-scoped**:

```rust
// SDK has source-based file operations:
client.sources().upload_file(source_id, file_name, file_data, content_type)
client.sources().list_files(source_id, params)
client.sources().get_file(source_id, file_id, params)
client.sources().delete_file(source_id, file_id)

// Also has agent-source relationships:
client.sources().agent_sources(agent_id).list()      // List sources for agent
client.sources().agent_sources(agent_id).attach(source_id)
client.sources().agent_sources(agent_id).detach(source_id)
```

**Problem:** Node.js uses agent file **session management** (open/close), which is different from source file storage. The SDK doesn't expose:
- Agent file session API (`/v1/agents/{id}/files`)
- Folder API (`/v1/folders`)

**Workaround Required:** Custom HTTP calls to these endpoints

---

### 4. Source Operations (15 operations)

**Node.js Tool:** `letta-source-manager.js`

| Operation | Node.js Method | Rust SDK Method | Status |
|-----------|---------------|-----------------|--------|
| list_sources | `client.sources.list()` | `client.sources().list()` | ✅ SDK |
| create_source | `client.sources.create()` | `client.sources().create()` | ✅ SDK |
| get_source | `client.sources.retrieve()` | `client.sources().get()` | ✅ SDK |
| update_source | `client.sources.update()` | `client.sources().update()` | ✅ SDK |
| delete_source | `client.sources.delete()` | `client.sources().delete()` | ✅ SDK |
| attach_to_agent | `client.agents.sources.attach()` | `client.sources().agent_sources(id).attach()` | ✅ SDK |
| detach_from_agent | `client.agents.sources.detach()` | `client.sources().agent_sources(id).detach()` | ✅ SDK |
| list_agent_sources | `client.agents.sources.list()` | `client.sources().agent_sources(id).list()` | ✅ SDK |
| upload_file_to_source | `client.sources.uploadFile()` | `client.sources().upload_file()` | ✅ SDK |
| list_source_files | `client.sources.files.list()` | `client.sources().list_files()` | ✅ SDK |
| get_source_file | `client.sources.files.retrieve()` | `client.sources().get_file()` | ✅ SDK |
| delete_source_file | `client.sources.files.delete()` | `client.sources().delete_file()` | ✅ SDK |
| list_source_passages | `client.sources.passages.list()` | `client.sources().list_passages()` | ✅ SDK |
| process_source | `axios POST /sources/{id}/process` | ❌ NOT IN SDK | ❌ GAP |
| get_source_jobs | `axios GET /sources/{id}/jobs` | ❌ NOT IN SDK | ❌ GAP |

**SDK Coverage: 13/15 - Very good, 2 require custom HTTP**

---

### 5. Tool Manager Operations (13 operations)

**Node.js Tool:** `letta-tool-manager.js`

*Need to investigate tools API in SDK*

**Initial estimate: 80% coverage** (standard CRUD + attach/detach should be available)

---

### 6. MCP Operations (10 operations)

**Node.js Tool:** `letta-mcp-ops.js`

These are **custom MCP-specific endpoints** like:
- `GET /v1/tools/mcp/servers`
- `GET /v1/tools/mcp/servers/{name}/tools`

**SDK Coverage: 0/10 - Expected, will need custom HTTP**

---

### 7. Job Monitor Operations (4 operations)

**Node.js Tool:** `letta-job-monitor.js`

*Need to check if SDK has jobs API*

**Initial estimate: 75% coverage**

---

## Critical Findings

### 🚨 Major Gap: File & Folder Operations

The Rust SDK **does not support**:
1. **Agent file sessions** (`/v1/agents/{id}/files/*`)
   - open_file, close_file, close_all_files
   - These are LRU-based file session management endpoints

2. **Folder operations** (`/v1/folders/*`)
   - list_folders, attach_folder, detach_folder, list_agents_in_folder
   - No folder types or API in SDK at all

### Workaround Options

1. **Add custom HTTP methods** to our LettaServer wrapper
2. **Contribute to SDK** - Add missing endpoints upstream
3. **Use direct HTTP calls** via reqwest for these 8 operations

### Recommended Approach

For the Rust refactor, use a **hybrid approach**:
- Use SDK for 70-75% of operations (faster, type-safe)
- Add custom HTTP methods for missing endpoints (8-10 operations)
- Keep axios-compatible error handling

---

## Next Steps

1. ✅ Create this coverage analysis document
2. ⏳ Verify blocks API, tools API, jobs API availability
3. ⏳ Implement custom HTTP handlers for gaps:
   - Agent file session operations (4 ops)
   - Folder operations (4 ops)
   - Agent advanced custom endpoints (7 ops)
   - MCP custom endpoints (10 ops)
4. ⏳ Continue implementing remaining SDK-based operations
5. ⏳ Test all operations for compatibility

---

## API Coverage by Tool

| Tool | Total Ops | SDK Support | Custom HTTP | Coverage % |
|------|-----------|-------------|-------------|------------|
| Agent Advanced | 22 | ~15 | ~7 | ~68% |
| Memory Unified | 15 | ~14 | ~1 | ~93% |
| File/Folder | 8 | 0 | 8 | **0%** |
| Source Manager | 15 | ~13 | ~2 | ~87% |
| Tool Manager | 13 | ~10 | ~3 | ~77% |
| MCP Ops | 10 | 0 | 10 | 0% (expected) |
| Job Monitor | 4 | ~3 | ~1 | ~75% |
| **TOTAL** | **87** | **~55** | **~32** | **~63%** |

**Conclusion:** The SDK covers approximately **63% of operations**. The remaining 37% (32 operations) will require custom HTTP implementation, primarily concentrated in file/folder ops and MCP-specific endpoints.
