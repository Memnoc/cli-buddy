use std::{fs, path::Path};

use crate::Result;

// PERF: region:    --- Dir Utils

/// Returns true if one or more dir was created
pub fn ensure_dir(dir: &Path) -> Result<bool> {
	if dir.is_dir() {
		Ok(false)
	} else {
		fs::create_dir_all(dir)?;
		Ok(true)
	}
}

// PERF: endregion: --- Dir Utils

// PERF: region:    --- File Utils

// PERF: endregion: --- File Utils
