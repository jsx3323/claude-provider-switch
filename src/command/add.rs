use crate::cli::validate_name;
use crate::error::CsError;
use crate::input;
use crate::output;
use crate::store::{save_profile, profile_path, derive_default_models,
                   KEY_BASE_URL, KEY_API_KEY, KEY_MODEL, KEY_SMALL_FAST_MODEL,
                   KEY_SUBAGENT_MODEL, KEY_EFFORT_LEVEL};

pub fn run(name: &str, force: bool) -> Result<(), CsError> {
    validate_name(name)?;

    let existed = profile_path(name).exists();
    if !force && existed {
        return Err(CsError::ProfileExists { name: name.into() });
    }

    let base_url = input::prompt_required(KEY_BASE_URL)?;
    let api_key = input::prompt_required(KEY_API_KEY)?;
    let model = input::prompt_required(KEY_MODEL)?;

    let defaults = derive_default_models(&model);
    let mut env = serde_json::Map::new();
    env.insert(KEY_BASE_URL.into(), serde_json::Value::String(base_url));
    env.insert(KEY_API_KEY.into(), serde_json::Value::String(api_key));
    env.insert(KEY_MODEL.into(), serde_json::Value::String(model.clone()));

    for (key, default_val) in &defaults {
        match input::prompt_optional(key, default_val)? {
            None => {
                env.insert(key.clone(), serde_json::Value::String(default_val.clone()));
                output::info(&format!("  → auto-derived: {}", default_val));
            }
            Some(val) => {
                env.insert(key.clone(), serde_json::Value::String(val));
            }
        }
    }

    // SUBAGENT_MODEL 默认继承 SMALL_FAST_MODEL，EFFORT_LEVEL 默认 high
    let small_fast = env.get(KEY_SMALL_FAST_MODEL).and_then(|v| v.as_str()).unwrap_or(&model).to_string();
    match input::prompt_optional(KEY_SUBAGENT_MODEL, &small_fast)? {
        None => {
            env.insert(KEY_SUBAGENT_MODEL.into(), serde_json::Value::String(small_fast.clone()));
            output::info(&format!("  → inherited from {}: {}", KEY_SMALL_FAST_MODEL, small_fast));
        }
        Some(val) => {
            env.insert(KEY_SUBAGENT_MODEL.into(), serde_json::Value::String(val));
        }
    }
    match input::prompt_optional(KEY_EFFORT_LEVEL, "high")? {
        None => {
            env.insert(KEY_EFFORT_LEVEL.into(), serde_json::Value::String("high".to_string()));
        }
        Some(val) => {
            env.insert(KEY_EFFORT_LEVEL.into(), serde_json::Value::String(val));
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
