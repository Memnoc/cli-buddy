// PERF: region:      --- Modules

pub mod asst;
use crate::Result;
use async_openai::{config::OpenAIConfig, Client};

// PERF: endregion:   --- Modules

// INFO: region: --- Client
const ENV_OPENAI_API_KEY: &str = "OPENAI_API_KEY";

pub type OpenAIClient = Client<OpenAIConfig>;

pub fn new_openai_client() -> Result<OpenAIClient> {
    if std::env::var(ENV_OPENAI_API_KEY).is_ok() {
        Ok(Client::new())
    } else {
        println!("No {ENV_OPENAI_API_KEY} env variable. Please set it");
        Err("No openai api key in env".into())
    }
}

// INFO: endregion: -- Client
