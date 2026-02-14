use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;

/// WordNet checker
/// 
/// Navigation: body > filter tag elements and check tag sequence
/// - Not found: tags are [form, form, h3]
/// - Found: tags are [div, div] with classes [header, form]
/// 
/// URL structure: http://wordnetweb.princeton.edu/perl/webwn?s=WORD
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 351-374)
pub async fn wordnet(word: &str) -> CheckerResult {
    let mut params = std::collections::HashMap::new();
    params.insert("s", word.to_string());

    match Earl::new("http://wordnetweb.princeton.edu", "/perl/webwn", Some(params)) {
        Ok(earl) => {
            match earl.fetch_dom().await {
                Ok(dom) => {
                    match domstroll(
                        "wordnet",
                        false,
                        &dom,
                        &[
                            (2, "html", None),
                            (3, "body", None),
                        ],
                    ) {
                        Ok(body_elem) => {
                            // Filter to only tag nodes and get tag names
                            let tag_nodes: Vec<_> = body_elem
                                .children()
                                .filter_map(|child| scraper::element_ref::ElementRef::wrap(child))
                                .collect();
                            
                            let tag_names: Vec<&str> = tag_nodes
                                .iter()
                                .map(|e| e.value().name())
                                .collect();
                            
                            // Check for not-found pattern: [form, form, h3]
                            if tag_names.len() >= 3
                                && tag_names[0] == "form"
                                && tag_names[1] == "form"
                                && tag_names[2] == "h3"
                            {
                                return CheckerResult {
                                    name: "Wordnet",
                                    result: Some(false),
                                    is_community: false,
                                };
                            }
                            
                            // Check for found pattern: [div, div] with classes [header, form]
                            if tag_names.len() >= 2 && tag_names[0] == "div" && tag_names[1] == "div" {
                                let class_names: Vec<&str> = tag_nodes
                                    .iter()
                                    .take(2)
                                    .map(|e| e.value().attr("class").unwrap_or(""))
                                    .collect();
                                
                                if class_names[0] == "header" && class_names[1] == "form" {
                                    return CheckerResult {
                                        name: "Wordnet",
                                        result: Some(true),
                                        is_community: false,
                                    };
                                }
                            }
                            
                            // Pattern didn't match
                            CheckerResult {
                                name: "Wordnet",
                                result: None,
                                is_community: false,
                            }
                        }
                        Err(e) => {
                            eprintln!("[ISAWORD/wordnet] {}", e);
                            CheckerResult {
                                name: "Wordnet",
                                result: None,
                                is_community: false,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/wordnet] {}", e);
                    CheckerResult {
                        name: "Wordnet",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/wordnet] {}", e);
            CheckerResult {
                name: "Wordnet",
                result: None,
                is_community: false,
            }
        }
    }
}
