# Reference: doc paths and inventory

## Canonical operator references (prefer these paths)

| Topic | Path |
|-------|------|
| Config / TOML | `docs/reference/api/config-reference.md` |
| CLI | `docs/reference/cli/commands-reference.md` |
| Channels | `docs/reference/api/channels-reference.md` |
| Providers | `docs/reference/api/providers-reference.md` |
| MCP HTTP/stdio server | `docs/mcp-serve.md` |
| Shell profiles (guide) | `docs/setup-guides/shell-profiles.md` |
| Example full config | `scripts/example-conf.toml` |

## Hub / inventory

- `docs/README.md` — docs entry hub.
- `docs/maintainers/docs-inventory.md` — classifies many docs; **verify paths** against the tree before trusting every row (inventory can drift).

## Localized mirrors (optional)

Examples: `docs/reference/api/config-reference.vi.md`, `docs/i18n/zh-CN/...`. Sync only when the user requests locale updates.

## Feature flags

Search `Cargo.toml` `[features]` and `cfg(feature = "...")` in code when documenting optional behavior (e.g. shell extras behind a feature name).
