/// Format a list of strings into human-friendly output
/// 
/// Examples:
/// [] -> ""
/// ["a"] -> "a"
/// ["a", "b"] -> "a and b"
/// ["a", "b", "c"] -> "a, b, and c"
pub fn human_friendly_list_formatter(items: &[&str], conjunction: &str) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].to_string(),
        2 => format!("{} {} {}", items[0], conjunction, items[1]),
        _ => {
            let all_but_last = items[..items.len() - 1].join(", ");
            format!("{}, {} {}", all_but_last, conjunction, items[items.len() - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(human_friendly_list_formatter(&[], "and"), "");
    }

    #[test]
    fn test_single() {
        assert_eq!(human_friendly_list_formatter(&["apple"], "and"), "apple");
    }

    #[test]
    fn test_two() {
        assert_eq!(
            human_friendly_list_formatter(&["apple", "banana"], "and"),
            "apple and banana"
        );
    }

    #[test]
    fn test_three() {
        assert_eq!(
            human_friendly_list_formatter(&["apple", "banana", "cherry"], "and"),
            "apple, banana, and cherry"
        );
    }

    #[test]
    fn test_with_or() {
        assert_eq!(
            human_friendly_list_formatter(&["a", "b", "c"], "or"),
            "a, b, or c"
        );
    }
}
