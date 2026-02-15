//! isaword - A Rust CLI tool for checking if a word exists in multiple dictionaries
//! 
//! This is a direct port of the `/isaword2` Discord bot slash command from:
//! - https://github.com/hippietrail/hippiebot.js/blob/main/commands/isaword.js
//! - https://github.com/hippietrail/lyre/blob/main/commands/isaword.js
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
pub use utils::theme::{get_theme, format_dict_colored};

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

/// Format results and print to stdout with colored output
/// 
/// Mirrors the TypeScript isaword function logic exactly (line 38-72)
/// but adds per-dictionary colors/emojis for visual distinction
pub async fn format_and_print_results(word: &str, results: &[CheckerResult]) {
    // Categorize results with full result objects for colored output
    let ins: Vec<&CheckerResult> = results
        .iter()
        .filter(|r| r.result == Some(true))
        .collect();

    let notins: Vec<&CheckerResult> = results
        .iter()
        .filter(|r| r.result == Some(false))
        .collect();

    let nulls: Vec<&CheckerResult> = results
        .iter()
        .filter(|r| r.result.is_none())
        .collect();

    let community_dict_count = results
        .iter()
        .filter(|r| r.is_community && r.result == Some(true))
        .count();

    let pro_dict_count = results
        .iter()
        .filter(|r| !r.is_community && r.result == Some(true))
        .count();

    // Log results with colored output
    if !ins.is_empty() {
        let ins_colored: Vec<String> = ins
            .iter()
            .map(|r| format_dict_colored(r.name, r.result))
            .collect();
        let ins_formatted: Vec<&str> = ins_colored.iter().map(|s| s.as_str()).collect();
        println!(
            "[ISAWORD] in: {}",
            human_friendly_list_formatter(&ins_formatted, "and")
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
        let notins_colored: Vec<String> = notins
            .iter()
            .map(|r| format_dict_colored(r.name, r.result))
            .collect();
        let notins_formatted: Vec<&str> = notins_colored.iter().map(|s| s.as_str()).collect();
        println!(
            "[ISAWORD] not in: {}",
            human_friendly_list_formatter(&notins_formatted, "and")
        );
    }

    if !nulls.is_empty() {
        let nulls_colored: Vec<String> = nulls
            .iter()
            .map(|r| format_dict_colored(r.name, r.result))
            .collect();
        let nulls_formatted: Vec<&str> = nulls_colored.iter().map(|s| s.as_str()).collect();
        println!(
            "[ISAWORD] null: {}",
            human_friendly_list_formatter(&nulls_formatted, "and")
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
                word, ins[0].name
            )
        } else {
            format!(
                "Hmm '{}' is in {}, but not in any other dictionary!",
                word, ins[0].name
            )
        }
    } else {
        // In multiple dictionaries
        if community_dict_count == ins.len() {
            let ins_names: Vec<&str> = ins.iter().map(|r| r.name).collect();
            format!(
                "'{}' is only in {} but not in any professionally edited dictionary!",
                word,
                human_friendly_list_formatter(&ins_names, "and")
            )
        } else if !notins.is_empty() {
            let ins_names: Vec<&str> = ins.iter().map(|r| r.name).collect();
            let notins_names: Vec<&str> = notins.iter().map(|r| r.name).collect();
            format!(
                "'{}' is in {} but not in {}",
                word,
                human_friendly_list_formatter(&ins_names, "and"),
                human_friendly_list_formatter(&notins_names, "or")
            )
        } else {
            let ins_names: Vec<&str> = ins.iter().map(|r| r.name).collect();
            format!(
                "'{}' is in {} at least...",
                word,
                human_friendly_list_formatter(&ins_names, "and")
            )
        }
    };

    println!("\n{}", message);
}

/// Run the word checker against a single dictionary
pub async fn run_cli_single(word: &str, dictionary: &str) {
    let result = checker_single(word, dictionary).await;
    
    match result {
        Some(result) => {
            println!("[ISAWORD] Checking '{}' in {}", word, result.name);
            let status_line = format_dict_colored(result.name, result.result);
            println!("{}", status_line);
        }
        None => {
            eprintln!("[ISAWORD] Unknown dictionary: {}", dictionary);
            eprintln!("Available dictionaries:");
            eprintln!("  - american-heritage");
            eprintln!("  - cambridge");
            eprintln!("  - chambers");
            eprintln!("  - dictionary-com");
            eprintln!("  - etymonline");
            eprintln!("  - longman");
            eprintln!("  - merriam-webster");
            eprintln!("  - oed");
            eprintln!("  - oxford-learners");
            eprintln!("  - wordnet");
            eprintln!("  - wordnik");
            eprintln!("  - wiktionary");
            eprintln!("  - urban-dictionary");
            std::process::exit(1);
        }
    }
}

/// Check a word against a single dictionary
fn checker_single_name_to_lowercase(name: &str) -> String {
    name.to_lowercase().replace("_", "-")
}

pub async fn checker_single(word: &str, dictionary: &str) -> Option<CheckerResult> {
    let dict = checker_single_name_to_lowercase(dictionary);
    
    match dict.as_str() {
        "american-heritage" | "american_heritage" | "ahd" => {
            Some(checkers::ahd::ahd(word).await)
        }
        "cambridge" => {
            Some(checkers::cambridge::cambridge(word).await)
        }
        "chambers" => {
            Some(checkers::chambers::chambers(word).await)
        }
        "dictionary-com" | "dictionary_com" | "dictcom" => {
            Some(checkers::dictcom::dictcom(word).await)
        }
        "etymonline" | "etym" => {
            Some(checkers::etymonline::etymonline(word).await)
        }
        "longman" => {
            Some(checkers::longman::longman(word).await)
        }
        "merriam-webster" | "merriam_webster" | "mw" => {
            Some(checkers::mw::mw(word).await)
        }
        "oed" => {
            Some(checkers::oed::oed(word).await)
        }
        "oxford-learners" | "oxford_learners" | "oxford" | "oxlearn" => {
            Some(checkers::oxfordlearners::oxfordlearners(word).await)
        }
        "wordnet" => {
            Some(checkers::wordnet::wordnet(word).await)
        }
        "wordnik" => {
            Some(checkers::wordnik::wordnik(word).await)
        }
        "wiktionary" | "wikt" => {
            Some(checkers::wikt::wikt("en", word).await)
        }
        "urban-dictionary" | "urban_dictionary" | "urban" => {
            Some(checkers::urban::urban(word).await)
        }
        _ => None,
    }
}
