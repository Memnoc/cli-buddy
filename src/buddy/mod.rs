//PERF: region:    --- Modules

mod config;

use crate::ais::asst::{self, AssistantId, ThreadId};
use crate::ais::{new_openai_client, OpenAIClient};
use crate::buddy::config::Config;
use crate::utils::files::ensure_dir;
use crate::Result;
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

//PERF: endregion: --- Modules

const BUDDY_TOML: &str = "buddy.toml";

#[derive(Debug)]
pub struct Buddy {
	dir: PathBuf,
	oac: OpenAIClient,
	asst_id: AssistantId,
	config: Config,
}

#[derive(Debug, From, Deref, Deserialize, Serialize)]
pub struct Conv {
	thread_id: ThreadId,
}

// INFO: Public functions
impl Buddy {
	fn data_dir(&self) -> Result<PathBuf> {
		let data_dir = self.dir.join(".buddy");
		ensure_dir(&data_dir)?;
		Ok(data_dir)
	}
	fn data_files_dir(&self) -> Result<PathBuf> {
		let dir = self.data_dir()?.join("files");
		ensure_dir(&dir);
		Ok(dir)
	}
}

/// Private functions
impl Buddy {}
