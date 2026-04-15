//! Resolved shell profile (built-in kind + optional custom id).

use crate::config::ShellSection;

/// Built-in shell pipeline tier (custom profiles map to one of these).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellProfileKind {
    Safe,
    Balanced,
    Autonomous,
}

/// Effective profile after resolving custom ids.
#[derive(Debug, Clone)]
pub struct ResolvedShellProfile {
    pub kind: ShellProfileKind,
    pub custom_id: Option<String>,
}

/// Resolve `shell.profile` into a built-in kind (custom entries extend a built-in).
pub fn resolve_shell_profile(shell: &ShellSection) -> anyhow::Result<ResolvedShellProfile> {
    let p = shell.normalized_profile();
    let (kind, custom) = match p.as_str() {
        "safe" => (ShellProfileKind::Safe, None),
        "balanced" => (ShellProfileKind::Balanced, None),
        "autonomous" => (ShellProfileKind::Autonomous, None),
        other => {
            let entry = shell
                .profiles
                .iter()
                .find(|e| e.id.trim().eq_ignore_ascii_case(other))
                .ok_or_else(|| anyhow::anyhow!("unknown shell profile: {other}"))?;
            let k = match entry.extends_normalized().as_str() {
                "safe" => ShellProfileKind::Safe,
                "balanced" => ShellProfileKind::Balanced,
                "autonomous" => ShellProfileKind::Autonomous,
                _ => anyhow::bail!("invalid extends for custom profile {}", entry.id),
            };
            (k, Some(entry.id.trim().to_string()))
        }
    };
    Ok(ResolvedShellProfile {
        kind,
        custom_id: custom,
    })
}
