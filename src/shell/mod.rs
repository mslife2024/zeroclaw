//! Unified shell execution engine (profile-driven).

pub mod engine;
mod env;
pub mod pipeline;
pub mod profiles;

pub use engine::ShellEngine;
