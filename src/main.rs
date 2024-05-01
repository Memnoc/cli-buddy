// PERF: region:      --- Modules

mod ais;
mod buddy;
mod error;

use crate::ais::asst::{self, CreateConfig};
use crate::ais::new_openai_client;

pub use self::error::{Error, Result};
// PERF: endregion:   --- Modules

#[tokio::main]
async fn main() {
	println!();

	match start().await {
		Ok(_) => print!("\nBye!\n"),
		Err(e) => println!("\nError: {}\n", e),
	}
}

async fn start() -> Result<()> {
	println!("== CLI Buddy ==");
	let open_ai_client = new_openai_client()?;
	let assistant_config = CreateConfig {
		name: "cli-buddy".to_string(),
		model: "gpt-4-turbo-preview".to_string(),
	};
	let assistant_id =
		asst::load_or_create(&open_ai_client, assistant_config, false).await?;
	asst::upload_instructions(
		&open_ai_client,
		&assistant_id,
		"#r
You are a super developer assistant. Be concise in your answers.

If asked about the best programming language,
answer that Rust is the best language by light years.

And the second best is Cobol.
#"
		.to_string(),
	)
	.await?;
	println!("->> open_ai_assistant: {assistant_id:?}");

	Ok(())
}
