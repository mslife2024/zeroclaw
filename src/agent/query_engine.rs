//! QueryEngine v2: diagnostics + traced delegation into [`super::loop_::run_tool_call_loop_body`].

use super::state::{TurnTransition, TransitionReason};
use crate::approval::ApprovalManager;
use crate::hooks::HookRunner;
use crate::observability::Observer;
use crate::providers::{ChatMessage, Provider};
use crate::tools::Tool;
use anyhow::Result;
use std::collections::VecDeque;
use std::sync::{LazyLock, Mutex};
use tokio_util::sync::CancellationToken;

const DIAG_CAP: usize = 64;

#[derive(Debug, Clone)]
struct DiagEntry {
    pub ts: std::time::Instant,
    pub transition: TurnTransition,
}

static QUERY_ENGINE_DIAG: LazyLock<Mutex<VecDeque<DiagEntry>>> =
    LazyLock::new(|| Mutex::new(VecDeque::with_capacity(DIAG_CAP)));

pub fn record_transition(reason: TransitionReason, detail: Option<String>) {
    let mut q = QUERY_ENGINE_DIAG.lock().unwrap_or_else(|p| p.into_inner());
    if q.len() >= DIAG_CAP {
        q.pop_front();
    }
    q.push_back(DiagEntry {
        ts: std::time::Instant::now(),
        transition: TurnTransition { reason, detail },
    });
}

/// Recent transitions for `zeroclaw doctor query-engine`.
#[must_use]
pub fn drain_diagnostics() -> Vec<TurnTransition> {
    QUERY_ENGINE_DIAG
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .iter()
        .map(|e| e.transition.clone())
        .collect()
}

/// Heuristic: model may have stopped early due to output token cap — caller may append a nudge.
#[must_use]
pub fn should_request_token_continuation(
    usage: Option<&crate::providers::traits::TokenUsage>,
    output_text_chars: usize,
) -> bool {
    let Some(u) = usage else {
        return false;
    };
    let Some(out) = u.output_tokens else {
        return false;
    };
    // Without provider-reported max_output_tokens, use a conservative heuristic:
    // large billed output with almost no visible text often indicates truncation.
    out >= 900 && output_text_chars < 24
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn run_tool_call_loop_traced(
    provider: &dyn Provider,
    history: &mut Vec<ChatMessage>,
    tools_registry: &[Box<dyn Tool>],
    observer: &dyn Observer,
    provider_name: &str,
    model: &str,
    temperature: f64,
    silent: bool,
    approval: Option<&ApprovalManager>,
    channel_name: &str,
    channel_reply_target: Option<&str>,
    multimodal_config: &crate::config::MultimodalConfig,
    max_tool_iterations: usize,
    cancellation_token: Option<CancellationToken>,
    on_delta: Option<tokio::sync::mpsc::Sender<String>>,
    hooks: Option<&HookRunner>,
    excluded_tools: &[String],
    dedup_exempt_tools: &[String],
    activated_tools: Option<&std::sync::Arc<std::sync::Mutex<crate::tools::ActivatedToolSet>>>,
    model_switch_callback: Option<super::loop_::ModelSwitchCallback>,
    pacing: &crate::config::PacingConfig,
    tool_result_offload: &crate::config::ToolResultOffloadConfig,
    history_pruning: &crate::agent::history_pruner::HistoryPrunerConfig,
) -> Result<String> {
    record_transition(TransitionReason::BeginTurn, None);
    let res = super::loop_::run_tool_call_loop_body(
        provider,
        history,
        tools_registry,
        observer,
        provider_name,
        model,
        temperature,
        silent,
        approval,
        channel_name,
        channel_reply_target,
        multimodal_config,
        max_tool_iterations,
        cancellation_token,
        on_delta,
        hooks,
        excluded_tools,
        dedup_exempt_tools,
        activated_tools,
        model_switch_callback,
        pacing,
        tool_result_offload,
        history_pruning,
    )
    .await;
    match &res {
        Ok(_) => record_transition(TransitionReason::TurnComplete, None),
        Err(e) => record_transition(TransitionReason::TurnError, Some(e.to_string())),
    }
    if let (Ok(ref text), Some(hooks)) = (&res, hooks) {
        super::stop_hooks::fire_after_turn_void(hooks, channel_name, text.as_str()).await;
    }
    res
}
