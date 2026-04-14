//! Phased compaction before LLM calls (`query_engine_v2`) and reactive trimming
//! after context-related failures.

use crate::agent::context_analyzer;
use crate::agent::history_pruner::{self, HistoryPrunerConfig};
use crate::providers::ChatMessage;
use anyhow::Result;

/// Why a compaction pass is running (drives aggressiveness).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompactionTrigger {
    #[default]
    Routine,
    /// Prior LLM attempt failed with a context-size style error.
    ReactiveContextError,
}

/// Per-iteration context for optional analyzer stages.
#[derive(Debug, Clone)]
pub struct CompactionContext {
    pub iteration: usize,
    pub last_tool_names: Vec<String>,
    pub trigger: CompactionTrigger,
    /// When true, run `analyze_turn_context` to log suggested tools (no filtering yet).
    pub log_context_signals: bool,
}

impl CompactionContext {
    #[must_use]
    pub fn new(iteration: usize, last_tool_names: Vec<String>, trigger: CompactionTrigger) -> Self {
        Self {
            iteration,
            last_tool_names,
            trigger,
            log_context_signals: false,
        }
    }
}

/// Run pruning (and optional context analysis) before an LLM call.
pub fn run_pre_llm_phases(
    history: &mut Vec<ChatMessage>,
    pruning: &HistoryPrunerConfig,
    ctx: &CompactionContext,
) -> Result<()> {
    if pruning.enabled {
        let _stats = history_pruner::prune_history(history, pruning);
    }

    if ctx.log_context_signals {
        let signals = context_analyzer::analyze_turn_context(
            history,
            "",
            ctx.iteration,
            &ctx.last_tool_names,
        );
        tracing::debug!(
            iteration = ctx.iteration,
            suggested_tools = ?signals.suggested_tools,
            history_relevant = signals.history_relevant,
            "context_analyzer signals"
        );
    }

    let _ = ctx.trigger; // reserved for future aggressiveness tuning

    Ok(())
}

/// Aggressive trim after a context-related LLM failure (best-effort).
pub fn run_reactive_compaction(
    history: &mut Vec<ChatMessage>,
    pruning: &HistoryPrunerConfig,
) -> Result<()> {
    if pruning.enabled {
        let _stats = history_pruner::prune_history(history, pruning);
    }
    Ok(())
}

/// True when the error likely reflects prompt/context limits recoverable by trimming.
#[must_use]
pub fn llm_error_suggests_context_retry(err: &anyhow::Error) -> bool {
    crate::providers::reliable::error_suggests_reactive_compaction(err)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn llm_error_suggests_context_retry_detects_window_hints() {
        let e = anyhow::anyhow!("prompt exceeds max length for this model");
        assert!(llm_error_suggests_context_retry(&e));
    }

    #[test]
    fn run_pre_llm_phases_no_panic_when_disabled() {
        let mut history = vec![
            ChatMessage::system("sys"),
            ChatMessage::user("hi".repeat(100)),
        ];
        let pruning = HistoryPrunerConfig::default();
        let ctx = CompactionContext::new(0, vec![], CompactionTrigger::Routine);
        run_pre_llm_phases(&mut history, &pruning, &ctx).unwrap();
    }
}
