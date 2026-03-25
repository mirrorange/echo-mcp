use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use axum::Router;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::{
        StreamableHttpServerConfig,
        io::stdio,
        streamable_http_server::{
            session::local::LocalSessionManager, tower::StreamableHttpService,
        },
    },
};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio_util::sync::CancellationToken;

pub const HTTP_MOUNT_PATH: &str = "/mcp";

#[derive(Clone)]
pub struct EchoServer {
    tool_router: ToolRouter<Self>,
}

impl EchoServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for EchoServer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EchoRequest {
    #[schemars(description = "The message that should be returned unchanged.")]
    pub message: String,
}

#[tool_router]
impl EchoServer {
    #[tool(
        name = "echo",
        description = "Return the provided message exactly as it was received."
    )]
    async fn echo(&self, Parameters(EchoRequest { message }): Parameters<EchoRequest>) -> String {
        message
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for EchoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(
                Implementation::new("echo-mcp", env!("CARGO_PKG_VERSION"))
                    .with_title("Echo MCP")
                    .with_description("A tiny MCP server for transport and client integration tests."),
            )
            .with_instructions(
                "Call the `echo` tool with a JSON object like {\"message\":\"hello\"}. The tool returns the same message unchanged.",
            )
    }
}

pub async fn serve_stdio() -> Result<()> {
    tracing::info!("starting echo-mcp over stdio");
    EchoServer::new().serve(stdio()).await?.waiting().await?;
    Ok(())
}

pub async fn serve_streamable_http(bind: SocketAddr) -> Result<()> {
    let cancellation_token = CancellationToken::new();
    let shutdown_token = cancellation_token.clone();

    let service: StreamableHttpService<EchoServer, LocalSessionManager> =
        StreamableHttpService::new(
            || Ok(EchoServer::new()),
            Default::default(),
            StreamableHttpServerConfig {
                stateful_mode: true,
                sse_keep_alive: Some(Duration::from_secs(15)),
                cancellation_token,
                ..Default::default()
            },
        );

    let router = Router::new().nest_service(HTTP_MOUNT_PATH, service);
    let listener = tokio::net::TcpListener::bind(bind).await?;

    tracing::info!(address = %bind, path = HTTP_MOUNT_PATH, "starting echo-mcp over streamable HTTP");

    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            let _ = tokio::signal::ctrl_c().await;
            shutdown_token.cancel();
        })
        .await?;

    Ok(())
}

pub fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("echo_mcp=info,info"));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn echo_tool_returns_same_message() {
        let server = EchoServer::new();
        let response = server
            .echo(Parameters(EchoRequest {
                message: "ping".to_string(),
            }))
            .await;

        assert_eq!(response, "ping");
    }

    #[test]
    fn server_advertises_echo_tool() {
        let server = EchoServer::new();
        let tools = server.tool_router.list_all();
        let echo_tool = tools
            .iter()
            .find(|tool| tool.name == "echo")
            .expect("echo tool should be registered");

        assert!(!echo_tool.input_schema.is_empty());
        assert_eq!(
            echo_tool.description.as_deref(),
            Some("Return the provided message exactly as it was received.")
        );
    }

    #[test]
    fn server_info_enables_tools() {
        let info = EchoServer::new().get_info();

        assert!(info.capabilities.tools.is_some());
        assert_eq!(info.server_info.name, "echo-mcp");
        assert!(info.instructions.is_some());
    }
}
