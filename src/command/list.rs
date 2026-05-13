use std::path::Path;

use crate::error::CsError;
use crate::output;
use crate::store::{list_profiles, read_current};

pub fn run(project: &Path) -> Result<(), CsError> {
    let profiles = list_profiles()?;
    let current = read_current(project)?;

    if profiles.is_empty() && current.is_none() {
        output::info("No profiles found. Use 'claude-switch add <name>' to create one.");
        return Ok(());
    }

    output::info(&format!("Profiles for {}:", project.display()));

    for name in &profiles {
        let is_active = current.as_ref() == Some(name);
        output::list_item(name, is_active);
    }

    // 活跃 profile 的文件被手动删除
    if let Some(active) = &current {
        if !profiles.contains(active) {
            output::list_item_missing(active);
        }
    }

    let active_count = if current.is_some() { 1 } else { 0 };
    output::info(&format!("{} profiles, {} active", profiles.len(), active_count));
    Ok(())
}
