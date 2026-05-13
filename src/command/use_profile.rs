use std::path::Path;

use crate::cli::validate_name;
use crate::error::CsError;
use crate::output;
use crate::store::{read_profile, merge_env, write_current, read_settings_local, write_settings_local};

pub fn run(name: &str, project: &Path) -> Result<(), CsError> {
    validate_name(name)?;

    let env_values = read_profile(name)?;
    let settings = read_settings_local(project)?;
    let (merged, changed) = merge_env(settings, &env_values)?;
    write_settings_local(project, &merged)?;
    write_current(project, name)?;

    output::success(&format!("Switched to profile '{}'", name));
    for key in &changed {
        output::info(&format!("  {} = {}", key, env_values.get(key).unwrap()));
    }
    Ok(())
}