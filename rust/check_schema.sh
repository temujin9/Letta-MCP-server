#!/bin/bash
cd /opt/stacks/letta-MCP-server/rust

# Start server in background with dummy credentials
LETTA_BASE_URL=http://example.com/v1 LETTA_PASSWORD=test ./target/release/letta-server --http > /tmp/server.log 2>&1 &
SERVER_PID=$!

# Wait for server to start
/bin/sleep 2

# Get schema
SCHEMA=$(curl -s -X POST http://localhost:3001/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}' | \
  jq '.result.tools[] | select(.name == "letta_agent_advanced") | .inputSchema')

# Kill server
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

# Check for $defs
echo "$SCHEMA" | jq '{
  "has_defs": has("$defs"),
  "has_refs": ([.. | objects | to_entries[] | select(.key == "$ref")] | length > 0),
  "defs_count": (if has("$defs") then (."$defs" | length) else 0 end),
  "top_level_keys": keys
}'
