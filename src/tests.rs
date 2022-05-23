#[cfg(test)]
mod tests {
    use crate::fif::*;

    #[test]
    fn test_find_matching_string() {
        let config = Configuration::default_from_pattern("the");
        let content = "The quick brown fox
        jumps over the lazy dog";
        let matches = find_in_lines(content.split("\n"), &config);
        assert!(matches.count() == 1);
    }

    #[test]
    fn test_find_matching_string_case_insensitive() {
        let config = Configuration {
            case_insensitive: true,
            pattern: Pattern::text_from_string("the")
        };
        let content = "The quick brown fox
        jumps over the lazy dog";
        let matches = find_in_lines(content.split("\n"), &config);
        assert!(matches.count() == 2);
    }

    #[test]
    fn test_line_number_is_correct() {
        let config = Configuration {
            case_insensitive: true,
            pattern: Pattern::text_from_string("int")
        };
        let content = "#include <stdio.h>
        int main() {
           // printf() displays the string inside quotation
           printf(\"Hello, World!\");
           return 0;
        }";
        let mut matches = find_in_lines(content.split("\n"), &config);
        
        assert_eq!(matches.next().unwrap().row, 2);
        assert_eq!(matches.next().unwrap().row, 3);
        assert_eq!(matches.next().unwrap().row, 4);
        assert!(matches.next().is_none());
    }
    
    #[cfg(feature = "regex")]
    mod regex_tests {
        use crate::fif::*;

        #[test]
        fn test_find_matching_with_regex() {
            let config = Configuration {
                case_insensitive: false,
                pattern: Pattern::regex_from_string(r".?the.?")
            };
            let content = "The quick brown
            fox jumps over
            the lazy dog"; 
            let matches = find_in_lines(content.split("\n"), &config);
            assert_eq!(matches.count(), 1);
        }
        
        #[test]
        fn test_find_matching_with_regex_case_insensitive() {
            let config = Configuration {
                case_insensitive: true,
                pattern: Pattern::regex_from_string(r".?the.?")
            };
            let content = "The quick brown
            fox jumps over the lazy dog"; 
            let matches = find_in_lines(content.split("\n"), &config);
            assert_eq!(matches.count(), 2);
        }
    }
}