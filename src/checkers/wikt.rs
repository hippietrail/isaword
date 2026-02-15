use crate::Earl;
use crate::checkers::CheckerResult;
use std::collections::HashMap;

/// Wiktionary checker
/// 
/// Queries the Wiktionary API (Mediawiki) to check if a word exists.
/// Returns 0 if missing, 1 if found, null if error.
/// Called with language code 'en' for English.
/// 
/// Note: API returns a JSON object with nested 'query.pages' structure.
/// Each page either has 'pageid' (exists) or 'missing' (doesn't exist).
/// 
/// Ported from: https://github.com/hippietrail/hippiebot.js/blob/main/ute/wikt.ts (line 36-65)
pub async fn wikt(wiki_lang: &str, word: &str) -> CheckerResult {
    match wikt_internal(wiki_lang, word).await {
        (Some(link), Some(result)) => {
            let human_link = format!("https://{}.wiktionary.org/wiki/{}", wiki_lang, word);
            println!(
                "[wikt] ignoring link: {} -> {}",
                link, human_link
            );
            CheckerResult {
                name: "Wiktionary",
                result: match result {
                    1 => Some(true),
                    0 => Some(false),
                    _ => None,
                },
                is_community: true,
            }
        }
        (Some(link), None) => {
            println!("[wikt] error response, link was: {}", link);
            CheckerResult {
                name: "Wiktionary",
                result: None,
                is_community: true,
            }
        }
        (None, _) => CheckerResult {
            name: "Wiktionary",
            result: None,
            is_community: true,
        },
    }
}

/// Internal Wiktionary API query
/// Returns (url_string, result) where result is 0=missing, 1=found, None=error
async fn wikt_internal(wiki_lang: &str, word: &str) -> (Option<String>, Option<u8>) {
    let mut params = HashMap::new();
    params.insert("action", "query".to_string());
    params.insert("format", "json".to_string());
    params.insert("titles", word.to_string());

    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "isaword/1.0 (+https://github.com/hippietrail/isaword)".to_string());

    let origin = format!("https://{}.wiktionary.org", wiki_lang);

    match Earl::with_headers(&origin, "/w/api.php", Some(params), headers) {
        Ok(earl) => {
            let link = earl.get_url_string();

            match earl.fetch_json().await {
                Ok(data) => {
                    // Navigate: data.query.pages
                    if let Some(query) = data.get("query") {
                        if let Some(pages) = query.get("pages") {
                            if let Some(pages_obj) = pages.as_object() {
                                // Should have exactly one page entry
                                if pages_obj.len() == 1 {
                                    if let Some(page) = pages_obj.values().next() {
                                        // Check for 'pageid' (found) or 'missing' (not found)
                                        if page.get("pageid").is_some() {
                                            return (Some(link), Some(1));
                                        } else if page.get("missing").is_some() {
                                            return (Some(link), Some(0));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Couldn't extract expected structure
                    (Some(link), None)
                }
                Err(e) => {
                    eprintln!("[wikt] {}", e);
                    (Some(link), None)
                }
            }
        }
        Err(e) => {
            eprintln!("[wikt] {}", e);
            (None, None)
        }
    }
}
