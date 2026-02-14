use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;
use crate::utils::dom::DomOpts;

/// American Heritage Dictionary checker
/// 
/// Navigates to div#results and checks children pattern:
/// - Not found: 1 text node child only
/// - Found: children count is divisible by 4, first child is a table
///          Pattern is: <table> <hr> <span.copyright> <comment> (repeats)
/// 
/// URL structure: https://ahdictionary.com/word/search.html?q=WORD
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 138-170)
pub async fn ahd(word: &str) -> CheckerResult {
    let mut params = std::collections::HashMap::new();
    params.insert("q", word.to_string());

    match Earl::new("https://ahdictionary.com", "/word/search.html", Some(params)) {
        Ok(earl) => {
            match earl.fetch_dom().await {
                Ok(dom) => {
                    // Navigate: html > body > #content > .container3 > #results
                    match domstroll(
                        "ahd",
                        false,
                        &dom,
                        &[
                            (0, "html", None),
                            (1, "body", None),
                            (3, "div", Some(DomOpts { id: Some("content".to_string()), ..Default::default() })),
                            (2, "div", Some(DomOpts { cls: Some("container3".to_string()), ..Default::default() })),
                            (1, "div", Some(DomOpts { id: Some("results".to_string()), ..Default::default() })),
                        ],
                    ) {
                        Ok(results_elem) => {
                            let children: Vec<_> = results_elem.children().collect();
                            let child_count = children.len();
                            println!("[ISAWORD/ahd] {} div#results.children.length: {}", word, child_count);
                            
                            // Check pattern: single text node = not found
                            if child_count == 1 {
                                // Check if it's a text node
                                if let Some(first) = children.first() {
                                    if first.value().as_text().is_some() {
                                        // It's a text node, word not found
                                        return CheckerResult {
                                            name: "American Heritage",
                                            result: Some(false),
                                            is_community: false,
                                        };
                                    }
                                }
                            }
                            
                            // Check pattern: children count % 4 == 0 && first is table
                            if child_count % 4 == 0 && child_count > 0 {
                                if let Some(first) = children.first() {
                                    if let Some(elem) = first.value().as_element() {
                                        if elem.name() == "table" {
                                            return CheckerResult {
                                                name: "American Heritage",
                                                result: Some(true),
                                                is_community: false,
                                            };
                                        }
                                    }
                                }
                            }
                            
                            // Pattern didn't match, log for debugging
                            if let Some(first) = children.first() {
                                if let Some(elem) = first.value().as_element() {
                                    println!("[ISAWORD/ahd] {} div#results.children[0].type: element", word);
                                    println!("[ISAWORD/ahd] {} div#results.children[0].name: {}", word, elem.name());
                                } else if first.value().as_text().is_some() {
                                    println!("[ISAWORD/ahd] {} div#results.children[0].type: text", word);
                                }
                            }
                            
                            CheckerResult {
                                name: "American Heritage",
                                result: None,
                                is_community: false,
                            }
                        }
                        Err(e) => {
                            eprintln!("[ISAWORD/ahd] {}", e);
                            CheckerResult {
                                name: "American Heritage",
                                result: None,
                                is_community: false,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/ahd] {}", e);
                    CheckerResult {
                        name: "American Heritage",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/ahd] {}", e);
            CheckerResult {
                name: "American Heritage",
                result: None,
                is_community: false,
            }
        }
    }
}
