//! Letta MCP Server - Main Entry Point
//!
//! This binary starts the Letta MCP server with the selected transport protocol.

use letta_server::LettaServer;
use std::env;

#[cfg(feature = "http")]
use turbomcp::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing/logging
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    // Get configuration from environment
    let base_url = env::var("LETTA_BASE_URL")
        .expect("LETTA_BASE_URL environment variable is required");
    let password = env::var("LETTA_PASSWORD")
        .expect("LETTA_PASSWORD environment variable is required");
    let transport = env::var("TRANSPORT").unwrap_or_else(|_| "stdio".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .expect("PORT must be a valid number");

    tracing::info!("╔══════════════════════════════════════╗");
    tracing::info!("║   Letta MCP Server (Rust/TurboMCP)  ║");
    tracing::info!("╚══════════════════════════════════════╝");
    tracing::info!("Version: 2.0.1");
    tracing::info!("Transport: {}", transport);
    tracing::info!("Letta API: {}", base_url);

    // Create server instance with Letta SDK
    let server = LettaServer::new(base_url, password)?;

    // Run with selected transport
    match transport.to_lowercase().as_str() {
        "http" => {
            let addr = format!("0.0.0.0:{}", port);
            tracing::info!("🚀 Starting HTTP transport");
            tracing::info!("📡 Listening on: http://{}", addr);
            tracing::info!("🔗 Endpoint: http://{}/mcp", addr);
            tracing::info!("⚠️  CORS: Allowing all origins (development mode)");
            tracing::info!("Ready for MCP client connections");

            // Set environment variable to allow any origin
            std::env::set_var("MCP_ALLOW_ANY_ORIGIN", "true");

            server.run_http(&addr).await?;
        }
        "stdio" | _ => {
            tracing::info!("🚀 Starting stdio transport");
            tracing::info!("Ready for MCP client connections");

            server.run_stdio().await?;
        }
    }

    tracing::info!("Server shutdown complete");
    Ok(())
}
