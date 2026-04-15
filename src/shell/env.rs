//! Safe environment variable allowlist for shell subprocesses.

use std::collections::HashSet;

use crate::security::SecurityPolicy;

#[cfg(not(target_os = "windows"))]
const SAFE_ENV_VARS: &[&str] = &[
    "PATH", "HOME", "TERM", "LANG", "LC_ALL", "LC_CTYPE", "USER", "SHELL", "TMPDIR",
];

#[cfg(target_os = "windows")]
const SAFE_ENV_VARS: &[&str] = &[
    "PATH",
    "PATHEXT",
    "HOME",
    "USERPROFILE",
    "HOMEDRIVE",
    "HOMEPATH",
    "SYSTEMROOT",
    "SYSTEMDRIVE",
    "WINDIR",
    "COMSPEC",
    "TEMP",
    "TMP",
    "TERM",
    "LANG",
    "USERNAME",
];

fn is_valid_env_var_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

/// Environment variable names to copy from the host into the shell subprocess.
pub fn collect_allowed_shell_env_vars(security: &SecurityPolicy) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    for key in SAFE_ENV_VARS
        .iter()
        .copied()
        .chain(security.shell_env_passthrough.iter().map(|s| s.as_str()))
    {
        let candidate = key.trim();
        if candidate.is_empty() || !is_valid_env_var_name(candidate) {
            continue;
        }
        if seen.insert(candidate.to_string()) {
            out.push(candidate.to_string());
        }
    }
    out
}
