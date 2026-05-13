pub const KEY_BASE_URL: &str = "ANTHROPIC_BASE_URL";
pub const KEY_API_KEY: &str = "ANTHROPIC_API_KEY";
pub const KEY_MODEL: &str = "ANTHROPIC_MODEL";
pub const KEY_SMALL_FAST_MODEL: &str = "ANTHROPIC_SMALL_FAST_MODEL";
pub const KEY_DEFAULT_HAIKU: &str = "ANTHROPIC_DEFAULT_HAIKU_MODEL";
pub const KEY_DEFAULT_SONNET: &str = "ANTHROPIC_DEFAULT_SONNET_MODEL";
pub const KEY_DEFAULT_OPUS: &str = "ANTHROPIC_DEFAULT_OPUS_MODEL";

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
