# Letta MCP Rust Server

High-performance Rust implementation of the Letta MCP server using TurboMCP framework.

## Architecture

- **Rust MCP Server**: Runs on internal port `6507`
- **nginx Proxy**: Exposes on port `3001` with SSE support and optimized buffering
- **TurboMCP**: Official Rust MCP framework with HTTP transport
- **Letta SDK**: Official letta-rs client library

## Quick Start

### Using Docker Compose (Recommended)

```bash
# From the rust/ directory
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

### Manual Docker Build

```bash
# Build image
docker build -t letta-mcp-rust:latest .

# Run container
docker run -d \
  -p 3001:3001 \
  -e LETTA_BASE_URL=https://your-letta-instance.com/v1 \
  -e LETTA_PASSWORD=your-password \
  --name letta-mcp-rust \
  letta-mcp-rust:latest
```

### Local Development

```bash
# Build release binary
cargo build --release

# Run with HTTP transport on port 6507
PORT=6507 ./target/release/letta-server --http
```

## Environment Variables

### Required

- `LETTA_BASE_URL`: Letta API base URL (must include /v1 suffix)
- `LETTA_PASSWORD`: Authentication password for Letta API

### Optional

- `PORT`: Internal server port (default: 6507, proxied through nginx to 3001)
- `RUST_LOG`: Logging level (debug, info, warn, error)
- `RUST_BACKTRACE`: Enable backtraces (0 or 1)

## Configuration

### nginx Proxy

The nginx proxy on port 3001 provides:
- SSE (Server-Sent Events) support for streaming
- CORS headers for cross-origin requests
- Request buffering optimization
- Connection keepalive
- Health check endpoint

### TurboMCP Features

This implementation uses TurboMCP with:
- `#[turbomcp(flatten)]` for auto-generated parameter schemas
- JSON Schema 2020-12 with `$defs` support
- Discriminator-based tool operations
- Type-safe parameter validation with schemars

## API Endpoints

- `POST http://localhost:3001/mcp` - Main MCP endpoint
- `GET http://localhost:3001/health` - Health check

## Tools Available

7 consolidated tools with 87 operations:

1. **letta_agent_advanced** (22 operations)
   - Agent CRUD, messaging, tool/source management

2. **letta_memory_unified** (15 operations)
   - Core memory, blocks, passages

3. **letta_tool_manager** (13 operations)
   - Tool operations and management

4. **letta_mcp_ops** (10 operations)
   - MCP server integration

5. **letta_source_manager** (15 operations)
   - Source and file operations

6. **letta_job_monitor** (4 operations)
   - Job tracking

7. **letta_file_folder_ops** (8 operations)
   - File and folder management

## Schema Validation

All tools include:
- Complete JSON Schema with `$defs` for referenced types
- Parameter descriptions for all fields
- Required/optional field specifications
- Type-safe deserialization with serde

Example schema structure:
```json
{
  "type": "object",
  "properties": { ... },
  "required": [ ... ],
  "$defs": {
    "Message": { ... },
    "Pagination": { ... }
  }
}
```

## Development

### Project Structure

```
rust/
├── Dockerfile              # Multi-stage build
├── docker-compose.yml      # Compose configuration
├── nginx.conf              # nginx proxy config
├── docker-entrypoint.sh    # Container startup script
├── Cargo.toml              # Workspace configuration
├── letta-server/           # MCP server implementation
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   ├── lib.rs         # Server core
│   │   └── tools/         # Tool implementations
│   └── Cargo.toml
└── letta-types/            # Shared types
    ├── src/lib.rs
    └── Cargo.toml
```

### Dependencies

- **turbomcp**: v2.0.0-rc.3 (with updated `$defs` support)
- **letta**: From oculairmedia/letta-rs fork (add-missing-endpoints branch)
- **tokio**: Async runtime
- **serde**: Serialization
- **schemars**: JSON Schema generation

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check without building
cargo check
```

### Docker Build Details

The multi-stage Dockerfile:
1. **Stage 1 (builder)**: Compiles Rust binary with full optimization
2. **Stage 2 (runtime)**: Minimal Debian image with nginx and binary

Optimizations:
- LTO (Link-Time Optimization)
- Single codegen unit
- Strip symbols
- Release profile with opt-level 3

## Comparison with Node.js Version

| Feature | Rust | Node.js |
|---------|------|---------|
| Memory Usage | ~15MB | ~100MB |
| Startup Time | <100ms | ~1s |
| Request Latency | <5ms | ~20ms |
| Binary Size | ~25MB | N/A |
| Dependencies | Compiled in | node_modules |
| Schema Validation | Compile-time | Runtime |
| Type Safety | Full | TypeScript |

## Troubleshooting

### Port Already in Use

```bash
# Check what's using port 3001
lsof -i :3001

# Or change the external port in docker-compose.yml
ports:
  - "3002:3001"  # External:Internal
```

### nginx Not Starting

Check logs:
```bash
docker-compose logs letta-mcp-rust
```

Verify nginx config:
```bash
docker exec letta-mcp-rust nginx -t
```

### Schema Validation Errors

The server uses strict schema validation with `$defs` support. If you see validation errors:

1. Check the tool definition includes all `$defs`
2. Verify `$ref` paths are correct
3. Ensure all referenced types are in `$defs`

## License

MIT
