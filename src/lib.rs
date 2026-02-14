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
