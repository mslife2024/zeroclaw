# ZeroClaw Commands Reference

This reference is derived from the current CLI surface (`zeroclaw --help`).

Last verified: **April 15, 2026**.

## Global options

- `--config-dir <DIR>` — alternate ZeroClaw config/workspace root (accepted on the top-level command and propagated to subcommands).

## Top-Level Commands

Order matches `zeroclaw --help`.

| Command | Purpose |
|---|---|
| `onboard` | Initialize workspace/config quickly or interactively |
| `agent` | Run interactive chat or single-message mode |
| `gateway` | Start or manage the HTTP/WebSocket gateway (webhooks, pairing, websockets) |
| `daemon` | Start supervised runtime (gateway + channels + optional heartbeat/scheduler) |
| `service` | Manage user-level OS service lifecycle |
| `doctor` | Run diagnostics for daemon, scheduler, and channel freshness |
| `status` | Print current configuration and system summary |
| `estop` | Engage/resume emergency stop levels and inspect estop state |
| `cron` | Manage scheduled tasks |
| `models` | Refresh provider model catalogs |
| `providers` | List provider IDs, aliases, and active provider |
| `channel` | Manage channels and channel health checks |
| `integrations` | Browse 50+ integrations |
| `skills` | List/install/remove skills |
| `migrate` | Import from external runtimes (currently OpenClaw) |
| `auth` | Manage provider subscription authentication profiles (OAuth, tokens, profiles) |
| `hardware` | Discover and introspect USB hardware |
| `peripheral` | Configure and flash peripherals |
| `memory` | List, get, clear, or summarize stored agent memory |
| `shell` | Set the shell execution profile in `config.toml` (restart required) |
| `mcp` | Run ZeroClaw as an MCP tool server (stdio or HTTP) |
| `config` | Export machine-readable config schema |
| `update` | Check for and install binary releases |
| `self-test` | Run installation self-tests (optional `--quick` to skip network) |
| `completions` | Generate shell completion scripts to stdout |
| `hands` | List or run autonomous hand packages under `~/.zeroclaw/hands/` |
| `desktop` | Launch or install the companion desktop app |

Builds compiled with the **`plugins-wasm`** Cargo feature also expose `plugin` (WASM plugin lifecycle); it is omitted from stock release help.

## Command Groups

### `onboard`

- `zeroclaw onboard`
- `zeroclaw onboard --channels-only`
- `zeroclaw onboard --force`
- `zeroclaw onboard --reinit`
- `zeroclaw onboard --api-key <KEY> --provider <ID> --memory <sqlite|lucid|markdown|none>`
- `zeroclaw onboard --api-key <KEY> --provider <ID> --model <MODEL_ID> --memory <sqlite|lucid|markdown|none>`
- `zeroclaw onboard --api-key <KEY> --provider <ID> --model <MODEL_ID> --memory <sqlite|lucid|markdown|none> --force`

`onboard` safety behavior:

- If `config.toml` already exists, onboarding offers two modes:
  - Full onboarding (overwrite `config.toml`)
  - Provider-only update (update provider/model/API key while preserving existing channels, tunnel, memory, hooks, and other settings)
- In non-interactive environments, existing `config.toml` causes a safe refusal unless `--force` is passed.
- Use `zeroclaw onboard --channels-only` when you only need to rotate channel tokens/allowlists.
- Use `zeroclaw onboard --reinit` to start fresh. This backs up your existing config directory with a timestamp suffix and creates a new configuration from scratch.

### `agent`

- `zeroclaw agent`
- `zeroclaw agent -m "Hello"`
- `zeroclaw agent --provider <ID> --model <MODEL> --temperature <0.0-2.0>`
- `zeroclaw agent --peripheral <board:path>`

Tip:

- In interactive chat, you can ask for route changes in natural language (for example “conversation uses kimi, coding uses gpt-5.3-codex”); the assistant can persist this via tool `model_routing_config`.

### `gateway` / `daemon`

Gateway:

- `zeroclaw gateway` — if no subcommand is given, starts the gateway using **`[gateway].host` / `[gateway].port`** from config only (no extra CLI flags on the bare command).
- `zeroclaw gateway start [--port <PORT>] [--host <HOST>]`
- `zeroclaw gateway restart [--port <PORT>] [--host <HOST>]` — tries graceful shutdown of an existing instance on that address, then starts.
- `zeroclaw gateway get-paircode [--new]` — read or rotate pairing code from a **running** gateway.

Daemon:

- `zeroclaw daemon [--host <HOST>] [--port <PORT>]` — full long-running runtime (uses gateway host/port overrides when passed).

### `mcp`

- `zeroclaw mcp serve` — stdio MCP (default; newline-delimited JSON-RPC)
- `zeroclaw mcp serve --allow-tool <NAME>` — add a tool to the allowlist (repeatable; merged with `[mcp_serve].allowed_tools`)
- `zeroclaw mcp serve --transport http [--bind <ADDR>] [--port <PORT>]` — HTTP `POST /mcp` (see [`mcp-serve.md`](../../mcp-serve.md) and `[mcp_serve]` in config)

### `shell`

- `zeroclaw shell profile safe` — set `[shell].profile` to `safe` (writes `config.toml`)
- `zeroclaw shell profile balanced` / `zeroclaw shell profile autonomous` — same for built-in tiers
- Custom ids must exist under `[[shell.profiles]]` in config (validated before save)

Restart the gateway or agent after changing profile; the engine reads the profile at process start only.

### `estop`

- `zeroclaw estop` (engage `kill-all`)
- `zeroclaw estop --level network-kill`
- `zeroclaw estop --level domain-block --domain "*.chase.com" [--domain "*.paypal.com"]`
- `zeroclaw estop --level tool-freeze --tool shell [--tool browser]`
- `zeroclaw estop status`
- `zeroclaw estop resume`
- `zeroclaw estop resume --network`
- `zeroclaw estop resume --domain "*.chase.com"`
- `zeroclaw estop resume --tool shell`
- `zeroclaw estop resume --otp <123456>`

Notes:

- `estop` commands require `[security.estop].enabled = true`.
- When `[security.estop].require_otp_to_resume = true`, `resume` requires OTP validation.
- OTP prompt appears automatically if `--otp` is omitted.

### `service`

- `zeroclaw service install`
- `zeroclaw service start`
- `zeroclaw service stop`
- `zeroclaw service restart`
- `zeroclaw service status`
- `zeroclaw service uninstall`

Service commands accept `--service-init auto|systemd|openrc` (default `auto`) to pin the init backend.

### `cron`

- `zeroclaw cron list`
- `zeroclaw cron add <expr> [--tz <IANA_TZ>] [--agent] [--allowed-tool <NAME> …] <command-or-prompt>`
- `zeroclaw cron add-at <rfc3339_timestamp> [--agent] [--allowed-tool <NAME> …] <command-or-prompt>`
- `zeroclaw cron add-every <every_ms> [--agent] [--allowed-tool <NAME> …] <command-or-prompt>`
- `zeroclaw cron once <delay> [--agent] [--allowed-tool <NAME> …] <command-or-prompt>` — delay examples: `30m`, `2h`, `1d`
- `zeroclaw cron update <id> [--expression <EXPR>] [--tz <TZ>] [--command <CMD>] [--name <NAME>] [--allowed-tool <NAME> …]`
- `zeroclaw cron remove <id>`
- `zeroclaw cron pause <id>`
- `zeroclaw cron resume <id>`

Notes:

- Mutating schedule/cron actions require `cron.enabled = true`.
- Use `--agent` so the payload is treated as an **agent prompt** instead of a shell string (repeatable `--allowed-tool` applies only to agent jobs).
- Shell command payloads for schedule creation (`add` / `add-at` / `add-every` / `once`) are validated by security command policy before job persistence. The scheduler also applies **`[shell]` Safe-tier** string checks (`[shell.safe].forbidden_paths` + null-byte rule), independent of `shell.profile`, while using `shell.timeout_secs` and `shell.login_shell` when the job runs.

### `models`

- `zeroclaw models refresh`
- `zeroclaw models refresh --provider <ID>`
- `zeroclaw models refresh --force`

`models refresh` currently supports live catalog refresh for provider IDs: `openrouter`, `openai`, `anthropic`, `groq`, `mistral`, `deepseek`, `xai`, `together-ai`, `gemini`, `ollama`, `llamacpp`, `sglang`, `vllm`, `astrai`, `venice`, `fireworks`, `cohere`, `moonshot`, `glm`, `zai`, `qwen`, and `nvidia`.

### `doctor`

- `zeroclaw doctor`
- `zeroclaw doctor query-engine` — in-process QueryEngine transition tail, last system-prompt assembly, layered-memory selector stats (when enabled), last post-compaction **memory injection** timestamp, and a short preview of the latest **session-memory summary** from consolidation (process-local).
- `zeroclaw doctor models [--provider <ID>] [--use-cache]`
- `zeroclaw doctor traces [--limit <N>] [--event <TYPE>] [--contains <TEXT>]`
- `zeroclaw doctor traces --id <TRACE_ID>`
- `zeroclaw doctor long-run [HAND]` — optional `HAND` is the TOML stem under `~/.zeroclaw/hands` (omit to scan every hand). For each selected hand, checks coordinator scratchpad freshness (`decisions.md` / `final_summary.md`), workspace AutoMemory index age when `[memory.layered]` is on, and whether the assembled hand system prompt still contains `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` (Phase 1 cache split).

`doctor traces` reads runtime tool/model diagnostics from `observability.runtime_trace_path`.

### `channel`

- `zeroclaw channel list`
- `zeroclaw channel start`
- `zeroclaw channel doctor`
- `zeroclaw channel bind-telegram <IDENTITY>`
- `zeroclaw channel add <type> <json>`
- `zeroclaw channel remove <name>`

Runtime in-chat commands (Telegram/Discord while channel server is running):

- `/models`
- `/models <provider>`
- `/model`
- `/model <model-id>`
- `/new`

Channel runtime also watches `config.toml` and hot-applies updates to:
- `default_provider`
- `default_model`
- `default_temperature`
- `api_key` / `api_url` (for the default provider)
- `reliability.*` provider retry settings

`add/remove` currently route you back to managed setup/manual config paths (not full declarative mutators yet).

### `integrations`

- `zeroclaw integrations info <name>`

### `skills`

- `zeroclaw skills list`
- `zeroclaw skills audit <source_or_name>`
- `zeroclaw skills install <source>`
- `zeroclaw skills remove <name>`

`<source>` accepts git remotes (`https://...`, `http://...`, `ssh://...`, and `git@host:owner/repo.git`) or a local filesystem path.

`skills install` always runs a built-in static security audit before the skill is accepted. The audit blocks:
- symlinks inside the skill package
- script-like files (`.sh`, `.bash`, `.zsh`, `.ps1`, `.bat`, `.cmd`)
- high-risk command snippets (for example pipe-to-shell payloads)
- markdown links that escape the skill root, point to remote markdown, or target script files

Use `skills audit` to manually validate a candidate skill directory (or an installed skill by name) before sharing it.

Skill manifests (`SKILL.toml`) support `prompts` and `[[tools]]`; both are injected into the agent system prompt at runtime, so the model can follow skill instructions without manually reading skill files.

### `migrate`

- `zeroclaw migrate openclaw [--source <path>] [--dry-run]`

### `auth`

- `zeroclaw auth login --provider <openai-codex|gemini> [--profile <NAME>] [--device-code]`
- `zeroclaw auth login --provider openai-codex --import <PATH>` (import existing `auth.json`; conflicts with `--device-code`)
- `zeroclaw auth paste-redirect --provider openai-codex [--profile <NAME>] [--input <URL_OR_CODE>]`
- `zeroclaw auth paste-token --provider anthropic [--profile <NAME>] [--token <VALUE>] [--auth-kind <authorization|api-key>]` (token omitted → interactive prompt)
- `zeroclaw auth setup-token --provider anthropic [--profile <NAME>]` — alias for `paste-token` oriented at interactive setup
- `zeroclaw auth refresh --provider openai-codex [--profile <NAME>]`
- `zeroclaw auth logout --provider <NAME> [--profile <NAME>]`
- `zeroclaw auth use --provider <NAME> --profile <NAME>`
- `zeroclaw auth list`
- `zeroclaw auth status`

Use `zeroclaw auth <subcommand> --help` for the full flag set.

### `memory`

- `zeroclaw memory list` (filters: see `zeroclaw memory list --help`)
- `zeroclaw memory get <key>`
- `zeroclaw memory stats`
- `zeroclaw memory clear` (scopes and `--yes`; see `--help`)

### `config`

- `zeroclaw config schema`

`config schema` prints a JSON Schema (draft 2020-12) for the full `config.toml` contract to stdout.

### `update`

- `zeroclaw update` — download and install latest release (with confirmation)
- `zeroclaw update --check` — check only
- `zeroclaw update --force` — skip confirmation
- `zeroclaw update --version <SEMVER>` — install a specific version

### `self-test`

- `zeroclaw self-test` — full suite (includes network-oriented checks when available)
- `zeroclaw self-test --quick` — skip network checks

### `hands`

- `zeroclaw hands list`
- `zeroclaw hands run <name>` — `name` is the `name` field from a hand TOML under `~/.zeroclaw/hands/`

### `desktop`

- `zeroclaw desktop` — launch the companion app
- `zeroclaw desktop --install` — download and install the pre-built app for this platform

### `completions`

- `zeroclaw completions bash`
- `zeroclaw completions fish`
- `zeroclaw completions zsh`
- `zeroclaw completions powershell`
- `zeroclaw completions elvish`

`completions` is stdout-only by design so scripts can be sourced directly without log/warning contamination.

### `hardware`

- `zeroclaw hardware discover`
- `zeroclaw hardware introspect <path>`
- `zeroclaw hardware info [--chip <chip_name>]`

### `peripheral`

- `zeroclaw peripheral list`
- `zeroclaw peripheral add <board> <path>`
- `zeroclaw peripheral flash [--port <serial_port>]`
- `zeroclaw peripheral setup-uno-q [--host <ip_or_host>]`
- `zeroclaw peripheral flash-nucleo`

## Validation Tip

To verify docs against your current binary quickly:

```bash
zeroclaw --help
zeroclaw --config-dir /path/to/workspace --help
zeroclaw <command> --help
```
