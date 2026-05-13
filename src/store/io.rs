use std::path::Path;
use std::fs;

use serde_json::Value;
use crate::error::{CsError, io_err, json_err, serialization_err};
use super::path::{profiles_dir, profile_path, project_current_path, settings_local_path};
use super::keys::is_claude_env_key;

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
        if path.extension().is_some_and(|ext| ext == "json")
            && let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                names.push(name.to_string());
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
    let dir = profiles_dir();
    fs::create_dir_all(&dir).map_err(|e| io_err(&dir, e))?;
    let path = profile_path(name);
    let json = serde_json::to_string_pretty(content).map_err(|e| serialization_err(&path.display().to_string(), e))?;
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
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).map_err(|e| json_err(&path, e)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(serde_json::json!({"permissions": {"allow": [], "deny": []}, "env": {}}))
        }
        Err(e) => Err(io_err(&path, e)),
    }
}

pub fn write_settings_local(project: &Path, content: &Value) -> Result<(), CsError> {
    let path = settings_local_path(project);
    let dir = path.parent().unwrap();
    fs::create_dir_all(dir).map_err(|e| io_err(dir, e))?;
    let json = serde_json::to_string_pretty(content).map_err(|e| serialization_err(&path.display().to_string(), e))?;
    fs::write(&path, json).map_err(|e| io_err(&path, e))?;
    Ok(())
}

pub fn read_current_env(project: &Path) -> Result<Value, CsError> {
    let settings = read_settings_local(project)?;
    let env = settings.get("env").cloned().unwrap_or(Value::Object(serde_json::Map::new()));
    let env_obj = env.as_object()
        .ok_or(CsError::MalformedJson { detail: "\"env\" field must be a JSON object".into() })?;
    let filtered: serde_json::Map<String, Value> = env_obj
        .iter()
        .filter(|(k, _)| is_claude_env_key(k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    Ok(Value::Object(filtered))
}
