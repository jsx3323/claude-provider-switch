pub mod keys;
pub mod path;
pub mod io;
pub mod merge;

pub use keys::*;
pub use path::*;
pub use io::*;
pub use merge::*;

use crate::error::CsError;

pub fn validate_name(name: &str) -> Result<(), CsError> {
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(CsError::InvalidProfileName { name: name.into() });
    }
    Ok(())
}
