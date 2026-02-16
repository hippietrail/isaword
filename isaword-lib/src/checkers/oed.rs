use crate::Earl;
use crate::checkers::CheckerResult;
use std::collections::HashMap;

/// Oxford English Dictionary checker
/// 
/// Uses the OED's autocomplete API to check if a word exists.
/// API returns an array of suggestions with metadata.
/// We match if any suggestion's name or label matches the input (case-sensitive).
/// 
/// Ported from: https://github.com/hippietrail/hippiebot.js/blob/main/commands/isaword.js (line 403-433)
pub async fn oed(word: &str) -> CheckerResult {
    let mut params = HashMap::new();
    params.insert("q", word.to_string());

    match Earl::new("https://www.oed.com", "/autocomplete/dictionary/", Some(params)) {
        Ok(earl) => {
            match earl.fetch_json().await {
                Ok(data) => {
                    // Should be an array
                    if let Some(array) = data.as_array() {
                        println!("[ISAWORD/oed] {} length: {}", word, array.len());

                        // Check each element for word match
                        let found = array.iter().any(|element| {
                            // Debug: print unexpected structure
                            if let Some(count) = element.get("count").and_then(|c| c.as_u64()) {
                                if count > 12 {
                                    println!("[ISAWORD/oed] {} count: {}", word, count);
                                }
                            }

                            if let Some(path) = element.get("path") {
                                if !path.is_null() {
                                    println!("[ISAWORD/oed] {} path: {}", word, path);
                                }
                            }

                            let name = element.get("name").and_then(|n| n.as_str()).unwrap_or("");
                            let label = element
                                .get("label")
                                .and_then(|l| l.as_str())
                                .unwrap_or("");
                            if name != label {
                                println!("[ISAWORD/oed] {} name: {}, label: {}", word, name, label);
                            }

                            let key_count = element.as_object().map(|o| o.len()).unwrap_or(0);
                            if key_count != 4 {
                                let keys: Vec<&str> = element
                                    .as_object()
                                    .map(|o| o.keys().map(|k| k.as_str()).collect())
                                    .unwrap_or_default();
                                println!(
                                    "[ISAWORD/oed] {} unexpected keys: {}",
                                    word,
                                    keys.join(", ")
                                );
                            }

                            // Match logic
                            if name == word {
                                return true;
                            }
                            if label == word {
                                return true;
                            }

                            false
                        });

                        CheckerResult {
                            name: "OED",
                            result: Some(found),
                            is_community: false,
                        }
                    } else {
                        println!("[ISAWORD/oed] {} - response not an array", word);
                        CheckerResult {
                            name: "OED",
                            result: None,
                            is_community: false,
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/oed] {}", e);
                    CheckerResult {
                        name: "OED",
                        result: None,
                        is_community: false,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/oed] {}", e);
            CheckerResult {
                name: "OED",
                result: None,
                is_community: false,
            }
        }
    }
}
