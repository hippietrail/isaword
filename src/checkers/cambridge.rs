use crate::Earl;
use crate::checkers::CheckerResult;

/// Cambridge Dictionary checker
/// 
/// Uses HEAD request to check for redirects.
/// If word exists, Cambridge serves the page (no redirect).
/// If word doesn't exist, Cambridge redirects to search results (3xx status).
/// 
/// Therefore: return true if NOT a redirect, false if is redirect.
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 79-90)
pub async fn cambridge(word: &str) -> CheckerResult {
    match Earl::new(
        "https://dictionary.cambridge.org",
        "/dictionary/english/",
        None,
    ) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.check_redirect().await {
                Ok(Some(is_redirect)) => {
                    // Invert: true if NOT redirect
                    CheckerResult {
                        name: "Cambridge",
                        result: Some(!is_redirect),
                        is_community: false,
                    }
                }
                Ok(None) => {
                    // Network error during check
                    CheckerResult {
                        name: "Cambridge",
                        result: None,
                        is_community: false,
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/cambridge] {}", e);
                    CheckerResult {
                        name: "Cambridge",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/cambridge] {}", e);
            CheckerResult {
                name: "Cambridge",
                result: None,
                is_community: false,
            }
        }
    }
}
