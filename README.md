# echo-mcp

A tiny MCP server for testing clients and transport setup. It exposes one tool, `echo`, which returns the input message unchanged and also prints it to `stderr`.

Built with [rmcp](https://docs.rs/rmcp/latest/rmcp/).

## Installation

### Cargo

echo-mcp is published on crates.io, so you can install the latest stable build with:

```bash
cargo install --locked echo-mcp
```

If you already have `echo-mcp` installed, rerun the command with `--force` to upgrade.

### GitHub releases

Download a pre-built binary from the [release page](https://github.com/mirrorange/echo-mcp/releases)

Release assets are machine-specific, so pick the archive that matches your OS once the download page opens.

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

The same message is also written to `stderr` when the tool receives it.

## Run

```bash
echo-mcp stdio
```

```bash
echo-mcp http --bind 127.0.0.1:8000
```

The streamable HTTP endpoint will be available at `http://127.0.0.1:8000/mcp`.

## Tool

- `echo`
  Returns the provided message exactly as received, and writes it to `stderr`.

## Example MCP Client Config

```json
{
  "mcpServers": {
    "echo-mcp": {
      "command": "echo-mcp",
      "args": ["stdio"]
    }
  }
}
```
