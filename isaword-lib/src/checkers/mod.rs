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
//! REASON: Collins uses Cloudflare bot detection AND client-side JavaScript rendering.
//!
//! TECHNICAL BARRIERS:
//! 1. Cloudflare Challenge: Initial response is "Just a moment..." HTML with bot detection
//!    script. Cloudflare requires solving a challenge token before serving content.
//! 2. JavaScript Rendering: Even after bypassing Cloudflare, the actual dictionary content
//!    is rendered entirely by JavaScript, not present in initial HTML.
//!
//! This makes Collins impossible to scrape with standard HTTP requests (reqwest + scraper).
//! Traditional approach (Earl + domstroll) can't handle either barrier.
//!
//! REQUIREMENTS TO SUPPORT COLLINS:
//! 1. Cloudflare bypass: Would need to handle CF challenge tokens (cloudflare-scraper crate?)
//! 2. JavaScript rendering: Would need headless browser (headless_chrome, puppeteer-rs, etc)
//! 3. Performance concern: Headless browser + Cloudflare handling would be much slower
//!
//! ALTERNATIVES TO INVESTIGATE:
//! 1. Check if Collins has a public API endpoint
//! 2. Evaluate using cloudflare-scraper + headless browser combo
//! 3. Consider if effort is worth the performance cost
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
