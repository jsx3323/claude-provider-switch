use std::path::{Path, PathBuf};
use std::fs;
use std::env;

use serde_json::Value;
use crate::error::{CsError, io_err, json_err};

pub const KEY_BASE_URL: &str = "ANTHROPIC_BASE_URL";
pub const KEY_API_KEY: &str = "ANTHROPIC_API_KEY";
pub const KEY_MODEL: &str = "ANTHROPIC_MODEL";
pub const KEY_SMALL_FAST_MODEL: &str = "ANTHROPIC_SMALL_FAST_MODEL";
pub const KEY_DEFAULT_HAIKU: &str = "ANTHROPIC_DEFAULT_HAIKU_MODEL";
pub const KEY_DEFAULT_SONNET: &str = "ANTHROPIC_DEFAULT_SONNET_MODEL";
pub const KEY_DEFAULT_OPUS: &str = "ANTHROPIC_DEFAULT_OPUS_MODEL";

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

pub fn validate_name(name: &str) -> Result<(), CsError> {
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(CsError::InvalidProfileName { name: name.into() });
    }
    Ok(())
}

pub fn list_profiles() -> Result<Vec<String>, CsError> {
    let dir = profiles_dir();
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    let entries = fs::read_dir(&dir).map_err(|e| io_err(&dir, e))?;
    for entry in entries {
        let entry = entry.map_err(|e| io_err(&dir, e))?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                names.push(name.to_string());
            }
        }
    }
    names.sort();
    Ok(names)
}

pub fn read_current(project: &Path) -> Result<Option<String>, CsError> {
    let path = project_current_path(project);
    match fs::read_to_string(&path) {
        Ok(content) => Ok(Some(content.trim().to_string())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(io_err(&path, e)),
    }
}

pub fn write_current(project: &Path, name: &str) -> Result<(), CsError> {
    let path = project_current_path(project);
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir).map_err(|e| io_err(dir, e))?;
    fs::write(&path, name).map_err(|e| io_err(&path, e))?;
    Ok(())
}

pub fn clear_current(project: &Path) -> Result<(), CsError> {
    let path = project_current_path(project);
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(io_err(&path, e)),
    }
}

pub fn read_profile(name: &str) -> Result<Value, CsError> {
    let path = profile_path(name);
    let content = fs::read_to_string(&path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            let available = list_profiles().unwrap_or_default();
            CsError::ProfileNotFound { name: name.into(), available }
        } else {
            io_err(&path, e)
        }
    })?;
    serde_json::from_str(&content).map_err(|e| json_err(&path, e))
}

pub fn save_profile(name: &str, content: &Value) -> Result<(), CsError> {
    validate_name(name)?;
    let dir = profiles_dir();
    fs::create_dir_all(&dir).map_err(|e| io_err(&dir, e))?;
    let path = profile_path(name);
    let json = serde_json::to_string_pretty(content).map_err(|e| json_err(&path, e))?;
    fs::write(&path, json).map_err(|e| io_err(&path, e))?;
    Ok(())
}

pub fn delete_profile(name: &str) -> Result<(), CsError> {
    let path = profile_path(name);
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let available = list_profiles().unwrap_or_default();
            Err(CsError::ProfileNotFound { name: name.into(), available })
        }
        Err(e) => Err(io_err(&path, e)),
    }
}

pub fn read_settings_local(project: &Path) -> Result<Value, CsError> {
    let path = settings_local_path(project);
    let content = fs::read_to_string(&path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            CsError::SettingsNotFound { path: project.display().to_string() }
        } else {
            io_err(&path, e)
        }
    })?;
    serde_json::from_str(&content).map_err(|e| json_err(&path, e))
}

pub fn write_settings_local(project: &Path, content: &Value) -> Result<(), CsError> {
    let path = settings_local_path(project);
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir).map_err(|e| io_err(dir, e))?;
    let json = serde_json::to_string_pretty(content).map_err(|e| json_err(&path, e))?;
    fs::write(&path, json).map_err(|e| io_err(&path, e))?;
    Ok(())
}

pub fn is_claude_env_key(key: &str) -> bool {
    key.starts_with("ANTHROPIC_")
}

pub fn derive_default_models(model: &str) -> [(String, String); 4] {
    [
        (KEY_SMALL_FAST_MODEL.into(), model.into()),
        (KEY_DEFAULT_HAIKU.into(), model.into()),
        (KEY_DEFAULT_SONNET.into(), model.into()),
        (KEY_DEFAULT_OPUS.into(), model.into()),
    ]
}

pub fn merge_env_to_settings(project: &Path, env_values: &Value) -> Result<Vec<String>, CsError> {
    let mut settings = read_settings_local(project)?;
    let profile_env = env_values.as_object().unwrap();

    let settings_env = settings
        .as_object_mut()
        .unwrap()
        .entry("env")
        .or_insert_with(|| Value::Object(serde_json::Map::new()));
    let env_obj = settings_env.as_object_mut().unwrap();

    env_obj.retain(|k, _| !is_claude_env_key(k));

    let mut written = Vec::new();
    for (key, value) in profile_env {
        env_obj.insert(key.clone(), value.clone());
        written.push(key.clone());
    }

    write_settings_local(project, &settings)?;
    Ok(written)
}

pub fn read_current_env(project: &Path) -> Result<Value, CsError> {
    let settings = read_settings_local(project)?;
    let env = settings.get("env").cloned().unwrap_or(Value::Object(serde_json::Map::new()));
    let env_obj = env.as_object().unwrap();
    let filtered: serde_json::Map<String, Value> = env_obj
        .iter()
        .filter(|(k, _)| is_claude_env_key(k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    Ok(Value::Object(filtered))
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