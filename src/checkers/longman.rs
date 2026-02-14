use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;
use crate::utils::dom::DomOpts;

/// Longman Dictionary checker
/// 
/// Checks if the <head> element has the .metadata class.
/// If head has .metadata class, word is found (return true).
/// If head doesn't have .metadata class, word not found (return false).
/// 
/// URL structure: https://www.ldoceonline.com/dictionary/WORD
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 259-275)
pub async fn longman(word: &str) -> CheckerResult {
    match Earl::new("https://www.ldoceonline.com", "/dictionary/", None) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.fetch_dom().await {
                Ok(dom) => {
                    // Navigate: html > head (with optional metadata class)
                    // The optional flag allows us to detect if head.metadata exists
                    match domstroll(
                        "ld",
                        false,
                        &dom,
                        &[
                            (1, "head", Some(DomOpts { cls: Some("metadata".to_string()), optional: true, ..Default::default() })),
                        ],
                    ) {
                        Ok(_head_with_metadata) => {
                            // If we got here, head has .metadata class
                            println!("[ISAWORD/longman] {} <head> has .metadata class", word);
                            CheckerResult {
                                name: "Longman",
                                result: Some(true),
                                is_community: false,
                            }
                        }
                        Err(_) => {
                            // domstroll failed because head doesn't have .metadata
                            println!("[ISAWORD/longman] {} <head> does not have .metadata class", word);
                            CheckerResult {
                                name: "Longman",
                                result: Some(false),
                                is_community: false,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/longman] {}", e);
                    CheckerResult {
                        name: "Longman",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/longman] {}", e);
            CheckerResult {
                name: "Longman",
                result: None,
                is_community: false,
            }
        }
    }
}
