use crate::error::CsError;
use crate::output;
use crate::profile::{find_project_dir, validate_name, read_profile, merge_env_to_settings, write_current};

pub fn run(name: &str) -> Result<(), CsError> {
    validate_name(name)?;
    let project = find_project_dir()?;

    let env_values = read_profile(name)?;
    let changed = merge_env_to_settings(&project, &env_values)?;
    write_current(&project, name)?;

    output::success(&format!("Switched to profile '{}'", name));
    for key in &changed {
        crate::output::info(&format!("  {} = {}", key, env_values.get(key).unwrap()));
    }
    Ok(())
}