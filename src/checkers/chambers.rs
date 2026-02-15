use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;
use crate::utils::dom::{DomOpts, find_body_index};

/// Chambers Dictionary checker
/// 
/// Navigates DOM to find p.message element and checks child count:
/// - 1 child = word found
/// - 3 children = word not found
/// - anything else = unknown/error
/// 
/// URL structure: https://chambers.co.uk/search/?query=WORD&title=21st
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 91-118)
pub async fn chambers(word: &str) -> CheckerResult {
    let mut params = std::collections::HashMap::new();
    params.insert("query", word.to_string());
    params.insert("title", "21st".to_string());

    match Earl::new("https://chambers.co.uk", "/search/", Some(params)) {
        Ok(earl) => {
            match earl.fetch_dom().await {
                 Ok(dom) => {
                     // Navigate: body.page-template-template-search-results > wrapper > content >
                     //           row > search-results > fullsearchresults > p.message
                     let body_idx = find_body_index(&dom).unwrap_or(3);
                     match domstroll(
                         "cham",
                         false,
                         &dom,
                         &[
                             (body_idx, "body", Some(DomOpts { cls: Some("page-template-template-search-results".to_string()), ..Default::default() })),
                            (7, "div", Some(DomOpts { id: Some("wrapper".to_string()), ..Default::default() })),
                            (4, "section", Some(DomOpts { id: Some("content".to_string()), ..Default::default() })),
                            (1, "div", Some(DomOpts { cls: Some("row".to_string()), ..Default::default() })),
                            (1, "div", Some(DomOpts { id: Some("search-results".to_string()), ..Default::default() })),
                            (8, "div", Some(DomOpts { id: Some("fullsearchresults".to_string()), ..Default::default() })),
                            (0, "p", Some(DomOpts { cls: Some("message".to_string()), ..Default::default() })),
                        ],
                    ) {
                        Ok(message_elem) => {
                            let child_count = message_elem.children().count();
                            println!("[ISAWORD/chambers] {} status: p.message has {} children", word, child_count);
                            
                            let result = match child_count {
                                1 => Some(true),
                                3 => Some(false),
                                _ => None,
                            };
                            
                            CheckerResult {
                                name: "Chambers",
                                result,
                                is_community: false,
                            }
                        }
                        Err(e) => {
                            eprintln!("[ISAWORD/chambers] {}", e);
                            CheckerResult {
                                name: "Chambers",
                                result: None,
                                is_community: false,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/chambers] {}", e);
                    CheckerResult {
                        name: "Chambers",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/chambers] {}", e);
            CheckerResult {
                name: "Chambers",
                result: None,
                is_community: false,
            }
        }
    }
}
