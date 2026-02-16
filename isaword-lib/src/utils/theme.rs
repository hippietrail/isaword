use colored::Colorize;

/// Dictionary theme with color, emoji, and branding
pub struct DictTheme {
    pub emoji: &'static str,
    pub color_name: &'static str, // For reference
}

/// Get theme for a dictionary by name
pub fn get_theme(dict_name: &str) -> DictTheme {
    match dict_name {
        // Professional dictionaries with color-coded themes
        "American Heritage" => DictTheme {
            emoji: "🔵",     // Blue - professional blue tone
            color_name: "blue",
        },
        "Cambridge" => DictTheme {
            emoji: "🟢",     // Green - Cambridge logo is green
            color_name: "green",
        },
        "Chambers" => DictTheme {
            emoji: "🟣",     // Purple - distinguished/classic
            color_name: "magenta",
        },
        "Dictionary.com" => DictTheme {
            emoji: "🟠",     // Orange - energetic web color
            color_name: "yellow",
        },
        "Etymonline" => DictTheme {
            emoji: "📚",     // Books - etymology focus
            color_name: "cyan",
        },
        "Longman" => DictTheme {
            emoji: "📖",     // Book - learner's dict
            color_name: "white",
        },
        "Merriam-Webster" => DictTheme {
            emoji: "🔴",     // Red - Webster's traditional
            color_name: "red",
        },
        "OED" => DictTheme {
            emoji: "👑",     // Crown - authoritative/prestigious
            color_name: "bright_magenta",
        },
        "Oxford Learners" => DictTheme {
            emoji: "🎓",     // Academic - learner's version
            color_name: "bright_cyan",
        },
        "Wordnet" => DictTheme {
            emoji: "🧠",     // Brain - linguistic AI
            color_name: "bright_blue",
        },
        "Wordnik" => DictTheme {
            emoji: "💬",     // Speech bubble - community-driven
            color_name: "bright_green",
        },
        "Wiktionary" => DictTheme {
            emoji: "🌍",     // Globe - collaborative/community
            color_name: "bright_yellow",
        },
        "Urban Dictionary" => DictTheme {
            emoji: "🔥",     // Fire - urban/slang
            color_name: "bright_red",
        },
        _ => DictTheme {
            emoji: "❓",
            color_name: "white",
        },
    }
}

/// Format a dictionary name with color and emoji
pub fn format_dict_colored(dict_name: &str, found: Option<bool>) -> String {
    let theme = get_theme(dict_name);
    let status = match found {
        Some(true) => "✓".green(),
        Some(false) => "✗".red(),
        None => "?".yellow(),
    };

    let colored_name = match theme.color_name {
        "blue" => dict_name.blue(),
        "green" => dict_name.green(),
        "magenta" => dict_name.magenta(),
        "yellow" => dict_name.yellow(),
        "cyan" => dict_name.cyan(),
        "white" => dict_name.white(),
        "red" => dict_name.red(),
        "bright_magenta" => dict_name.bright_magenta(),
        "bright_cyan" => dict_name.bright_cyan(),
        "bright_blue" => dict_name.bright_blue(),
        "bright_green" => dict_name.bright_green(),
        "bright_yellow" => dict_name.bright_yellow(),
        "bright_red" => dict_name.bright_red(),
        _ => dict_name.white(),
    };

    format!("{}{} {}", theme.emoji, status, colored_name)
}
