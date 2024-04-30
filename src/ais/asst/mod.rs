use crate::ais::OpenAIClient;
use crate::Result;
use async_openai::config;
use async_openai::types::AssistantObject;
use async_openai::types::AssistantToolsRetrieval;
use async_openai::types::CreateAssistantRequest;
use async_openai::types::ModifyAssistantRequest;
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

pub async fn load_or_create(
	open_ai_client: &OpenAIClient,
	config: CreateConfig,
	recreate: bool,
) -> Result<AssistantId> {
	let assistant_object = first_by_name(open_ai_client, &config.name).await?;
	let mut assistant_id = assistant_object.map(|o| AssistantId::from(o.id));
	// -- Delete assistant if recreate true and assistant id
	if let (true, Some(assistant_id_ref)) = (recreate, assistant_id.as_ref()) {
		delete(open_ai_client, assistant_id_ref).await?;
		assistant_id.take();
		println!("Assistant {} deleted", config.name);
	}
	// -- Create if needed
	if let Some(assistant_id) = assistant_id {
		println!("Assistant {} laoded", config.name);
		Ok(assistant_id)
	} else {
		let assistant_name = config.name.clone();
		let assistant_id = create(open_ai_client, config).await?;
		Ok(assistant_id)
	}
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

pub async fn upload_instructions(
	open_ai_client: &OpenAIClient,
	assistant_id: &AssistantId,
	instruction_content: String,
) -> Result<()> {
	let opeanai_assistant = open_ai_client.assistants();
	let modify = ModifyAssistantRequest {
		instructions: Some(instruction_content),
		..Default::default()
	};
	opeanai_assistant.update(assistant_id, modify).await?;

	Ok(())
}

pub async fn delete(
	open_ai_client: &OpenAIClient,
	assistant_id: &AssistantId,
) -> Result<()> {
	let opeanai_assistant = open_ai_client.assistants();

	// TODO: delete files

	// -- Delete assistant
	opeanai_assistant.delete(assistant_id).await?;

	Ok(())
}

// NOTE: endregion:    ---Assistant CRUD
