use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;
use crate::utils::dom::DomOpts;

/// Merriam-Webster Dictionary checker
/// 
/// Logic:
/// 1. Check body.class for "definitions-page"
///    - If not present, word not found (return false)
///    - If present, continue to step 2
/// 2. Check for .redesign-container within the page
///    - If present, word fully found (return true)
///    - If absent, partial match (return false)
/// 
/// Partial matches lack the redesign-container div.
/// 
/// URL structure: https://www.merriam-webster.com/dictionary/WORD
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 229-258)
pub async fn mw(word: &str) -> CheckerResult {
    match Earl::new("https://www.merriam-webster.com", "/dictionary/", None) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.fetch_dom().await {
                Ok(dom) => {
                    // Navigate to body
                    match domstroll(
                        "mw",
                        false,
                        &dom,
                        &[
                            (2, "html", None),
                            (3, "body", None),
                        ],
                    ) {
                        Ok(body_elem) => {
                            let body_classes = body_elem
                                .value()
                                .attr("class")
                                .unwrap_or("")
                                .split_whitespace()
                                .collect::<Vec<_>>();
                            
                            println!("[ISAWORD/mw] {} body class: {}", 
                                word,
                                body_classes.iter().map(|c| format!("'{}'", c)).collect::<Vec<_>>().join(", ")
                            );
                            
                            // Check for definitions-page class
                            if !body_classes.contains(&"definitions-page") {
                                return CheckerResult {
                                    name: "Merriam-Webster",
                                    result: Some(false),
                                    is_community: false,
                                };
                            }
                            
                            // definitions-page found, check for partial matches
                            // Look for .redesign-container in the hierarchy
                            match domstroll(
                                "mw",
                                false,
                                &dom,
                                &[
                                    (2, "html", None),
                                    (3, "body", None),
                                    (17, "div", Some(DomOpts { cls: Some("outer-container".to_string()), ..Default::default() })),
                                    (1, "div", Some(DomOpts { cls: Some("main-container".to_string()), ..Default::default() })),
                                    (3, "div", Some(DomOpts { cls: Some("redesign-container".to_string()), optional: true, ..Default::default() })),
                                ],
                            ) {
                                Ok(_redesign_container) => {
                                    println!("[ISAWORD/mw] {} maybeRedesignContainer: exists", word);
                                    CheckerResult {
                                        name: "Merriam-Webster",
                                        result: Some(true),
                                        is_community: false,
                                    }
                                }
                                Err(_) => {
                                    println!("[ISAWORD/mw] {} maybeRedesignContainer: does not exist", word);
                                    CheckerResult {
                                        name: "Merriam-Webster",
                                        result: Some(false),
                                        is_community: false,
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[ISAWORD/mw] {}", e);
                            CheckerResult {
                                name: "Merriam-Webster",
                                result: None,
                                is_community: false,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/mw] {}", e);
                    CheckerResult {
                        name: "Merriam-Webster",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/mw] {}", e);
            CheckerResult {
                name: "Merriam-Webster",
                result: None,
                is_community: false,
            }
        }
    }
}
