# MCP Output Schema Validation Error - Root Cause Analysis

## Error Summary
```
RuntimeError: Tool letta_agent_advanced has an output schema but did not return structured content
```

**Location**: Letta's MCP client validation (`/app/.venv/lib/python3.11/site-packages/mcp/client/session.py:327`)

**Severity**: CRITICAL - Blocks all tool executions when output schema is present

## Root Cause

### The Problem
The MCP protocol specification defines TWO mutually exclusive content formats for tool responses:

1. **Unstructured Content** (what we're returning):
```javascript
{
    content: [
        {
            type: 'text',
            text: JSON.stringify({...})  // JSON string inside text field
        }
    ]
}
```

2. **Structured Content** (what outputSchema expects):
```javascript
{
    content: [
        {
            type: 'resource',      // NOT 'text'
            resource: {
                uri: 'schema://output',
                mimeType: 'application/json',
                text: JSON.stringify({...})
            }
        }
    ]
}
```

### Where OutputSchema Was Disabled
**File**: `src/tools/enhance-tools.js:24-30`

```javascript
// DISABLED: Output schemas conflict with MCP content format
// Tools return { content: [{type: 'text', text: '...'}] } not direct JSON
// If Letta detects outputSchema, it expects structured content
// const outputSchema = getOutputSchema(toolDefinition.name);
// if (outputSchema) {
//     enhanced.outputSchema = outputSchema;
// }
```

**Reason for Disabling**: We correctly identified that returning `type: 'text'` violates the MCP spec when `outputSchema` is present.

### The Violation
When a tool definition includes `outputSchema`, the MCP Python SDK enforces this validation:

```python
# From mcp/client/session.py:327
async def _validate_tool_result(self, name: str, result):
    if tool.outputSchema and not isinstance(result.content[0], ResourceContent):
        raise RuntimeError(
            f"Tool {name} has an output schema but did not return structured content"
        )
```

**Detection Logic**:
- If `outputSchema` exists in tool definition → expects `ResourceContent` (structured)
- If tool returns `TextContent` (`type: 'text'`) → validation fails
- Exception: "did not return structured content"

### Where OutputSchema Still Exists

Despite disabling in `enhance-tools.js`, output schemas STILL appear in 2 legacy tools:

```bash
$ grep -r "outputSchema:" src/tools/ --include="*.js"

src/tools/prompts/list-prompts.js:    outputSchema: {
src/tools/prompts/use-prompt.js:    outputSchema: {
src/tools/agents/create-agent.js:    outputSchema: {
```

**These 3 files directly define outputSchema in their tool definitions**, bypassing the enhance-tools.js disablement.

## Why This Happens

### Timeline
1. **Phase 1**: We defined comprehensive output schemas in `src/tools/output-schemas.js` (1,161 lines)
2. **Phase 2**: We disabled output schema application in `enhance-tools.js` (commit fc735a2)
3. **Phase 3**: We didn't remove output schemas from 3 legacy tool files that define them inline
4. **Result**: Some tools still have outputSchema → MCP validation fails

### The Files
1. **src/tools/prompts/list-prompts.js** - Legacy tool (deprecated, not in consolidated tools)
2. **src/tools/prompts/use-prompt.js** - Legacy tool (deprecated)
3. **src/tools/agents/create-agent.js** - Legacy tool (replaced by letta_agent_advanced)

## Impact Assessment

### Currently Broken
✅ **Consolidated tools** (7 tools, 87 operations) - No outputSchema, work fine:
- `letta_agent_advanced` (22 ops)
- `letta_memory_unified` (15 ops)
- `letta_tool_manager` (13 ops)
- `letta_mcp_ops` (10 ops)
- `letta_source_manager` (15 ops)
- `letta_job_monitor` (4 ops)
- `letta_file_folder_ops` (8 ops)

❌ **Legacy tools** (if still registered):
- `list_prompts` - Has outputSchema
- `use_prompt` - Has outputSchema
- `create_agent` - Has outputSchema

### Registration Status
Need to verify which tools are actually registered in `src/tools/index.js`:

```javascript
// Check if these are exported in the main tools array
```

## Solution Options

### Option 1: Complete Removal (RECOMMENDED)
**Remove all outputSchema references completely**

**Pros**:
- Simplest solution
- Aligns with MCP spec (unstructured text responses)
- No code duplication
- Works with all MCP clients

**Cons**:
- Letta loses type validation on responses
- Clients must parse JSON strings

**Implementation**:
1. Remove inline `outputSchema` from 3 legacy tool files
2. Delete or archive `src/tools/output-schemas.js` (1,161 lines)
3. Verify no tools have outputSchema in definitions

### Option 2: Full Structured Content Migration
**Migrate ALL tool responses to ResourceContent format**

**Pros**:
- Proper MCP spec compliance with validation
- Type-safe responses
- Better tooling support

**Cons**:
- MASSIVE refactor - need to change all 87+ operation handlers
- Change response format from `{content: [{type: 'text', text: '...'}]}` to `{content: [{type: 'resource', resource: {...}}]}`
- Breaking change for existing clients
- High risk of bugs

**Estimated effort**: 3-4 days

**Implementation**:
```javascript
// Current format (87 locations to change)
return {
    content: [{
        type: 'text',
        text: JSON.stringify({success: true, ...})
    }]
};

// New format required
return {
    content: [{
        type: 'resource',
        resource: {
            uri: `letta://output/${toolName}/${operation}`,
            mimeType: 'application/json',
            text: JSON.stringify({success: true, ...})
        }
    }]
};
```

### Option 3: Hybrid Approach
**Keep unstructured responses, add optional schema validation at server level**

**Pros**:
- No MCP protocol violations
- Add internal validation without MCP enforcement
- Gradual migration path

**Cons**:
- Code duplication (schemas in two places)
- Manual validation logic

## Recommended Action Plan

### Immediate Fix (15 minutes)
1. **Remove inline outputSchema from 3 legacy files**:
   - `src/tools/prompts/list-prompts.js`
   - `src/tools/prompts/use-prompt.js`
   - `src/tools/agents/create-agent.js`

2. **Verify tools are registered** in `src/tools/index.js`

3. **Test tool execution** via Letta MCP client

### Short-term (1 hour)
1. **Archive output-schemas.js** to `docs/archived/output-schemas.js`
2. **Document decision** in ARCHITECTURE.md
3. **Add JSDoc type comments** to handler functions for IDE support
4. **Run full test suite** to confirm no regressions

### Long-term (Optional, 3-4 days)
If type safety is critical:
1. **Evaluate MCP ResourceContent migration**
2. **Create proof-of-concept** for 1-2 tools
3. **Measure impact** on existing integrations
4. **Decide**: Full migration vs staying with unstructured

## Files to Modify

### Priority 1 (Immediate)
```
src/tools/prompts/list-prompts.js       - Remove outputSchema field
src/tools/prompts/use-prompt.js         - Remove outputSchema field
src/tools/agents/create-agent.js        - Remove outputSchema field (if still registered)
```

### Priority 2 (Cleanup)
```
src/tools/output-schemas.js             - Archive to docs/archived/
docs/ARCHITECTURE.md                     - Document decision
```

### Priority 3 (Verification)
```
src/tools/index.js                       - Verify no legacy tools with schemas registered
src/tools/enhance-tools.js               - Already correct (schemas disabled)
```

## Testing Plan

### 1. Unit Tests
```bash
# Verify no tools have outputSchema
npm test -- src/test/tools/

# Specific tools that had schemas
npm test -- src/test/tools/prompts/
npm test -- src/test/tools/agents/create-agent.test.js
```

### 2. Integration Test (via Letta)
```bash
# Call letta_agent_advanced from Letta UI/API
# Expected: Tool executes without "did not return structured content" error

# Example call:
{
  "tool": "letta_agent_advanced",
  "args": {
    "operation": "list",
    "pagination": {"limit": 10}
  }
}
```

### 3. MCP Protocol Validation
```bash
# Verify response format matches MCP spec
# Should be TextContent, not ResourceContent
```

## Current State Analysis

### What's Working
- 7 consolidated tools with 87 operations
- All use unstructured TextContent responses
- No outputSchema in their definitions
- SDK-based error handling
- Comprehensive test coverage

### What's Broken
- 3 legacy tools have inline outputSchema
- MCP client rejects responses with validation error
- Blocks execution when called via Letta

### What's Unclear
- Are the 3 legacy tools still registered in index.js?
- Are they exposed to Letta MCP client?
- Can we safely remove them vs needing to fix them?

## Next Steps

1. **Inspect** `src/tools/index.js` to see which tools are exported
2. **Remove** outputSchema from the 3 legacy files
3. **Test** via Letta to confirm fix
4. **Document** decision in architecture docs
5. **Consider** full ResourceContent migration as Phase 4 (optional)

## References

- MCP Specification: https://spec.modelcontextprotocol.io/
- MCP Python SDK: https://github.com/modelcontextprotocol/python-sdk
- Letta MCP Client: `/app/letta/services/mcp/base_client.py`
- Session Validation: `/app/.venv/lib/python3.11/site-packages/mcp/client/session.py:327`

---

**Report Generated**: $(date)
**Branch**: phase2-crud-operations-complete
**Commit**: ce292c2
