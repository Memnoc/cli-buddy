use std::io::Write;
use std::time::Duration;

use crate::ais::msg;
use crate::ais::msg::get_text_content;
use crate::ais::OpenAIClient;
use crate::Result;
use async_openai::config;
use async_openai::types::AssistantObject;
use async_openai::types::AssistantToolsRetrieval;
use async_openai::types::CreateAssistantRequest;
use async_openai::types::CreateRunRequest;
use async_openai::types::CreateThreadRequest;
use async_openai::types::ModifyAssistantRequest;
use async_openai::types::RunStatus;
use async_openai::types::ThreadObject;
use console::Term;
use derive_more::{Deref, Display, From};
use serde::Deserialize;
use serde::Serialize;
use tokio::time::sleep;

// NOTE: region:    --- Constants

const DEFAULT_QUERY: &[(&str, &str)] = &[("limit", "100")];
const POLLING_DURATION_MS: u64 = 500;

// NOTE: region:    --- Constants

// PERF: region    ---Types

pub struct CreateConfig {
	pub name: String,
	pub model: String,
}

#[derive(Debug, From, Deref, Display)]
pub struct AssistantId(String);

#[derive(Debug, From, Deref, Display, Deserialize, Serialize)]
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
	let openai_assistant = open_ai_client.assistants();
	let modify = ModifyAssistantRequest {
		instructions: Some(instruction_content),
		..Default::default()
	};
	openai_assistant.update(assistant_id, modify).await?;

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

// NOTE: region:    --- Thread

pub async fn create_thread(open_ai_client: &OpenAIClient) -> Result<ThreadId> {
	let openai_threads = open_ai_client.threads();

	let response = openai_threads
		.create(CreateThreadRequest {
			..Default::default()
		})
		.await?;
	Ok(response.id.into())
}

pub async fn get_thread(
	open_ai_client: &OpenAIClient,
	thread_id: &ThreadId,
) -> Result<ThreadObject> {
	let openai_threads = open_ai_client.threads();

	let thread_object = openai_threads.retrieve(thread_id).await?;

	Ok(thread_object)
}

pub async fn run_thread_msg(
	open_ai_client: &OpenAIClient,
	assistant_id: &AssistantId,
	thread_id: &ThreadId,
	msg: &str,
) -> Result<String> {
	let msg = msg::user_msg(msg);

	// -- Attach message to the thread

	let _message_object = open_ai_client
		.threads()
		.messages(thread_id)
		.create(msg)
		.await?;

	// -- Create a run the thread

	let run_request = CreateRunRequest {
		assistant_id: assistant_id.to_string(),
		..Default::default()
	};

	let run = open_ai_client
		.threads()
		.runs(thread_id)
		.create(run_request)
		.await?;

	// -- Loop to get result
	let term = Term::stdout();
	loop {
		term.write_str(">")?;
		let run = open_ai_client
			.threads()
			.runs(thread_id)
			.retrieve(&run.id)
			.await?;
		term.write_str("<")?;
		match run.status {
			RunStatus::Completed => {
				term.write_str("\n")?;
				return get_first_thread_msg_content(open_ai_client, thread_id)
					.await;
			}
			RunStatus::Queued | RunStatus::InProgress => (),
			other => {
				term.write_str("\n")?;
				return Err(format!("ERROR WHILE RUN: {:?}", other).into());
			}
		}
		sleep(Duration::from_millis(POLLING_DURATION_MS)).await;
	}
}

pub async fn get_first_thread_msg_content(
	open_ai_client: &OpenAIClient,
	thread_id: &ThreadId,
) -> Result<String> {
	static QUERY: [(&str, &str); 1] = [("limit", "1")];

	let messages = open_ai_client
		.threads()
		.messages(thread_id)
		.list(&QUERY)
		.await?;
	let msg = messages
		.data
		.into_iter()
		.next()
		.ok_or_else(|| "No message found".to_string())?;

	let text = get_text_content(msg)?;
	Ok(text)
}

// NOTE: endregion:    --- Thread
