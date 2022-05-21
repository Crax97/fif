#[cfg(test)]
mod tests {
    use crate::fif::*;

    #[test]
    fn test_find_matching_string() {
        let config = Configuration::default_from_pattern("the");
        let content = "The quick brown fox
        jumps over the lazy dog";
        let matches = find_in_lines(content.split("\n"), &config);
        assert!(matches.len() == 1);
    }

    #[test]
    fn test_find_matching_string_case_insensitive() {
        let config = Configuration {
            case_sensitive: false,
            pattern: "the".to_string()
        };
        let content = "The quick brown fox
        jumps over the lazy dog";
        let matches = find_in_lines(content.split("\n"), &config);
        assert!(matches.len() == 2);
    }
}