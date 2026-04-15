# Telegram slash commands and control hub

This guide reflects **current ZeroClaw behavior** (not third-party tutorials).

## Built-in runtime commands

On Telegram (and other channels where model switching is supported), these are handled **before** the LLM:

| Command | Purpose |
| --- | --- |
| `/new` | Clear session history for this sender/thread and start fresh. |
| `/models` | List providers or switch provider (`/models <provider>`). |
| `/model` | Show or set model id (`/model <id>`). |
| `/config` | Show routing / model summary for this chat. |

These do **not** require the control hub.

## Optional control hub (`/z` by default)

When enabled in `config.toml`:

```toml
[channels_config.telegram]
bot_token = "…"
allowed_users = ["YOUR_TELEGRAM_USER_ID"]
control_hub_enabled = true
# control_hub_prefix = "z"   # optional; default is z
```

Messages such as `/z skills list` or `/z channel doctor` are handled **before** the LLM and run curated operations (see [telegram-control-hub-spec.md](../design/telegram-control-hub-spec.md)).

**Security:** Leave `control_hub_enabled = false` unless you explicitly want Telegram senders who pass `allowed_users` (and pairing, if used) to trigger host management commands.

## Bot command menu (`setMyCommands`)

When the Telegram channel connects, ZeroClaw registers a small default command list (`new`, `models`, `model`, `config`) and, if the hub is enabled, the hub prefix. Skills with `user_invocable = true` may add additional menu rows (names are sanitized for Telegram).

## Skills: `user_invocable` and `prompt_injection_mode`

- **`user_invocable`**: Declared in `SKILL.md` / `SKILL.toml` as described in the design spec. It affects Telegram menu registration only; it does **not** by itself add new hub verbs.
- **`prompt_injection_mode`**: Global setting under `[skills]` in `config.toml` (`full` or `compact`). It is **not** read from per-skill YAML for runtime behavior.

## Misconceptions (common external guides)

- There is **no** `/commands` runtime handler in ZeroClaw; use the Telegram command menu or this doc.
- Slash routing for the hub is **not** inferred from skill prose alone; it requires `control_hub_enabled` and a valid prefix.
