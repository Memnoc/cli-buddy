use super::OpenAiClient;
use crate::Result;
use derive_more::{Deref, Display, From};

// PERF: region    ---Types

pub struct CreateConfig {
    name: String,
    model: String,
}

#[derive(Debug, From, Deref, Display)]
pub struct AssistantId(String);

// PERF: endregion ---Types

pub async fn create(open_ai_client: &OpenAiClient, config: CreateConfig) -> Result<AssistantId> {
    todo!()
}
