#!/bin/bash
# Test script to inspect generated schema for letta_agent_advanced tool

echo "=== Testing letta_agent_advanced schema generation ==="
echo ""

# Start server in background
LETTA_BASE_URL="http://test" LETTA_PASSWORD="test" timeout 2 ./target/release/letta-server --http 2>&1 &
PID=$!
sleep 1

# Make MCP tools/list request
SCHEMA=$(curl -s -X POST http://localhost:6507/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}' | \
  jq '.result.tools[] | select(.name == "letta_agent_advanced") | .inputSchema.properties')

echo "Schema properties for letta_agent_advanced:"
echo "$SCHEMA" | jq '.'

echo ""
echo "Checking for missing 'type' fields..."
echo "llm_config type: $(echo "$SCHEMA" | jq -r '.llm_config.type // "MISSING"')"
echo "embedding_config type: $(echo "$SCHEMA" | jq -r '.embedding_config.type // "MISSING"')"
echo "tool_ids type: $(echo "$SCHEMA" | jq -r '.tool_ids.type // "MISSING"')"
echo "export_data type: $(echo "$SCHEMA" | jq -r '.export_data.type // "MISSING"')"
echo "update_data type: $(echo "$SCHEMA" | jq -r '.update_data.type // "MISSING"')"
echo "pagination type: $(echo "$SCHEMA" | jq -r '.pagination.anyOf[0].type // "MISSING"')"
echo "filters type: $(echo "$SCHEMA" | jq -r '.filters.anyOf[0].type // "MISSING"')"
echo "search_filters type: $(echo "$SCHEMA" | jq -r '.search_filters.anyOf[0].type // "MISSING"')"

# Cleanup
kill $PID 2>/dev/null
wait $PID 2>/dev/null

