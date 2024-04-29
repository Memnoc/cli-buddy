use crate::ais::OpenAIClient;
use crate::Result;
use async_openai::types::AssistantObject;
use async_openai::types::AssistantToolsRetrieval;
use async_openai::types::CreateAssistantRequest;
use derive_more::{Deref, Display, From};

// NOTE: region:    --- Constants

const DEFAULT_QUERY: &[(&str, &str)] = &[("limit", "100")];

// NOTE: region:    --- Constants

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

// search assisstants by name
pub async fn first_by_name(
	open_ai_client: &OpenAIClient,
	name: &str,
) -> Result<Option<AssistantObject>> {
	let openai_assistant = open_ai_client.assistants(); // init assistant
	let assistant = openai_assistant.list(DEFAULT_QUERY).await?.data; // returns a list of assistants
	let assistant_object = assistant // iterate and compare the names
		.into_iter()
		.find(|a| a.name.as_ref().map(|n| n == name).unwrap_or(false));

	Ok(assistant_object)
}

// NOTE: endregion:    ---Assistant CRUD
