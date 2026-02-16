use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;

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
/// Ported from: https://github.com/hippietrail/hippiebot.js/blob/main/commands/isaword.js (line 229-258)
pub async fn mw(word: &str) -> CheckerResult {
    // Try plain Earl without custom headers first - reqwest might have better defaults
    match Earl::new("https://www.merriam-webster.com", "/dictionary/", None) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.fetch_text().await {
                Ok(text) => {
                    let dom = scraper::Html::parse_document(&text);
                    
                    // First domstroll: navigate to body
                     match domstroll(
                        "mw.1",
                        false,
                        &dom,
                        &[
                            (2, "body", None),
                        ],
                    ) {
                        Ok(body_elem) => {
                            let body_classes = body_elem
                                .value()
                                .attr("class")
                                .unwrap_or("")
                                .split_whitespace()
                                .collect::<Vec<_>>();
                            
                            // Check for definitions-page class
                            if !body_classes.contains(&"definitions-page") {
                                return CheckerResult {
                                    name: "Merriam-Webster",
                                    result: Some(false),
                                    is_community: false,
                                };
                            }
                            
                            // definitions-page found, check for partial matches
                            // Look for .redesign-container in body's children
                            let outer_container_found = body_elem.children()
                                .filter_map(|child| scraper::element_ref::ElementRef::wrap(child))
                                .find(|el| {
                                    el.value().name() == "div" && 
                                    el.value().attr("class")
                                        .map(|cls| cls.contains("outer-container"))
                                        .unwrap_or(false)
                                });
                            
                            let has_redesign = match outer_container_found {
                                Some(outer) => {
                                    outer.children()
                                        .filter_map(|child| scraper::element_ref::ElementRef::wrap(child))
                                        .find(|el| {
                                            el.value().name() == "div" && 
                                            el.value().attr("class")
                                                .map(|cls| cls.contains("main-container"))
                                                .unwrap_or(false)
                                        })
                                        .and_then(|main| {
                                            main.children()
                                                .filter_map(|child| scraper::element_ref::ElementRef::wrap(child))
                                                .find(|el| {
                                                    el.value().name() == "div" && 
                                                    el.value().attr("class")
                                                        .map(|cls| cls.contains("redesign-container"))
                                                        .unwrap_or(false)
                                                })
                                        })
                                        .is_some()
                                }
                                None => false,
                            };
                            
                            CheckerResult {
                                name: "Merriam-Webster",
                                result: Some(has_redesign),
                                is_community: false,
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
