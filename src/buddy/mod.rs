//PERF: region:    --- Modules

mod config;
use self::config::Config;
use crate::ais::{asst::AssistantId, OpenAIClient};
use std::path::PathBuf;

//PERF: endregion: --- Modules

const BUDDY_TOML: &str = "buddy.toml";

#[derive(Debug)]
pub struct Buddy {
	dir: PathBuf,
	oac: OpenAIClient,
	asst_id: AssistantId,
	config: Config,
}
