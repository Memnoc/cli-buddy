// PERF: region:      --- Modules

mod ais;
mod buddy;
mod error;

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
    println!("->> open_ai_client: {open_ai_client:?}");

    Ok(())
}
