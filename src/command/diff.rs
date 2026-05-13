use crate::error::CsError;
use crate::profile::{find_project_dir, read_current_env, read_profile};
use similar::{ChangeTag, TextDiff};
use colored::Colorize;

pub fn run(name: &str) -> Result<(), CsError> {
    let project = find_project_dir()?;
    let current_env = read_current_env(&project)?;
    let profile_env = read_profile(name)?;

    let current_json = serde_json::to_string_pretty(&current_env)
        .map_err(|e| crate::error::json_err("current env", e))?;
    let profile_json = serde_json::to_string_pretty(&profile_env)
        .map_err(|e| crate::error::json_err(name, e))?;

    if current_json == profile_json {
        crate::output::info(&format!("No differences between current env and profile '{}'", name));
        return Ok(());
    }

    println!("--- current env");
    println!("+++ profile: {}", name);

    let diff = TextDiff::from_lines(&current_json, &profile_json);
    for change in diff.iter_all_changes() {
        let line = change.to_string_lossy();
        match change.tag() {
            ChangeTag::Delete => println!("-{}", line.red()),
            ChangeTag::Insert => println!("+{}", line.green()),
            ChangeTag::Equal => println!(" {}", line),
        }
    }
    Ok(())
}