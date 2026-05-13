use std::io;
use std::path::Path;

use crate::error::CsError;
use crate::output;
use crate::store::{validate_name, delete_profile, read_current, clear_current};

pub fn run(name: &str, force: bool, project: &Path) -> Result<(), CsError> {
    validate_name(name)?;
    let current = read_current(project)?;

    let is_active = current.as_ref() == Some(&name.to_string());
    if is_active && !force {
        output::info(&format!("Profile '{}' is currently active.", name));
        output::info("Deleting will remove the profile but leave settings.local.json unchanged.");
        if !prompt_confirm()? {
            output::info("Cancelled.");
            return Ok(());
        }
    }

    delete_profile(name)?;

    if is_active {
        clear_current(project)?;
        output::success(&format!("Deleted profile '{}' (was active). settings.local.json unchanged.", name));
    } else {
        output::success(&format!("Deleted profile '{}'", name));
    }
    Ok(())
}

fn prompt_confirm() -> Result<bool, CsError> {
    eprintln!("Continue? [y/N] ");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).map_err(|e| crate::error::io_err("stdin", e))?;
    Ok(answer.trim().to_lowercase() == "y")
}
