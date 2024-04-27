use crate::ais::OpenAIClient;
use crate::Result;
use async_openai::types::CreateAssistantRequest;
use derive_more::{Deref, Display, From};

// PERF: region    ---Types

pub struct CreateConfig {
    name: String,
    model: String,
}

#[derive(Debug, From, Deref, Display)]
pub struct AssistantId(String);
#[derive(Debug, From, Deref, Display)]
pub struct ThreadId(String);

#[derive(Debug, From, Deref, Display)]
pub struct FileId(String);

// PERF: endregion ---Types

pub async fn create(open_ai_client: &OpenAIClient, config: CreateConfig) -> Result<AssistantId> {
    let openai_assistant = open_ai_client.assistants(); // assistant manager

    let assistant_obj = openai_assistant
        .create(CreateAssistantRequest {
            model: config.model,
            name: Some(config.name),
            ..Default::default()
        })
        .await?;
    todo!()
}
