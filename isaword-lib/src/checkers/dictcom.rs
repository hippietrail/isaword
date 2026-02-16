use crate::{Earl, domstroll};
use crate::checkers::CheckerResult;
use crate::utils::dom::{DomOpts, find_body_index};

/// Dictionary.com checker - CURRENTLY BLOCKED BY BOT DETECTION
/// 
/// STATUS: ❌ BROKEN - Every request returns 404 page regardless of word or headers
/// 
/// ROOT CAUSE: Dictionary.com is blocking reqwest/Rust HTTP clients at the network level.
/// - All requests get "404: Not found | Dictionary.com" response with pg-404 body class
/// - Tested with browser User-Agent header: still 404
/// - `curl` from CLI works fine (200 OK) - same IP
/// - Node.js `fetch` (used in TypeScript version) may have different IP routing
/// - Not a Cloudflare challenge (no cf-challenge headers in 404 response)
/// - Likely: reqwest's IP range is blacklisted for bot activity
/// 
/// POSSIBLE FIXES (not yet attempted):
/// 1. Residential proxy (would mask IP range)
/// 2. Headless browser (Playwright/Puppeteer) - adds JS rendering overhead
/// 3. Find alternative API endpoint
/// 4. Accept that Dictionary.com is unavailable for automated checking
/// 
/// See isaword-bnk for issue tracking.
/// 
/// TypeScript version has better detection logic (isaword.ts lines 348-391) but same network block:
/// - Check for "pg-dcom-noresult" body class -> not found
/// - Check for "sec-redirect-tip" in box-content-primary children -> misspelling
/// 
/// Ported from: https://github.com/hippietrail/hippiebot.js/blob/main/commands/isaword.js (line 327-350)
pub async fn dictcom(word: &str) -> CheckerResult {
    // Try with a browser User-Agent to avoid bot detection
    let mut headers = std::collections::HashMap::new();
    headers.insert("User-Agent".to_string(), 
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string());
    
    match Earl::with_headers("https://www.dictionary.com", "/browse/", None, headers) {
        Ok(mut earl) => {
            earl.set_last_path_segment(word);
            
            match earl.fetch_text().await {
                 Ok(text) => {
                     // Debug: print raw HTML
                     eprintln!("[ISAWORD/dict.com] Raw text (first 1500 chars):\n{}\n---", 
                               &text.chars().take(1500).collect::<String>());
                     let dom = scraper::Html::parse_document(&text);
                     
                     let body_idx = find_body_index(&dom).unwrap_or(2);
                     // Navigate: html > body > #root > .dictionary-site > main
                     match domstroll(
                         "dict.com",
                         true,
                         &dom,
                         &[
                             (body_idx, "body", None),
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
