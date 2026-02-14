//! isaword - A Rust CLI tool for checking if a word exists in multiple dictionaries
//! 
//! This is a direct port of the `/isaword2` Discord bot slash command from:
//! - /Users/hippietrail/hippiebot.js/commands/isaword.js
//! - /Users/hippietrail/lyre/commands/isaword.js
//! 
//! Both Discord bot implementations are identical. This Rust version maintains
//! the same logic for checking 14 different dictionaries in parallel and
//! formatting results with the same human-friendly output.

pub mod utils;
pub mod checkers;

pub use utils::earl::Earl;
pub use utils::dom::domstroll;
pub use checkers::CheckerResult;
pub use utils::format::human_friendly_list_formatter;

pub async fn run_cli(word: &str) {
    println!("Checking word: {}", word);
    
    // Test with urban dictionary
    let result = checkers::urban::urban(word).await;
    println!("Result: {:?}", result);
}
