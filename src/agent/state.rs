//! Explicit engine state for the QueryEngine orchestration path.

/// Reason for the last state transition (auditing / diagnostics).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionReason {
    BeginTurn,
    PreModelCompaction,
    ModelCall,
    ToolRound,
    PostToolHooks,
    RetryAfterCompaction,
    BudgetHalt,
    Cancelled,
    LoopDetectorInterrupt,
    ModelSwitch,
    TurnComplete,
    /// Turn ended with an error (non-cancel).
    TurnError,
}

/// One transition step recorded when `query_engine_v2` is enabled.
#[derive(Debug, Clone)]
pub struct TurnTransition {
    pub reason: TransitionReason,
    pub detail: Option<String>,
}

/// Lightweight turn-local state (iteration is owned by the tool loop).
#[derive(Debug, Clone, Default)]
pub struct EngineState {
    pub iteration: usize,
    pub last_transition: Option<TransitionReason>,
}
