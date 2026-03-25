use std::net::SocketAddr;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "A minimal MCP echo server built with rmcp.",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    transport: Transport,
}

#[derive(Debug, Subcommand)]
enum Transport {
    /// Serve MCP over stdio.
    Stdio,
    /// Serve MCP over streamable HTTP at /mcp.
    #[command(alias = "streamable-http")]
    Http {
        /// Socket address to bind, for example 127.0.0.1:8000.
        #[arg(long, default_value = "127.0.0.1:8000")]
        bind: SocketAddr,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    echo_mcp::init_tracing();

    match Cli::parse().transport {
        Transport::Stdio => echo_mcp::serve_stdio().await,
        Transport::Http { bind } => echo_mcp::serve_streamable_http(bind).await,
    }
}
