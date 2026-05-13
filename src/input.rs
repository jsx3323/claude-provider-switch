use std::io;

use crate::error::{CsError, io_err};
use crate::output;

pub fn prompt_required(field: &str) -> Result<String, CsError> {
    loop {
        eprintln!("{}: ", field);
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| io_err("stdin", e))?;
        let value = input.trim().to_string();
        if value.is_empty() {
            output::error(&format!("{} is required", field));
            continue;
        }
        return Ok(value);
    }
}

pub fn prompt_optional(field: &str, default: &str) -> Result<Option<String>, CsError> {
    eprintln!("{} (optional, default: {}): ", field, default);
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| io_err("stdin", e))?;
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed))
    }
}

pub fn prompt_confirm() -> Result<bool, CsError> {
    eprintln!("Continue? [y/N] ");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).map_err(|e| io_err("stdin", e))?;
    Ok(answer.trim().to_lowercase() == "y")
}
