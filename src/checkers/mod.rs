//! Dictionary checkers - parallel HTTP queries to verify word existence
//! 
//! This module implements 13 dictionary checker functions that mirror the TypeScript
//! implementations from:
//! - /Users/hippietrail/hippiebot.js/commands/isaword.js
//! - /Users/hippietrail/lyre/commands/isaword.js
//! 
//! Each checker function:
//! - Takes a word as input
//! - Returns a CheckerResult with the word's existence status (true/false/null)
//! - Logs results and errors using println! (like TS console.log)
//! - Handles network errors gracefully (returns null, continues execution)
//! 
//! Checkers are designed to be executed in parallel via futures::join_all()

pub mod urban;

/// Result of a single dictionary checker
/// 
/// - name: Display name of the dictionary
/// - result: Some(true) = found, Some(false) = not found, None = error/unknown
/// - is_community: true if user-editable (like Wiktionary), false if professionally maintained
#[derive(Debug, Clone)]
pub struct CheckerResult {
    pub name: &'static str,
    pub result: Option<bool>,
    pub is_community: bool,
}
