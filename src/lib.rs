//! isaword - A Rust CLI tool for checking if a word exists in multiple dictionaries
//! 
//! This is a direct port of the `/isaword2` Discord bot slash command from:
//! - /Users/hippietrail/hippiebot.js/commands/isaword.js
//! - /Users/hippietrail/lyre/commands/isaword.js
//! 
//! Both Discord bot implementations are identical. This Rust version maintains
//! the same logic for checking 14 different dictionaries in parallel and
//! formatting results with the same human-friendly output.

pub mod utils;
pub mod checkers;

pub use utils::earl::Earl;
pub use utils::dom::domstroll;
pub use checkers::CheckerResult;
pub use utils::format::human_friendly_list_formatter;

/// Run the word checker - main entry point for the CLI
pub async fn run_cli(word: &str) {
    let results = checker_all(word).await;
    format_and_print_results(word, &results).await;
}

/// Check a word against all 13 dictionaries in parallel
/// 
/// Returns results in order:
/// 1. American Heritage
/// 2. Cambridge
/// 3. Chambers
/// 4. Dictionary.com
/// 5. Etymonline
/// 6. Longman
/// 7. Merriam-Webster
/// 8. OED
/// 9. Oxford Learners
/// 10. Wordnet
/// 11. Wordnik
/// 12. Wiktionary (community)
/// 13. Urban Dictionary (community)
pub async fn checker_all(word: &str) -> Vec<CheckerResult> {
    // Execute all checkers concurrently
    // Use tokio::join! macro which supports different future types
    let (ahd, cambridge, chambers, dictcom, etymonline, longman, mw, oed, oxfordlearners, wordnet, wordnik, wikt_en, urban) = tokio::join!(
        checkers::ahd::ahd(word),
        checkers::cambridge::cambridge(word),
        checkers::chambers::chambers(word),
        checkers::dictcom::dictcom(word),
        checkers::etymonline::etymonline(word),
        checkers::longman::longman(word),
        checkers::mw::mw(word),
        checkers::oed::oed(word),
        checkers::oxfordlearners::oxfordlearners(word),
        checkers::wordnet::wordnet(word),
        checkers::wordnik::wordnik(word),
        checkers::wikt::wikt("en", word),
        checkers::urban::urban(word),
    );

    vec![
        ahd, cambridge, chambers, dictcom, etymonline, longman, mw, oed,
        oxfordlearners, wordnet, wordnik, wikt_en, urban,
    ]
}

/// Format results and print to stdout
/// 
/// Mirrors the TypeScript isaword function logic exactly (line 38-72)
pub async fn format_and_print_results(word: &str, results: &[CheckerResult]) {
    // Categorize results
    let ins: Vec<&str> = results
        .iter()
        .filter_map(|r| if r.result == Some(true) { Some(r.name) } else { None })
        .collect();

    let notins: Vec<&str> = results
        .iter()
        .filter_map(|r| if r.result == Some(false) { Some(r.name) } else { None })
        .collect();

    let nulls: Vec<&str> = results
        .iter()
        .filter_map(|r| if r.result.is_none() { Some(r.name) } else { None })
        .collect();

    let community_dict_count = results
        .iter()
        .filter(|r| r.is_community && r.result == Some(true))
        .count();

    let pro_dict_count = results
        .iter()
        .filter(|r| !r.is_community && r.result == Some(true))
        .count();

    // Log results
    if !ins.is_empty() {
        println!(
            "[ISAWORD] in: {}",
            human_friendly_list_formatter(&ins, "and")
        );
    } else {
        println!("[ISAWORD] in: none");
    }

    if !ins.is_empty() {
        println!(
            "[ISAWORD] in {} professional dictionaries vs {} community dictionaries",
            pro_dict_count, community_dict_count
        );
    }

    if !notins.is_empty() {
        println!(
            "[ISAWORD] not in: {}",
            human_friendly_list_formatter(&notins, "and")
        );
    }

    if !nulls.is_empty() {
        println!(
            "[ISAWORD] null: {}",
            human_friendly_list_formatter(&nulls, "and")
        );
    }

    // Format response message
    let message = if ins.is_empty() {
        format!("No sign of '{}' in any dictionary I checked!", word)
    } else if ins.len() == results.len() {
        format!("'{}' is in every dictionary I checked!", word)
    } else if ins.len() == 1 {
        // Only in one dictionary
        if pro_dict_count == 0 {
            format!(
                "'{}' is only in {}, not in any professionally edited dictionary!",
                word, ins[0]
            )
        } else {
            format!(
                "Hmm '{}' is in {}, but not in any other dictionary!",
                word, ins[0]
            )
        }
    } else {
        // In multiple dictionaries
        if community_dict_count == ins.len() {
            format!(
                "'{}' is only in {} but not in any professionally edited dictionary!",
                word,
                human_friendly_list_formatter(&ins, "and")
            )
        } else if !notins.is_empty() {
            format!(
                "'{}' is in {} but not in {}",
                word,
                human_friendly_list_formatter(&ins, "and"),
                human_friendly_list_formatter(&notins, "or")
            )
        } else {
            format!(
                "'{}' is in {} at least...",
                word,
                human_friendly_list_formatter(&ins, "and")
            )
        }
    };

    println!("\n{}", message);
}
