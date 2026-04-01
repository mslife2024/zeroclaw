# ZeroClaw as an MCP server

Expose a **curated** subset of ZeroClaw tools over the [Model Context Protocol](https://modelcontextprotocol.io/) (`2024-11-05`), for editors and other MCP clients.

## Security defaults

- **Allowlist only** — tools come from `[mcp_serve].allowed_tools` and `zeroclaw mcp serve --allow-tool`. If both are empty, the safe default is `memory_recall` and `file_read` only.
- **Stricter preset** — unless `[mcp_serve].relax_tool_policy = true`, only read-oriented tools from the built-in safe list may appear on the allowlist.
- **HTTP** — without `[mcp_serve].auth_token`, the server **refuses** to bind to non-loopback addresses (use `127.0.0.1` / `::1` / `localhost`, or set a token). With a token, require `Authorization: Bearer <token>` on HTTP routes (same style as the gateway REST API).

## Transports

### Stdio (default)

One JSON-RPC object per line on stdin; responses on stdout.

```bash
zeroclaw mcp serve
# or explicitly:
zeroclaw mcp serve --transport stdio
```

### HTTP

`POST /mcp` with a single JSON-RPC request body; JSON-RPC response in the body. `GET /health` is unauthenticated (for probes). `GET /` returns a small JSON descriptor when authorized (same rules as `POST`).

```bash
zeroclaw mcp serve --transport http --bind 127.0.0.1 --port 8787
```

Defaults for `--bind` / `--port` come from `[mcp_serve].http_bind` and `[mcp_serve].http_port` (default port `8787`).

## Client snippets

### Cursor / VS Code (stdio)

Add an MCP server entry that runs the binary with stdio transport (paths are examples):

```json
{
  "mcpServers": {
    "zeroclaw": {
      "command": "zeroclaw",
      "args": ["mcp", "serve", "--transport", "stdio"]
    }
  }
}
```

### Claude Desktop (stdio)

In Claude Desktop’s MCP config, use the same `command` / `args` pattern as above for your platform’s `zeroclaw` path.

### curl (HTTP, local)

```bash
curl -sS -X POST "http://127.0.0.1:8787/mcp" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}'
```

With `[mcp_serve].auth_token` set:

```bash
curl -sS -X POST "http://127.0.0.1:8787/mcp" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}'
```

## Config (`[mcp_serve]`)

| Key | Role |
|-----|------|
| `allowed_tools` | Tool names to expose (merged with `--allow-tool`). |
| `tool_timeout_secs` | Per-tool execution timeout. |
| `relax_tool_policy` | Allow any registered tool name on the allowlist when `true` (review first). |
| `http_bind` | Default HTTP listen address. |
| `http_port` | Default HTTP listen port. |
| `auth_token` | Optional bearer secret for HTTP; enables non-loopback binds when set. |

See [config-reference.md](reference/api/config-reference.md) for the full schema export.
