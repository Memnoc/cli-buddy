use async_openai::types::{CreateMessageRequest, MessageObject};

use crate::Result;

//PERF: region: --- Message Constructors

pub fn user_msg(content: impl Into<String>) -> CreateMessageRequest {
	CreateMessageRequest {
		role: "user".to_string(),
		content: content.into(),
		..Default::default()
	}
}

//PERF: endregion: --- Message Constructors

//PERF: region: --- Content Extractor

pub fn get_text_content(msg: MessageObject) -> Result<String> {
	let msg_content = msg
		.content
		.into_iter()
		.next()
		.ok_or_else(|| "No message content found".to_string())?;
	todo!()
}

//PERF: endregion: --- Content Extractor
