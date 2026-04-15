---
name: sync-docs-with-code
description: Aligns ZeroClaw operator documentation and example TOML with the current Rust config schema, CLI flags, and runtime behavior. Use when the user asks to update docs after code changes, when touching `src/config/schema.rs`, CLI/subcommands, gateway, shell engine, cron, tools, or `scripts/example-conf.toml`.
---

# Sync documentation with latest code

## Goal

Documentation that claims to describe **runtime contracts** must match the **implemented** behavior. Prefer small, accurate diffs over broad rewrites.

## Sources of truth (read these first)

| Area | Primary code |
|------|----------------|
| Config keys, defaults, validation | `src/config/schema.rs` (`Config::validate`, section structs) |
| CLI surface | `src/main.rs` (clap), or run `cargo run --bin zeroclaw -- --help` and `zeroclaw <cmd> --help` |
| Shell tool / profiles / cron spawn | `src/shell/engine.rs`, `src/shell/profiles.rs`, `src/shell/pipeline.rs`, `src/shell/env.rs` |
| Example full config | `scripts/example-conf.toml` (should list real sections; comments OK) |

For a wider doc index (some paths may lag), see [reference.md](reference.md).

## Workflow

1. **Scope** — From the diff or user request, list what changed (new TOML section, renamed key, new subcommand, behavior change in a tool).
2. **Verify behavior** — Read the relevant Rust; do not infer from old markdown. If behavior is behind a Cargo feature (e.g. `shell-full`), say so explicitly in docs.
3. **Update operator docs** — Touch only files that describe that surface:
   - **Config contract** → `docs/reference/api/config-reference.md` (bump **Last verified** date when you materially change it).
   - **CLI** → `docs/reference/cli/commands-reference.md`.
   - **User-facing shell profiles** → `docs/setup-guides/shell-profiles.md` and, if the comparison narrative needs it, `docs/toolings.md`.
   - **README / hub** → only if user-facing entry points mention the old behavior.
4. **Example config** — If schema or defaults changed, update the matching block in `scripts/example-conf.toml` (commented optional keys are fine).
5. **Consistency checks** — Defaults and key names in prose must match `schema.rs`. CLI examples must match clap output.
6. **i18n** — If the user asks for localized parity, mirror changes under `docs/i18n/` or `docs/reference/api/*.vi.md` etc.; otherwise English canonical refs above are enough.

## Writing rules

- Describe **what the binary does today**; mark reserved or unused config keys as **reserved** if they exist in schema but have no engine effect.
- When behavior differs by context (e.g. cron uses Safe-tier shell checks regardless of `shell.profile`), state both paths clearly.
- Avoid duplicating huge tables; cross-link `config-reference.md` from guides instead of copying every key.

## Verification

```bash
cargo run --bin zeroclaw -- --help
cargo run --bin zeroclaw -- config schema > /tmp/schema.json
```

Use stdout from `--help` to reconcile `commands-reference.md`. Use `config schema` JSON Schema export to cross-check new or renamed keys when in doubt.

## Anti-patterns

- Updating only `README.md` while leaving `docs/reference/api/config-reference.md` wrong.
- Documenting aspirational behavior (future validators, unimplemented keys) as if shipped—use **reserved** / **optional feature** language.
- Changing unrelated locales or vi docs without an explicit request.
