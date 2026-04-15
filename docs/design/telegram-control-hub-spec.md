# Telegram control hub — design specification

This document describes the **implemented** Telegram control surface in ZeroClaw: what is parsed before the LLM, how it maps to behavior, and the security model.

## Goals

- Optional **pre-LLM** handling for a fixed, auditable command surface on Telegram only.
- **Opt-in** via config so a deployed bot cannot silently become a remote admin console.
- **Telegram Bot API compliance** for `setMyCommands` (`command`: 1–32 characters from `[A-Za-z0-9_]`, `description`: ≤256 characters).

## Configuration

Under `[channels_config.telegram]`:

| Key | Default | Purpose |
| --- | --- | --- |
| `control_hub_enabled` | `false` | When `true`, messages starting with `/<control_hub_prefix>` are dispatched by the control hub before the agent. |
| `control_hub_prefix` | `z` | Bot command token (no slash). Invalid values (empty, wrong charset, length) disable hub dispatch while still allowing other features. |

## Runtime slash commands (all channels)

Handled in `parse_runtime_command` in `src/channels/mod.rs` (not the control hub):

- `/new` — new session
- `/models`, `/model`, `/config` — provider/model routing (where supported by channel)

Telegram additionally receives **`setMyCommands`** entries for `new`, `models`, `model`, `config`, plus the hub token when the hub is enabled, plus optional per-skill menu rows (see below).

## Control hub syntax

- Form: `/<prefix>` or `/<prefix>@BotUserName` followed by arguments.
- Example (default prefix): `/z skills list`, `/z@MyBot channel doctor`.

## Dispatch mapping

The hub dispatches the first argument as a **topic**, then forwards to the same `zeroclaw` CLI the operator would run on the host (via `std::env::current_exe()` and `ZEROCLAW_CONFIG_DIR`), except:

- **`mcp serve`** — rejected (long-running server; must use host CLI).
- **`estop resume`** — rejected when interactive OTP / confirmation may be required; use host CLI.

`skills list` is answered from the in-process skill registry (no subprocess) for efficiency.

## Skill metadata: `user_invocable`

Skills may declare:

- In **SKILL.md** YAML frontmatter: `user_invocable: true` or `user-invocable: true` (alias).
- In **SKILL.toml** under `[skill]`: `user_invocable = true`.

When set, the sanitized skill `name` may be added as an extra **BotCommand** row (if it does not collide with built-ins). This is **menu discovery only**; behavior is still defined by the hub dispatcher and CLI parity, not by executing arbitrary skill markdown as code.

## Security

- **Allowlist**: Existing Telegram `allowed_users` and pairing rules apply before any handler runs.
- **Default off**: `control_hub_enabled = false` means `/z …` is normal chat text for the agent.
- **Host parity**: Successful hub actions that mutate config (e.g. `shell profile`) have the same effect as CLI; restart requirements match CLI messaging.

## Non-goals

- Replacing BotFather for base bot identity setup (token still comes from config).
- Guaranteeing every CLI subflag is exposed from Telegram; the hub covers a curated subset.
