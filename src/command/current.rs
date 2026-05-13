use crate::error::CsError;
use crate::output;
use crate::profile::{find_project_dir, read_current};

pub fn run() -> Result<(), CsError> {
    let project = find_project_dir()?;
    let current = read_current(&project)?;

    match current {
        Some(name) => output::success(&format!("Current profile: {}", name)),
        None => output::info("No active profile (settings.local.json is not managed by claude-switch)"),
    }
    Ok(())
}