# Letta SDK v1.0 Compatibility Assessment

## Executive Summary

This MCP server makes direct HTTP API calls to the Letta server using axios, rather than using the official Letta SDK. Therefore, we need to ensure our API requests are compatible with the v1.0 API changes.

## Key Findings from Research

### 1. Response Format ✅ No Changes Needed
- List endpoints **still return direct arrays**, not paginated objects with `.items`
- Response structure: `list[AgentState]` (not `{ items: [...] }`)
- Pagination is handled via query parameters (`before`, `after`, `limit`)

### 2. Property Naming ✅ Mostly Compliant
- API uses **snake_case** for all properties
- Current code already uses:
  - `llm_config` ✓
  - `embedding_config` ✓
  - `agent_id`, `tool_ids`, `block_ids` ✓

### 3. Query Parameter Changes ⚠️ ACTION REQUIRED
The following parameters have been deprecated for SDK v1.0+:
- `include_relationships` → `include` (NEW)
- `sort_by` → `order_by` (NEW)
- `ascending` → `order` (accepts "asc"/"desc") (NEW)

### 4. SDK Version Detection ⚠️ ACTION REQUIRED
The server detects SDK v1.0 clients via headers. We should add headers to identify as v1.0 compatible client.

### 5. Tool Call Structure Changes 📝 NOTE
- SDK v1.0 changed tool calls from single object → array for parallel tools
- This affects agent message responses but not our request structure

## Required Changes

### Priority 1: API Headers
**File**: `src/core/server.js`
- Add SDK version identifier to headers
- Likely via User-Agent or custom header

### Priority 2: Query Parameters (if used)
Review all tools that use list operations:
- `src/tools/agents/list-agents.js` - Check for sorting parameters
- `src/tools/agents/get-agent-summary.js` - Check for include_relationships usage
- Other list operations

### Priority 3: Archive Management APIs
New archive APIs introduced in v0.14.0:
- Direct passage creation in archives
- Pre-populate knowledge bases

## Tools Assessment

### Currently Compliant ✅
1. **create-agent.js** - Uses `llm_config` (snake_case) ✓
2. **list-agents.js** - Returns direct array, no pagination wrapper ✓
3. **get-agent-summary.js** - Uses snake_case properties ✓
4. **modify-agent.js** - Uses PATCH with correct endpoint ✓
5. **update-memory-block.js** - Uses snake_case ✓
6. **modify-passage.js** - Uses snake_case ✓

### Needs Review ⚠️
1. Any tools using `include_relationships` parameter
2. Any tools using `sort_by` or `ascending` parameters
3. Tools that parse tool_call responses (parallel tool calling)

## Recommendations

1. **Add SDK v1.0 headers** to all API requests to ensure server treats us as v1.0 client
2. **Audit all list operations** for deprecated parameters
3. **Test with actual Letta server v0.14.0+** to verify compatibility
4. **Consider adding new archive management tools** for v1.0 features
5. **Update documentation** to reflect v1.0 compatibility

## Next Steps

1. Update `getApiHeaders()` to include v1.0 identification
2. Search codebase for deprecated parameter usage
3. Update any found instances
4. Test against Letta server
5. Document changes
