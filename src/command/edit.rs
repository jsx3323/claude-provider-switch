use crate::cli::validate_name;
use crate::error::CsError;
use crate::input;
use crate::output;
use crate::store::{read_profile, save_profile, derive_default_models,
                   KEY_BASE_URL, KEY_API_KEY, KEY_MODEL, KEY_SMALL_FAST_MODEL,
                   KEY_SUBAGENT_MODEL, KEY_EFFORT_LEVEL, KEY_AUTO_COMPACT_WINDOW};

pub fn run(name: &str) -> Result<(), CsError> {
    validate_name(name)?;

    let existing = read_profile(name)?;
    let env = existing.as_object()
        .ok_or(CsError::MalformedJson { detail: "profile must be a JSON object".into() })?;

    // 从现有 profile 提取值作为默认
    let get_str = |key: &str| env.get(key).and_then(|v| v.as_str()).unwrap_or("");

    let base_url = input::prompt_with_default(KEY_BASE_URL, get_str(KEY_BASE_URL))?;
    let api_key = input::prompt_with_default(KEY_API_KEY, get_str(KEY_API_KEY))?;
    let model = input::prompt_with_default(KEY_MODEL, get_str(KEY_MODEL))?;

    let defaults = derive_default_models(&model);
    let mut new_env = serde_json::Map::new();
    new_env.insert(KEY_BASE_URL.into(), serde_json::Value::String(base_url));
    new_env.insert(KEY_API_KEY.into(), serde_json::Value::String(api_key));
    new_env.insert(KEY_MODEL.into(), serde_json::Value::String(model.clone()));

    for (key, default_val) in &defaults {
        // 现有值优先于 derive 值作为默认
        let existing_val = get_str(key);
        let display_default = if existing_val.is_empty() { default_val } else { existing_val };
        match input::prompt_optional(key, display_default)? {
            None => {
                let val = if existing_val.is_empty() { default_val.clone() } else { existing_val.to_string() };
                new_env.insert(key.clone(), serde_json::Value::String(val));
                if existing_val.is_empty() {
                    output::info(&format!("  → auto-derived: {}", default_val));
                }
            }
            Some(val) => {
                new_env.insert(key.clone(), serde_json::Value::String(val));
            }
        }
    }

    // SUBAGENT_MODEL 默认继承 SMALL_FAST_MODEL，EFFORT_LEVEL 默认 high
    let small_fast_in_new = new_env.get(KEY_SMALL_FAST_MODEL).and_then(|v| v.as_str()).unwrap_or(&model).to_string();
    let subagent_existing = get_str(KEY_SUBAGENT_MODEL);
    let subagent_default = if subagent_existing.is_empty() { &small_fast_in_new } else { subagent_existing };
    match input::prompt_optional(KEY_SUBAGENT_MODEL, subagent_default)? {
        None => {
            let val = if subagent_existing.is_empty() { small_fast_in_new.clone() } else { subagent_existing.to_string() };
            new_env.insert(KEY_SUBAGENT_MODEL.into(), serde_json::Value::String(val));
            if subagent_existing.is_empty() {
                output::info(&format!("  → inherited from {}: {}", KEY_SMALL_FAST_MODEL, small_fast_in_new));
            }
        }
        Some(val) => {
            new_env.insert(KEY_SUBAGENT_MODEL.into(), serde_json::Value::String(val));
        }
    }

    let effort_existing = get_str(KEY_EFFORT_LEVEL);
    let effort_default = if effort_existing.is_empty() { "high" } else { effort_existing };
    match input::prompt_optional(KEY_EFFORT_LEVEL, effort_default)? {
        None => {
            let val = if effort_existing.is_empty() { "high".to_string() } else { effort_existing.to_string() };
            new_env.insert(KEY_EFFORT_LEVEL.into(), serde_json::Value::String(val));
        }
        Some(val) => {
            new_env.insert(KEY_EFFORT_LEVEL.into(), serde_json::Value::String(val));
        }
    }

    // AUTO_COMPACT_WINDOW 是真正的可选 key：空输入表示清空（与 add 行为一致）
    if let Some(val) = input::prompt_optional(KEY_AUTO_COMPACT_WINDOW, get_str(KEY_AUTO_COMPACT_WINDOW))? {
        new_env.insert(KEY_AUTO_COMPACT_WINDOW.into(), serde_json::Value::String(val));
    }

    // 保留非标准 key（如 ANTHROPIC_AUTH_TOKEN 等 derive_default_models 不覆盖的 key）。
    // AUTO_COMPACT_WINDOW 用户可能主动清空，不从此处还原
    for (key, value) in env.iter() {
        if key == KEY_AUTO_COMPACT_WINDOW {
            continue;
        }
        if !new_env.contains_key(key) {
            new_env.insert(key.clone(), value.clone());
        }
    }

    save_profile(name, &serde_json::Value::Object(new_env))?;
    output::success(&format!("Updated profile '{}'", name));
    Ok(())
}