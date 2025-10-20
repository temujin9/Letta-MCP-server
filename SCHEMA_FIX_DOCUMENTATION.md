# Schema Validation Fix Documentation

## Problem Summary

When migrating from Node.js to Rust implementation, several MCP tools had invalid schemas in the Letta API:
- **letta_agent_advanced**: Fields showing as "complex types" without type information
- **letta_tool_manager**: Invalid schema due to missing type definitions
- **letta_mcp_ops**: Invalid schema due to missing type definitions

Additionally, field descriptions (doc comments) were not appearing in the schema for any tools except `letta_agent_advanced`.

## Root Causes

### 1. serde_json::Value doesn't implement JsonSchema

When using `Option<serde_json::Value>` in request structs, schemars cannot generate a proper schema. This results in fields appearing without a `type` property, causing Letta to reject the schema as invalid.

**Example of invalid schema:**
```json
{
  "json_schema": {
    "description": "Tool JSON schema object"
    // Missing: "type": "object"
  }
}
```

### 2. Parameter Flattening vs. Request Struct Pattern

TurboMCP's schema generation works differently depending on how tool functions receive parameters:

**Flattened Parameters (OLD - descriptions don't work):**
```rust
#[tool]
async fn letta_memory_unified(
    &self,
    operation: String,
    agent_id: Option<String>,
    block_id: Option<String>,
    // ... many individual parameters
) -> McpResult<String>
```
- TurboMCP generates schema from function signature
- Doc comments on struct fields are ignored
- No field descriptions in generated schema

**Request Struct Pattern (NEW - descriptions work):**
```rust
#[tool]
async fn letta_memory_unified(
    &self,
    request: memory_unified::MemoryUnifiedRequest,
) -> McpResult<String>
```
- TurboMCP uses schemars-generated schema from the struct
- Doc comments (`///`) are included as field descriptions
- Proper schema with all metadata

### 3. Complex Types Need Inlining

Without `#[schemars(inline)]`, schemars generates `$ref` references that Letta doesn't resolve properly, causing "complex types" to appear without type information.

## Solution

### Step 1: Create JsonValue Wrapper

Create a newtype wrapper that implements JsonSchema for serde_json::Value:

**File: `letta-server/src/tools/agent_advanced.rs`** (lines 12-44)
```rust
/// Newtype wrapper for serde_json::Value with JsonSchema implementation
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct JsonValue(pub Value);

impl schemars::JsonSchema for JsonValue {
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // Letta prefers simple type definitions without anyOf
        schemars::json_schema!({
            "type": "object"
        })
    }

    fn inline_schema() -> bool {
        true
    }
}
```

**Key points:**
- `#[serde(transparent)]`: Serializes/deserializes exactly like the inner Value
- Manual `JsonSchema` implementation: Provides explicit `"type": "object"`
- `inline_schema() -> true`: Forces schema inlining instead of $ref
- Public field `.0` allows extracting inner Value when needed

### Step 2: Use JsonValue in Request Structs

Replace `Option<serde_json::Value>` with `Option<JsonValue>`:

**Before:**
```rust
pub struct ToolManagerRequest {
    pub json_schema: Option<Value>,
    pub args_json_schema: Option<Value>,
    pub args: Option<Value>,
}
```

**After:**
```rust
use super::agent_advanced::JsonValue;

pub struct ToolManagerRequest {
    /// Tool JSON schema object (for create/update operations)
    pub json_schema: Option<JsonValue>,

    /// Tool arguments JSON schema (for create/update operations)
    pub args_json_schema: Option<JsonValue>,

    /// Tool execution arguments (for run_from_source operation)
    pub args: Option<JsonValue>,
}
```

### Step 3: Extract Inner Value When Passing to SDK

When calling SDK methods that expect `Option<Value>`, extract the inner value using `.map(|j| j.0)`:

```rust
let create_request = letta::types::tool::CreateToolRequest {
    source_code,
    description: request.description,
    json_schema: request.json_schema.map(|j| j.0),        // Extract inner Value
    args_json_schema: request.args_json_schema.map(|j| j.0),  // Extract inner Value
    source_type,
    tags: request.tags,
    return_char_limit: request.return_char_limit,
    pip_requirements: None,
};
```

For non-Option fields:
```rust
let args = request.args.ok_or(...)?;
let run_request = RunToolFromSourceRequest {
    args: args.0,  // Extract inner Value
    // ...
};
```

### Step 4: Add Inline Attribute to Complex Structs

Add `#[schemars(inline)]` to struct definitions that are used as field types:

**File: `letta-types/src/lib.rs`**
```rust
#[derive(Debug, Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[schemars(inline)]  // Force schema inlining
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}
```

**File: `letta-server/src/tools/agent_advanced.rs`**
```rust
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[schemars(inline)]  // Force schema inlining
pub struct BulkDeleteFilters {
    /// Filter agents by name pattern
    pub agent_name_filter: Option<String>,
    // ...
}
```

### Step 5: Convert All Tools to Request Struct Pattern

Update all tool functions in `letta-server/src/lib.rs` to use request struct pattern:

**Before:**
```rust
#[tool]
async fn letta_memory_unified(
    &self,
    operation: String,
    agent_id: Option<String>,
    // ... 10+ individual parameters
) -> McpResult<String> {
    // Manually construct request struct
    let request = MemoryUnifiedRequest {
        operation,
        agent_id,
        // ... map all fields
    };
    memory_unified::handle_memory_unified(&self.client, request).await
}
```

**After:**
```rust
#[tool]
async fn letta_memory_unified(
    &self,
    request: memory_unified::MemoryUnifiedRequest,
) -> McpResult<String> {
    // TurboMCP uses schemars schema with doc comments
    memory_unified::handle_memory_unified(&self.client, request).await
}
```

### Step 6: Update Handlers to Return String

Change handler return types from `Result<XxxResponse, McpError>` to `Result<String, McpError>`:

**Before:**
```rust
pub async fn handle_memory_unified(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<MemoryUnifiedResponse, McpError> {
    match request.operation {
        MemoryOperation::GetCoreMemory => handle_get_core_memory(client, request).await,
        // ...
    }
}
```

**After:**
```rust
pub async fn handle_memory_unified(
    client: &LettaClient,
    request: MemoryUnifiedRequest,
) -> Result<String, McpError> {
    let response = match request.operation {
        MemoryOperation::GetCoreMemory => handle_get_core_memory(client, request).await?,
        // ...
    };

    Ok(serde_json::to_string_pretty(&response)?)
}
```

### Step 7: Add Doc Comments to All Request Structs

Add `///` doc comments to all fields in request structs:

```rust
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MemoryUnifiedRequest {
    /// The operation to perform (get_core_memory, update_core_memory, list_blocks, create_block, etc.)
    pub operation: MemoryOperation,

    /// Agent ID (required for agent-specific operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// Memory block ID (required for block operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,

    // ... all other fields with descriptions
}
```

## Files Modified

### Core Implementation
1. **letta-server/src/tools/agent_advanced.rs**: Created JsonValue wrapper (lines 12-44)
2. **letta-server/src/tools/tool_manager.rs**:
   - Imported JsonValue
   - Updated request struct to use JsonValue
   - Updated handlers to extract inner Value
3. **letta-server/src/tools/mcp_ops.rs**:
   - Imported JsonValue
   - Updated request struct to use JsonValue
   - Updated handlers to extract inner Value

### Request Structs Updated
4. **letta-server/src/tools/memory_unified.rs**: Added doc comments, updated handler
5. **letta-server/src/tools/source_manager.rs**: Added doc comments, updated handler
6. **letta-server/src/tools/job_monitor.rs**: Added doc comments, updated handler
7. **letta-server/src/tools/file_folder_ops.rs**: Added doc comments, updated handler

### Shared Types
8. **letta-types/src/lib.rs**: Added `#[schemars(inline)]` to Pagination and Message structs

### Tool Registration
9. **letta-server/src/lib.rs**: Updated all 7 tools to use request struct pattern

## Verification

After applying the fix, verify:

1. **All tools are listed:**
```bash
curl -s -X POST http://localhost:3001/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}' \
  | jq -r '.result.tools[] | .name'
```

2. **Schema has type fields:**
```bash
curl -s -X POST http://localhost:3001/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}' \
  | jq '.result.tools[] | select(.name == "letta_tool_manager") | .inputSchema.properties.json_schema'
```

Expected output:
```json
{
  "description": "Tool JSON schema object (for create/update operations)",
  "type": ["object", "null"]
}
```

3. **All tools have descriptions:**
```bash
curl -s -X POST http://localhost:3001/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}' \
  | jq -r '.result.tools[] | "\(.name): \(.inputSchema.properties.operation.description)"'
```

All tools should show operation descriptions.

## Key Takeaways

1. **Always use JsonValue wrapper** for `serde_json::Value` fields in request structs that need JsonSchema
2. **Use request struct pattern** for tool functions to get doc comments in schema
3. **Add inline attribute** to complex struct types to avoid $ref issues
4. **Extract inner Value** when passing JsonValue to SDK methods using `.map(|j| j.0)` or `.0`
5. **Return serialized strings** from handlers for MCP compatibility
6. **Document all fields** with `///` comments for better schema descriptions

## Related Issues

- Original issue: letta_agent_advanced showing "complex types" without type information
- Root cause: serde_json::Value doesn't implement JsonSchema trait
- TurboMCP modification: Custom flatten support on feature/flatten-structs branch
- Letta API validation: Strict schema requirements with explicit type fields

## References

- schemars documentation: https://graham.cool/schemars/
- TurboMCP fork: feature/flatten-structs branch
- Letta SDK: @letta-ai/letta-client v0.0.68664
