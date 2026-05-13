use thiserror::Error;

#[derive(Error, Debug)]
pub enum CsError {
    #[error("Profile '{name}' not found. Available: {available:?}")]
    ProfileNotFound {
        name: String,
        available: Vec<String>,
    },

    #[error("Profile '{name}' already exists. Use --force to overwrite.")]
    ProfileExists { name: String },

    #[error("No .claude/settings.local.json found in {path}")]
    SettingsNotFound { path: String },

    #[error("No active profile for this project")]
    NoActiveProfile,

    #[error("Invalid profile name '{name}'. Use only letters, digits, hyphens, and underscores.")]
    InvalidProfileName { name: String },

    #[error("I/O error at {path}: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },

    #[error("Invalid JSON in {path}: {source}")]
    Json {
        path: String,
        source: serde_json::Error,
    },
}

impl CsError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CsError::ProfileNotFound { .. } => 1,
            CsError::ProfileExists { .. } => 2,
            CsError::SettingsNotFound { .. } => 3,
            CsError::NoActiveProfile => 4,
            CsError::InvalidProfileName { .. } => 5,
            CsError::Io { .. } => 6,
            CsError::Json { .. } => 7,
        }
    }

    pub fn hint(&self) -> Option<String> {
        match self {
            CsError::SettingsNotFound { .. } => {
                Some("Run 'claude' first to initialize the project, then use 'claude-switch add'".into())
            }
            CsError::NoActiveProfile => {
                Some("Use 'claude-switch add <name>' to create and activate a profile".into())
            }
            _ => None,
        }
    }
}

pub fn io_err(path: &str, source: std::io::Error) -> CsError {
    CsError::Io { path: path.into(), source }
}

pub fn json_err(path: &str, source: serde_json::Error) -> CsError {
    CsError::Json { path: path.into(), source }
}