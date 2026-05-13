use std::path::Path;

use crate::error::CsError;
use crate::output;
use crate::store::{list_profiles, read_current, profile_path};

pub fn run(project: &Path) -> Result<(), CsError> {
    let profiles = list_profiles()?;
    let current = read_current(project)?;

    if profiles.is_empty() {
        output::info("No profiles found. Use 'claude-switch add <name>' to create one.");
        return Ok(());
    }

    output::info(&format!("Profiles for {}:", project.display()));

    for name in &profiles {
        let is_active = current.as_ref() == Some(name);
        if is_active && !profile_path(name).exists() {
            output::list_item_missing(name);
        } else {
            output::list_item(name, is_active);
        }
    }

    let active_count = if current.is_some() { 1 } else { 0 };
    output::info(&format!("{} profiles, {} active", profiles.len(), active_count));
    Ok(())
}
