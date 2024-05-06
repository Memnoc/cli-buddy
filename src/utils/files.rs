use std::{
	fs,
	path::{Path, PathBuf},
};

use globset::{Glob, GlobSet, GlobSetBuilder};

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

pub fn list_files(
	dir: &Path,
	include_globs: Option<&[&str]>,
	exclude_globs: Option<&[&str]>,
) -> Result<Vec<PathBuf>> {
	let base_dir_exclude = base_dir_exclude_globs()?;

	// --- Determine recursive depth
	let depth = include_globs
		.map(|globs| globs.iter().any(|&g| g.contains("**")))
		.map(|v| if v { 100 } else { 1 })
		.unwrap_or(1);

	let include_globs = include_globs.map(get_glob_set).transpose()?;
	let exclude_globs = exclude_globs.map(get_glob_set).transpose()?;

	todo!()
}

fn base_dir_exclude_globs() -> Result<GlobSet> {
	get_glob_set(&[".git", "target"])
}

pub fn get_glob_set(globs: &[&str]) -> Result<GlobSet> {
	let mut builder = GlobSetBuilder::new();
	for glob in globs {
		builder.add(Glob::new(glob)?);
	}
	Ok(builder.build()?)
}

// PERF: endregion: --- Dir Utils

// PERF: region:    --- File Utils

// PERF: endregion: --- File Utils
