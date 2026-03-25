# echo-mcp

A tiny MCP server for testing clients and transport setup. It exposes one tool, `echo`, which simply returns the input message unchanged.

Built with [rmcp](https://docs.rs/rmcp/latest/rmcp/).

## Features

- `stdio` transport for local MCP client integrations
- streamable HTTP transport on `/mcp`
- a single `echo` tool that accepts:

```json
{
  "message": "hello"
}
```

and returns:

```text
hello
```

## Run

```bash
cargo run -- stdio
```

```bash
cargo run -- http --bind 127.0.0.1:8000
```

The streamable HTTP endpoint will be available at `http://127.0.0.1:8000/mcp`.

## Tool

- `echo`
  Returns the provided message exactly as received.

## Example MCP Client Config

```json
{
  "mcpServers": {
    "echo-mcp": {
      "command": "/absolute/path/to/echo-mcp/target/debug/echo-mcp",
      "args": ["stdio"]
    }
  }
}
```
