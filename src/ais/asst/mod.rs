use crate::ais::OpenAIClient;
use crate::Result;
use async_openai::types::AssistantObject;
use async_openai::types::AssistantToolsRetrieval;
use async_openai::types::CreateAssistantRequest;
use derive_more::{Deref, Display, From};

// PERF: region    ---Types

pub struct CreateConfig {
	pub name: String,
	pub model: String,
}

#[derive(Debug, From, Deref, Display)]
pub struct AssistantId(String);
#[derive(Debug, From, Deref, Display)]
pub struct ThreadId(String);

#[derive(Debug, From, Deref, Display)]
pub struct FileId(String);

// PERF: endregion ---Types

// NOTE: region:   ---Assistant CRUD
pub async fn create(
	open_ai_client: &OpenAIClient,
	config: CreateConfig,
) -> Result<AssistantId> {
	let openai_assistant = open_ai_client.assistants(); // assistant manager

	let assistant_obj = openai_assistant
		.create(CreateAssistantRequest {
			// returning an assistan object
			model: config.model,
			name: Some(config.name),
			tools: Some(vec![AssistantToolsRetrieval::default().into()]),
			..Default::default()
		})
		.await?;
	Ok(assistant_obj.id.into())
}

pub async fn first_by_name(
	open_ai_client: &OpenAIClient,
	name: &str,
) -> Result<Option<AssistantObject>> {
	todo!()
}

// NOTE: endregion:    ---Assistant CRUD
