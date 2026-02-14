use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;
use crate::utils::dom::DomOpts;

/// Wordnik checker
/// 
/// Navigates deep into page structure to find div.guts.active and checks children pattern:
/// - Found: alternating h3/ul tags (any number of pairs)
/// - Not found: div.guts has 3 children where child[1] is p.weak
/// - Anything else: unknown
/// 
/// URL structure: https://www.wordnik.com/words/WORD
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 375-403)
pub async fn wordnik(word: &str) -> CheckerResult {
    match Earl::new("https://www.wordnik.com", "/words/", None) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.fetch_dom().await {
                Ok(dom) => {
                    // Navigate to div.guts (the "active" class is on the parent)
                     match domstroll(
                         "wordnik1",
                         false,
                         &dom,
                         &[
                             (2, "body", None),
                            (3, "div", Some(DomOpts { cls: Some("word_page".to_string()), ..Default::default() })),
                            (1, "div", Some(DomOpts { cls: Some("content".to_string()), ..Default::default() })),
                            (7, "div", Some(DomOpts { cls: Some("module-row".to_string()), ..Default::default() })),
                            (1, "div", Some(DomOpts { cls: Some("module-2columnLeft".to_string()), ..Default::default() })),
                            (1, "div", Some(DomOpts { id: Some("define".to_string()), ..Default::default() })),
                            (3, "div", Some(DomOpts { cls: Some("guts".to_string()), ..Default::default() })),
                        ],
                    ) {
                        Ok(guts_elem) => {
                            let children: Vec<_> = guts_elem.children().collect();
                            let tag_count = children.iter().filter(|c| {
                                scraper::element_ref::ElementRef::wrap(**c).is_some()
                            }).count();
                            println!("[ISAWORD/wordnik] {} {} kids, {} are tags", word, children.len(), tag_count);
                            
                            // Check pattern 1: alternating h3/ul tags
                            let tag_children: Vec<_> = children
                                .iter()
                                .filter_map(|child| scraper::element_ref::ElementRef::wrap(*child))
                                .collect();
                            
                            let tag_names: Vec<&str> = tag_children.iter().map(|e| e.value().name()).collect();
                            
                            // Check if alternating h3/ul pattern
                            if !tag_names.is_empty() {
                                let is_alternating = tag_names.iter().enumerate().all(|(i, name)| {
                                    match i % 2 {
                                        0 => name == &"h3",
                                        _ => name == &"ul",
                                    }
                                });
                                
                                if is_alternating {
                                    return CheckerResult {
                                        name: "Wordnik",
                                        result: Some(true),
                                        is_community: false,
                                    };
                                }
                            }
                            
                            // Check pattern 2: 3 children with [1] being p.weak
                            if children.len() == 3 {
                                if let Some(child1) = children.get(1) {
                                    if let Some(elem) = scraper::element_ref::ElementRef::wrap(*child1) {
                                        if elem.value().name() == "p" {
                                            if let Some(class) = elem.value().attr("class") {
                                                if class == "weak" {
                                                    return CheckerResult {
                                                        name: "Wordnik",
                                                        result: Some(false),
                                                        is_community: false,
                                                    };
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Pattern didn't match
                            CheckerResult {
                                name: "Wordnik",
                                result: None,
                                is_community: false,
                            }
                        }
                        Err(e) => {
                            eprintln!("[ISAWORD/wordnik] {}", e);
                            CheckerResult {
                                name: "Wordnik",
                                result: None,
                                is_community: false,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/wordnik] {}", e);
                    CheckerResult {
                        name: "Wordnik",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/wordnik] {}", e);
            CheckerResult {
                name: "Wordnik",
                result: None,
                is_community: false,
            }
        }
    }
}
