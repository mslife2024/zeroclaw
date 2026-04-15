# Changelog

## [Unreleased]

### Breaking changes

- The tool-call loop and WebSocket streaming path now use `TurnEventSink` instead of a raw `String` delta channel. Downstream code that called `run_tool_call_loop` with an `mpsc::Sender<String>` must switch to `mpsc::Sender<TurnEventSink>` (see `DeltaText` vs `Emit` in `src/agent/agent.rs`).
- The `query_engine_v2` feature flag and legacy non-v2 branches were removed earlier; only the QueryEngine-orchestrated path remains. After `git pull`, run `cargo clean && cargo build` if you hit stale incremental-build errors.

### Improvements

- The tool-call loop emits structured `TurnEvent` values (`ToolCall`, `ToolResult`) alongside draft progress strings so Web and channel consumers can share one ordering-preserving stream.
- Optional **layered memory** (`[memory.layered]` in config): AutoMemory topic files + SessionMemory turn markdown under `~/.zeroclaw/`, selective injection into the system prompt dynamic tail, `zeroclaw doctor query-engine` stats, and filesystem writes piggybacking on the existing consolidation pass when `memory.auto_save` is on. Documented in `docs/reference/api/config-reference.md`.
- **Memory consolidation** is now **awaited** on the main QueryEngine / agent turn path (gateway agent path included); the `MemoryConsolidationHook` remains registered for config compatibility but is a no-op so consolidation is not invoked twice. A post-prune **memory reload** markdown fragment (session-memory digest + optional AutoMemory index snippet) can merge into the system prompt dynamic tail after compaction.
- **`zeroclaw doctor long-run [HAND]`** checks coordinator hands: scratchpad mtimes under `~/.zeroclaw/hands/<name>/`, workspace AutoMemory index staleness when layered memory is enabled, and presence of `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` in the assembled hand prompt. `zeroclaw doctor query-engine` also prints last memory-injection timing and a short session-memory summary preview.
