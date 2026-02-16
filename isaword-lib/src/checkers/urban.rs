use crate::Earl;
use crate::checkers::CheckerResult;
use std::collections::HashMap;

/// Urban Dictionary checker - simple JSON API
pub async fn urban(word: &str) -> CheckerResult {
    let mut params = HashMap::new();
    params.insert("term", word.to_string());

    match Earl::new("https://api.urbandictionary.com", "/v0/define", Some(params)) {
        Ok(earl) => {
            match earl.fetch_json().await {
                Ok(data) => {
                    if let Some(list) = data.get("list").and_then(|l| l.as_array()) {
                        let found = !list.is_empty();
                        println!("[ISAWORD/urban] {} status: {}", word, list.len());
                        CheckerResult {
                            name: "Urban Dictionary",
                            result: Some(found),
                            is_community: true,
                        }
                    } else {
                        println!("[ISAWORD/urban] {} - no list in response", word);
                        CheckerResult {
                            name: "Urban Dictionary",
                            result: None,
                            is_community: true,
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ISAWORD/urban] {}", e);
                    CheckerResult {
                        name: "Urban Dictionary",
                        result: None,
                        is_community: true,
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[ISAWORD/urban] {}", e);
            CheckerResult {
                name: "Urban Dictionary",
                result: None,
                is_community: true,
            }
        }
    }
}
