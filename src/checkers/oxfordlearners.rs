use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;
use crate::utils::dom::DomOpts;

/// Oxford Learners Dictionary checker
/// 
/// Performs extremely deep DOM navigation (10+ levels) to find entry metadata.
/// If the .xenglish div is not present, word not found.
/// If .webtop div is found (indicating entry content), word is found.
/// 
/// URL structure: https://www.oxfordlearnersdictionaries.com/definition/english/WORD
/// 
/// NOTE: TS version has TODO about user input concatenation, which we avoid by
/// using setLastPathSegment pattern. Also, this is a very fragile selector
/// due to deep nesting and reliance on specific class names.
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 171-209)
pub async fn oxfordlearners(word: &str) -> CheckerResult {
    match Earl::new(
        "https://www.oxfordlearnersdictionaries.com",
        "/definition/english/",
        None,
    ) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.fetch_dom().await {
                Ok(dom) => {
                    // Step 1: Navigate to ox-container
                     match domstroll(
                         "ox",
                         false,
                         &dom,
                         &[
                             (2, "body", None),
                             (1, "div", Some(DomOpts { id: Some("ox-container".to_string()), ..Default::default() })),
                         ],
                     ) {
                        Ok(_ox_container) => {
                            // Step 2: Navigate through ox_container's children to find xenglish
                             match domstroll(
                                 "ox",
                                 false,
                                 &dom,
                                 &[
                                     (2, "body", None),
                                     (1, "div", Some(DomOpts { id: Some("ox-container".to_string()), ..Default::default() })),
                                     (5, "div", Some(DomOpts { cls: Some("xenglish".to_string()), optional: true, ..Default::default() })),
                                 ],
                             ) {
                                Ok(_xenglish) => {
                                    // xenglish found, continue to find webtop
                                     match domstroll(
                                         "ox",
                                         false,
                                         &dom,
                                         &[
                                             (2, "body", None),
                                             (1, "div", Some(DomOpts { id: Some("ox-container".to_string()), ..Default::default() })),
                                             (5, "div", Some(DomOpts { cls: Some("xenglish".to_string()), ..Default::default() })),
                                            (3, "div", Some(DomOpts { cls: Some("responsive_row".to_string()), ..Default::default() })),
                                            (3, "div", Some(DomOpts { cls: Some("responsive_entry_center".to_string()), ..Default::default() })),
                                            (1, "div", Some(DomOpts { cls: Some("responsive_entry_center_wrap".to_string()), ..Default::default() })),
                                            (3, "div", Some(DomOpts { id: Some("ox-wrapper".to_string()), ..Default::default() })),
                                            (1, "div", Some(DomOpts { id: Some("main_column".to_string()), ..Default::default() })),
                                            (1, "div", Some(DomOpts { id: Some("main-container".to_string()), ..Default::default() })),
                                            (5, "div", Some(DomOpts { id: Some("entryContent".to_string()), ..Default::default() })),
                                            (0, "div", None), // id will be WORD
                                            (0, "div", Some(DomOpts { cls: Some("top-container".to_string()), ..Default::default() })),
                                            (0, "div", None), // id will be 'WORD_topg_N'
                                            (0, "div", Some(DomOpts { cls: Some("webtop".to_string()), ..Default::default() })),
                                        ],
                                    ) {
                                        Ok(webtop) => {
                                            // Extract parent IDs for debug logging
                                            if let Some(parent_node) = webtop.parent() {
                                                if let Some(parent_parent_node) = parent_node.parent() {
                                                    if let Some(ppp_node) = parent_parent_node.parent() {
                                                        if let Some(parent) = scraper::element_ref::ElementRef::wrap(parent_node) {
                                                            if let Some(pppelem) = scraper::element_ref::ElementRef::wrap(ppp_node) {
                                                                let pid = parent.value().attr("id").unwrap_or("?");
                                                                let pppid = pppelem.value().attr("id").unwrap_or("?");
                                                                println!("[ISAWORD/oxlearn] {} pid: '{}', pppid: '{}'", word, pid, pppid);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            CheckerResult {
                                                name: "Oxford Learners",
                                                result: Some(true),
                                                is_community: false,
                                            }
                                        }
                                        Err(_) => {
                                            println!("[ISAWORD/oxlearn] {} - could not find webtop", word);
                                            CheckerResult {
                                                name: "Oxford Learners",
                                                result: Some(false),
                                                is_community: false,
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    // xenglish not found
                                    println!("[ISAWORD/oxlearn] {} no 'xenglish' class", word);
                                    CheckerResult {
                                        name: "Oxford Learners",
                                        result: Some(false),
                                        is_community: false,
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[ISAWORD/oxlearn] {}", e);
                            CheckerResult {
                                name: "Oxford Learners",
                                result: None,
                                is_community: false,
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/oxlearn] {}", e);
                    CheckerResult {
                        name: "Oxford Learners",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/oxlearn] {}", e);
            CheckerResult {
                name: "Oxford Learners",
                result: None,
                is_community: false,
            }
        }
    }
}
