use crate::{Earl, domstroll};
use crate::utils::dom::{DomOpts, find_body_index};

/// Etymonline checker
/// 
/// Complex scraper that navigates to word sections and extracts names to match against input.
/// Returns (bool_or_null, message_string) where bool indicates if word was found.
/// 
/// Logic:
/// - If first 3 children are [h2, p, p], word is not in etymonline (return null)
/// - Otherwise, find all div.word--C9UPa nodes (word sections)
/// - Extract word names from each section
/// - Match against input (case-insensitive)
/// - Return true if any match, false if none match
/// - Append helpful messages
/// 
/// URL structure: https://etymonline.com/word/WORD
/// 
/// Ported from: https://github.com/hippietrail/hippiebot.js/blob/main/ute/etym.ts (line 4-57)
pub async fn etym(word: &str) -> (Option<bool>, String) {
    let result = etym_internal(word).await;
    // Return just the bool, not the message string
    (result.0, result.1)
}

async fn etym_internal(word: &str) -> (Option<bool>, String) {
    match Earl::new("https://etymonline.com", "/word/", None) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.fetch_dom().await {
                 Ok(dom) => {
                     let body_idx = find_body_index(&dom).unwrap_or(2);
                     // Navigate to the main container
                     match domstroll(
                         "etym",
                         false,
                         &dom,
                         &[
                             (body_idx, "body", None),
                            (1, "div", Some(DomOpts { id: Some("root".to_string()), ..Default::default() })),
                            (0, "div", None),
                            (0, "div", Some(DomOpts { cls: Some("container--1mazc".to_string()), ..Default::default() })),
                            (1, "div", Some(DomOpts { cls: Some("main".to_string()), ..Default::default() })),
                            (0, "div", Some(DomOpts { cls: Some("ant-row-flex".to_string()), ..Default::default() })),
                            (0, "div", Some(DomOpts { cls: Some("ant-col-lg-17".to_string()), ..Default::default() })),
                        ],
                    ) {
                        Ok(lg17) => {
                            // Check if word is in etymonline at all
                            let children: Vec<_> = lg17.children().filter_map(|c| {
                                scraper::element_ref::ElementRef::wrap(c)
                            }).collect();
                            
                            // If first 3 are [h2, p, p], word not in etymonline
                            if children.len() >= 3
                                && children[0].value().name() == "h2"
                                && children[1].value().name() == "p"
                                && children[2].value().name() == "p"
                            {
                                return (None, "Not in Etymonline".to_string());
                            }
                            
                            // Find all div.word--C9UPa nodes
                            let word_sections: Vec<_> = children
                                .iter()
                                .filter(|elem| {
                                    if elem.value().name() == "div" {
                                        if let Some(class) = elem.value().attr("class") {
                                            return class.contains("word--C9UPa");
                                        }
                                    }
                                    false
                                })
                                .collect();
                            
                            // Extract word names from each section
                            let mut words = Vec::new();
                            for section in word_sections {
                                // Navigate: div.word--C9UPa > ? > ? > ? (looking for word name)
                                if let Some(child1) = section.children().next() {
                                    if let Some(elem1) = scraper::element_ref::ElementRef::wrap(child1) {
                                        if let Some(child2) = elem1.children().next() {
                                            if let Some(elem2) = scraper::element_ref::ElementRef::wrap(child2) {
                                                // This should be the span with class word__name--TTbAA
                                                if let Some(class) = elem2.value().attr("class") {
                                                    if class.contains("word__name--TTbAA") {
                                                        // Get text content
                                                        if let Some(child3) = elem2.children().next() {
                                                            if let Some(text) = child3.value().as_text() {
                                                                words.push(text.to_string());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Deduplicate
                            let mut unique_words = Vec::new();
                            for w in &words {
                                if !unique_words.contains(w) {
                                    unique_words.push(w.clone());
                                }
                            }
                            
                            let verb = if words.len() == 1 { "is" } else { "are" };
                            let suffix = if words.len() == 1 { "" } else { "s" };
                            let result_string = format!(
                                "[there {} {} word section{}.]({})",
                                verb,
                                words.len(),
                                suffix,
                                earl.get_url_string()
                            );
                            
                            println!("[ute/etym] {} words: {:?}", word, unique_words);
                            
                            // Check if any word matches (case-insensitive)
                            let result_bool = if unique_words
                                .iter()
                                .any(|w| w.to_lowercase() == word.to_lowercase())
                            {
                                // At least one match
                                let mut message = result_string;
                                if unique_words.len() != 1 {
                                    let word_list = unique_words
                                        .iter()
                                        .map(|w| format!("'{}'", w))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    message.push_str(&format!(
                                        "\nat least one of {} matches '{}'.",
                                        word_list, word
                                    ));
                                }
                                (Some(true), message)
                            } else {
                                // No matches
                                let mut message = result_string;
                                if unique_words.len() == 1 {
                                    message.push_str(&format!(
                                        "\nbut '{}' does not match '{}'!",
                                        unique_words[0], word
                                    ));
                                } else {
                                    let word_list = unique_words
                                        .iter()
                                        .map(|w| format!("'{}'", w))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    message.push_str(&format!(
                                        "\nbut none of {} match '{}'!",
                                        word_list, word
                                    ));
                                }
                                (Some(false), message)
                            };
                            
                            result_bool
                        }
                        Err(e) => {
                            eprintln!("[ute/etym] {}", e);
                            (None, format!("An error occurred while fetching data."))
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ute/etym] {}", e);
                    (None, format!("An error occurred while fetching data."))
                }
            }
        }
        Err(e) => {
            eprintln!("[ute/etym] {}", e);
            (None, format!("An error occurred while fetching data."))
        }
    }
}
