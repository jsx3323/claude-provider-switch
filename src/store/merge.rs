use std::path::Path;

use serde_json::Value;
use crate::error::CsError;
use super::keys::is_claude_env_key;
use super::io::{read_settings_local, write_settings_local};

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
