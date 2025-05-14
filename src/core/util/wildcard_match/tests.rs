use crate::core::util::wildcard_match::wildcard_match;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(wildcard_match("hello", "hello"));
        assert!(!wildcard_match("hello", "hell"));
        assert!(!wildcard_match("hello", "ello"));
    }

    #[test]
    fn test_star_wildcard() {
        assert!(wildcard_match("file*.txt", "file.txt"));
        assert!(wildcard_match("file*.txt", "file123.txt"));
        assert!(wildcard_match("file*.txt", "files/doc.txt"));
    }

    #[test]
    fn test_question_wildcard() {
        assert!(wildcard_match("file?.txt", "file1.txt"));
        assert!(wildcard_match("file?.txt", "fileX.txt"));
        assert!(!wildcard_match("file?.txt", "file.txt"));
        assert!(!wildcard_match("file?.txt", "file12.txt"));
    }

    #[test]
    fn test_star_and_question_combined() {
        assert!(wildcard_match("a*b?c", "axyzbxc"));
        assert!(wildcard_match("a*b?c", "ab1c"));
        assert!(!wildcard_match("a*b?c", "abc"));
    }

    #[test]
    fn test_leading_and_trailing_star() {
        assert!(wildcard_match("*test*", "my_test_file"));
        assert!(wildcard_match("*test*", "test"));
        assert!(!wildcard_match("*test*", "tes"));
    }

    #[test]
    fn test_only_star() {
        assert!(wildcard_match("*", ""));
        assert!(wildcard_match("*", "anything"));
    }

    #[test]
    fn test_empty_cases() {
        assert!(wildcard_match("", ""));
        assert!(!wildcard_match("", "nonempty"));
    }

    #[test]
    fn test_consecutive_stars() {
        assert!(wildcard_match("a**b", "acb"));
        assert!(wildcard_match("a***b", "ab"));
    }

    #[test]
    fn test_trailing_question() {
        assert!(wildcard_match("file?", "file1"));
        assert!(!wildcard_match("file?", "file12"));
    }

    #[test]
    fn test_unicode_characters() {
        assert!(wildcard_match("こんにちは*", "こんにちは世界"));
        assert!(wildcard_match("こんにち?世界", "こんにちは世界"));
    }
}
