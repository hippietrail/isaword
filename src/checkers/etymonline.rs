use crate::checkers::CheckerResult;
use super::etym;

/// Etymonline checker
/// 
/// Wrapper around the etym() helper that returns only the boolean result
/// (discards the message string).
/// 
/// Ported from: /Users/hippietrail/hippiebot.js/commands/isaword.js (line 434-438)
pub async fn etymonline(word: &str) -> CheckerResult {
    let (result, _message) = etym::etym(word).await;
    
    println!("[ISAWORD/etymonline] {} result: {:?}", word, result);
    
    CheckerResult {
        name: "Etymonline",
        result,
        is_community: false,
    }
}
