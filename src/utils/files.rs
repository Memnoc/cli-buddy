use crate::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::fs::{self, File};
use std::io::{BufRead as _, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// PERF: region:        --- File Parser/Writer

pub fn load_from_toml<T>(file: impl AsRef<Path>) -> Result<T>
where
	T: serde::de::DeserializeOwned,
{
	let content = read_to_string(file.as_ref())?;

	Ok(toml::from_str(&content)?)
}

pub fn load_from_json<T>(file: impl AsRef<Path>) -> Result<T>
where
	T: serde::de::DeserializeOwned,
{
	let val = serde_json::from_reader(get_reader(file.as_ref())?)?;
	Ok(val)
}

pub fn save_to_json<T>(file: impl AsRef<Path>, data: &T) -> Result<()>
where
	T: serde::Serialize,
{
	let file = file.as_ref();

	let file = File::create(file)
		.map_err(|e| format!("Cannot create file '{:?}': {}", file, e))?;
	serde_json::to_writer_pretty(file, data)?;

	Ok(())
}

// PERF: end region:    --- File Parser/Writer

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

	// --- Prep globs
	let include_globs = include_globs.map(get_glob_set).transpose()?;
	let exclude_globs = exclude_globs.map(get_glob_set).transpose()?;

	// --- Build files iterator
	let walk_dir_iterator = WalkDir::new(dir)
		.max_depth(depth)
		.into_iter()
		.filter_entry(|e| {
			if e.file_type().is_dir() {
				!base_dir_exclude.is_match(e.path())
			}
			// else file, we apply the globs
			else {
				if let Some(exclude_globs) = exclude_globs.as_ref() {
					if exclude_globs.is_match(e.path()) {
						return false;
					}
				}
				match include_globs.as_ref() {
					Some(globs) => globs.is_match(e.path()),
					None => true,
				}
			}
		})
		.filter_map(|e| e.ok().filter(|e| e.file_type().is_file()));

	let paths = walk_dir_iterator.map(|e| e.into_path());

	Ok(paths.collect())
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

fn read_to_string(file: &Path) -> Result<String> {
	if !file.is_file() {
		return Err(format!("File not found: {}", file.display()).into());
	}
	let content = fs::read_to_string(file)?;

	Ok(content)
}

fn get_reader(file: &Path) -> Result<BufReader<File>> {
	let Ok(file) = File::open(file) else {
		return Err(format!("File not found: {}", file.display()).into());
	};

	Ok(BufReader::new(file))
}

// PERF: endregion: --- File Utils
