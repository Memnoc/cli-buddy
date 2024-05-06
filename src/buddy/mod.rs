//PERF: region:    --- Modules

mod config;

use crate::ais::asst::{self, AssistantId, ThreadId};
use crate::ais::{new_openai_client, OpenAIClient};
use crate::buddy::config::Config;
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
