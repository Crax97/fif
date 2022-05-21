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
            pattern: Pattern::from_string("the")
        };
        let content = "The quick brown fox
        jumps over the lazy dog";
        let matches = find_in_lines(content.split("\n"), &config);
        assert!(matches.len() == 2);
    }
}

#[cfg(regex)]
#[cfg(test)]
mod regex_tests {
    #[test]
    fn test_find_matching_with_regex() {
        let config = Configuration {
            case_sensitive: false,
            pattern: Regex()
        };
        let content = "The quick brown fox jumps over the lazy dog"; 
        let matches = find_in_lines(content, config);
        assert_eq!(matches.len(), 2);
    }
}