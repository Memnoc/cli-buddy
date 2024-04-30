use async_openai::types::CreateMessageRequest;

use crate::Result;

//PERF: region: --- Message Constructors

pub fn user_msg(content: impl Into<String>) -> CreateMessageRequest {
	CreateMessageRequest {
		role: "user".to_string(),
		content: content.into(),
		..Default::default()
	}
}

//PERF: region: --- Message Constructors
