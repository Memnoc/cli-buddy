// PERF: region:      --- Modules

mod ais;
mod buddy;
mod error;

pub use self::error::{Error, Result};
// PERF: endregion:   --- Modules

fn main() {
    println!("== CLI Buddy ==");
}
