use std::path::{Path, PathBuf};
use std::env;

use crate::error::{CsError, io_err};

pub fn store_dir() -> PathBuf {
    std::env::var("CLAUDE_SWITCH_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("/"))
                .join(".claude-switch")
        })
}

pub fn profiles_dir() -> PathBuf {
    store_dir().join("profiles")
}

pub fn profile_path(name: &str) -> PathBuf {
    profiles_dir().join(format!("{}.json", name))
}

pub fn project_current_path(project: &Path) -> PathBuf {
    let hash = simple_hash(project.to_string_lossy().as_ref());
    store_dir().join("projects").join(hash).join("current")
}

pub fn settings_local_path(project: &Path) -> PathBuf {
    project.join(".claude").join("settings.local.json")
}

pub fn find_project_dir() -> Result<PathBuf, CsError> {
    let cwd = env::current_dir().map_err(|e| io_err("current_dir", e))?;
    let mut dir = cwd.as_path();
    loop {
        if dir.join(".claude").exists() {
            return Ok(dir.to_path_buf());
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return Ok(cwd),
        }
    }
}

pub fn simple_hash(s: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis
    for b in s.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3); // FNV prime
    }
    format!("{:016x}", hash)
}

// dirs::home_dir 不在依赖中，自己实现
mod dirs {
    use std::path::PathBuf;
    pub fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
