# Multi-stage build for Letta MCP Rust Server with nginx proxy
# Stage 1: Build the Rust binary
# Using nightly for Rust 2024 edition features (let chains, etc.)
FROM rustlang/rust:nightly-slim AS builder

# Install build dependencies including git for fetching git dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY letta-server ./letta-server
COPY letta-types ./letta-types

# Configure git to use https for GitHub (public repos don't need auth)
RUN git config --global url."https://github.com/".insteadOf git@github.com:

# Clear cargo git cache to ensure fresh fetch of turbomcp workspace structure
RUN rm -rf ~/.cargo/git

# Fetch all dependencies first (uses Cargo.lock for exact versions)
RUN cargo fetch --locked

# Build release binary (will use cached dependencies from fetch)
RUN cargo build --release --bin letta-server --locked

# Stage 2: Create minimal runtime image with nginx
FROM debian:bookworm-slim

# Install runtime dependencies and nginx
RUN apt-get update && apt-get install -y \
    ca-certificates \
    nginx \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 letta

# Copy binary from builder
COPY --from=builder /build/target/release/letta-server /usr/local/bin/letta-server
RUN chmod +x /usr/local/bin/letta-server

# Copy nginx configuration
COPY nginx.conf /etc/nginx/sites-available/default

# Create directories for nginx
RUN mkdir -p /var/log/nginx /var/lib/nginx/body /var/lib/nginx/proxy \
    && chown -R letta:letta /var/log/nginx /var/lib/nginx

# Expose proxy port (nginx)
EXPOSE 3001

# Health check on proxy port
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/health || exit 1

# Copy entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Switch to app user
USER letta

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
