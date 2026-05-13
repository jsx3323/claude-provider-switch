use serde_json::Value;
use crate::error::CsError;
use super::keys::is_claude_env_key;

pub fn merge_env(mut settings: Value, env_values: &Value) -> Result<(Value, Vec<String>), CsError> {
    let profile_env = env_values.as_object()
        .ok_or(CsError::MalformedJson { detail: "env_values must be a JSON object".into() })?;

    let settings_env = settings
        .as_object_mut()
        .ok_or(CsError::MalformedJson { detail: "settings must be a JSON object".into() })?
        .entry("env")
        .or_insert_with(|| Value::Object(serde_json::Map::new()));
    let env_obj = settings_env.as_object_mut()
        .ok_or(CsError::MalformedJson { detail: "\"env\" field must be a JSON object".into() })?;

    env_obj.retain(|k, _| !is_claude_env_key(k));

    let mut written = Vec::new();
    for (key, value) in profile_env {
        env_obj.insert(key.clone(), value.clone());
        written.push(key.clone());
    }

    Ok((settings, written))
}