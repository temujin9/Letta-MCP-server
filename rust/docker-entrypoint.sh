#!/bin/bash
set -e

echo "Starting Letta MCP server on port ${PORT:-6507}..."

# Run letta-server
exec /usr/local/bin/letta-server --http
