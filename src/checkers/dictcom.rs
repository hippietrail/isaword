use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;
use crate::utils::dom::DomOpts;

/// Dictionary.com checker
/// 
/// Checks the main element's children count:
/// - 3 children = word not found
/// - 4 children = word found
/// - anything else = unknown/error
/// 
/// URL structure: https://www.dictionary.com/browse/WORD
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 327-350)
pub async fn dictcom(word: &str) -> CheckerResult {
    match Earl::new("https://www.dictionary.com", "/browse/", None) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.fetch_dom().await {
                Ok(dom) => {
                    // Navigate: html > body > #root > .dictionary-site > main
                    match domstroll(
                        "dict.com",
                        false,
                        &dom,
                        &[
                            (3, "html", None),
                            (3, "body", None),
                            (1, "div", Some(DomOpts { id: Some("root".to_string()), ..Default::default() })),
                            (0, "div", Some(DomOpts { cls: Some("dictionary-site".to_string()), ..Default::default() })),
                            (1, "main", None),
                        ],
                    ) {
                        Ok(main_elem) => {
                            let child_count = main_elem.children().count();
                            println!("[ISAWORD/dict.com] {} main.children.length: {}", word, child_count);
                            
                            let result = match child_count {
                                3 => Some(false),
                                4 => Some(true),
                                _ => None,
                            };
                            
                            CheckerResult {
                                name: "Dictionary.com",
                                result,
                                is_community: false,
                            }
                        }
                        Err(e) => {
                            eprintln!("[ISAWORD/dict.com] {}", e);
                            CheckerResult {
                                name: "Dictionary.com",
                                result: None,
                                is_community: false,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/dict.com] {}", e);
                    CheckerResult {
                        name: "Dictionary.com",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/dict.com] {}", e);
            CheckerResult {
                name: "Dictionary.com",
                result: None,
                is_community: false,
            }
        }
    }
}
