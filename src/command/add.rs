use std::io;

use crate::error::CsError;
use crate::output;
use crate::store::{validate_name, save_profile, profile_path, derive_default_models, KEY_BASE_URL, KEY_API_KEY, KEY_MODEL};

pub fn run(name: &str, force: bool) -> Result<(), CsError> {
    validate_name(name)?;

    let existed = profile_path(name).exists();
    if !force && existed {
        return Err(CsError::ProfileExists { name: name.into() });
    }

    let base_url = prompt_required(KEY_BASE_URL)?;
    let api_key = prompt_required(KEY_API_KEY)?;
    let model = prompt_required(KEY_MODEL)?;

    let defaults = derive_default_models(&model);
    let mut env = serde_json::Map::new();
    env.insert(KEY_BASE_URL.into(), serde_json::Value::String(base_url));
    env.insert(KEY_API_KEY.into(), serde_json::Value::String(api_key));
    env.insert(KEY_MODEL.into(), serde_json::Value::String(model));

    for (key, default_val) in &defaults {
        let input = prompt_optional(key, default_val)?;
        if input.is_empty() {
            env.insert(key.clone(), serde_json::Value::String(default_val.clone()));
            output::info(&format!("  → auto-derived: {}", default_val));
        } else {
            env.insert(key.clone(), serde_json::Value::String(input));
        }
    }

    save_profile(name, &serde_json::Value::Object(env))?;

    if existed {
        output::success(&format!("Overwritten profile '{}'", name));
    } else {
        output::success(&format!("Created profile '{}'", name));
    }
    Ok(())
}

fn prompt_required(field: &str) -> Result<String, CsError> {
    loop {
        eprintln!("{}: ", field);
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| crate::error::io_err("stdin", e))?;
        let value = input.trim().to_string();
        if value.is_empty() {
            output::error(&format!("{} is required", field));
            continue;
        }
        return Ok(value);
    }
}

fn prompt_optional(field: &str, default: &str) -> Result<String, CsError> {
    eprintln!("{} (optional, default: {}): ", field, default);
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| crate::error::io_err("stdin", e))?;
    Ok(input.trim().to_string())
}
