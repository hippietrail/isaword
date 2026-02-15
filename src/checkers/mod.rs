//! Dictionary checkers - parallel HTTP queries to verify word existence
//! 
//! This module implements 13 dictionary checker functions that mirror the TypeScript
//! implementations from:
//! - https://github.com/hippietrail/hippiebot.js/blob/main/commands/isaword.js
//! - https://github.com/hippietrail/lyre/blob/main/commands/isaword.js
//! 
//! Each checker function:
//! - Takes a word as input
//! - Returns a CheckerResult with the word's existence status (true/false/null)
//! - Logs results and errors using println! (like TS console.log)
//! - Handles network errors gracefully (returns null, continues execution)
//! 
//! Checkers are designed to be executed in parallel via futures::join_all()
//!
//! # Not Implemented: Collins Dictionary
//!
//! Collins dictionary is NOT supported. See issue isaword-go4 for details.
//!
//! REASON: Collins website is entirely built with JavaScript (client-side rendering).
//! The initial HTML response contains only a JavaScript bundle - no actual dictionary
//! content that can be scraped server-side.
//!
//! Traditional scraping approach (Earl + domstroll) requires server-rendered HTML with
//! DOM content present in the initial response. Collins renders everything client-side,
//! making it impossible to determine word existence without a headless browser.
//!
//! TO SUPPORT COLLINS IN THE FUTURE:
//! 1. Implement headless browser integration (headless_chrome, puppeteer-rs, etc)
//! 2. Investigate if Collins has a public API endpoint
//! 3. Evaluate performance/complexity tradeoffs
//!
//! See: https://github.com/hippietrail/hippiebot.js/blob/main/commands/isaword.ts#L221-L239

pub mod urban;
pub mod oed;
pub mod wikt;
pub mod cambridge;
pub mod chambers;
pub mod dictcom;
pub mod longman;
pub mod ahd;
pub mod mw;
pub mod oxfordlearners;
pub mod wordnet;
pub mod wordnik;
pub mod etym;
pub mod etymonline;

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
