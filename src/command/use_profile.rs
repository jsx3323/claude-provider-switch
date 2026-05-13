use std::path::Path;

use crate::error::CsError;
use crate::output;
use crate::store::{validate_name, read_profile, merge_env_to_settings, write_current};

pub fn run(name: &str, project: &Path) -> Result<(), CsError> {
    validate_name(name)?;

    let env_values = read_profile(name)?;
    let changed = merge_env_to_settings(project, &env_values)?;
    write_current(project, name)?;

    output::success(&format!("Switched to profile '{}'", name));
    for key in &changed {
        output::info(&format!("  {} = {}", key, env_values.get(key).unwrap()));
    }
    Ok(())
}
