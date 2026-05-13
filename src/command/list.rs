use colored::Colorize;
use crate::error::CsError;
use crate::output;
use crate::profile::{find_project_dir, list_profiles, read_current};

pub fn run() -> Result<(), CsError> {
    let project = find_project_dir()?;
    let profiles = list_profiles()?;
    let current = read_current(&project)?;

    if profiles.is_empty() {
        output::info("No profiles found. Use 'claude-switch add <name>' to create one.");
        return Ok(());
    }

    output::info(&format!("Profiles for {}:", project.display()));

    for name in &profiles {
        let is_active = current.as_ref() == Some(name);
        if is_active && !crate::profile::profile_path(name).exists() {
            println!("  {} {} {}", "*".green().bold(), name.bold(), "(active - missing!)".red());
        } else {
            output::list_item(name, is_active);
        }
    }

    let active_count = if current.is_some() { 1 } else { 0 };
    output::info(&format!("{} profiles, {} active", profiles.len(), active_count));
    Ok(())
}