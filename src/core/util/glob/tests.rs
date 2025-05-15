use crate::core::util::glob::glob_match;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(glob_match("hello", "hello"));
        assert!(!glob_match("hello", "hell"));
        assert!(!glob_match("hello", "ello"));
    }

    #[test]
    fn test_star_wildcard() {
        assert!(glob_match("file*.txt", "file.txt"));
        assert!(glob_match("file*.txt", "file123.txt"));
        assert!(glob_match("file*.txt", "files/doc.txt"));
    }

    #[test]
    fn test_question_wildcard() {
        assert!(glob_match("file?.txt", "file1.txt"));
        assert!(glob_match("file?.txt", "fileX.txt"));
        assert!(!glob_match("file?.txt", "file.txt"));
        assert!(!glob_match("file?.txt", "file12.txt"));
    }

    #[test]
    fn test_star_and_question_combined() {
        assert!(glob_match("a*b?c", "axyzbxc"));
        assert!(glob_match("a*b?c", "ab1c"));
        assert!(!glob_match("a*b?c", "abc"));
    }

    #[test]
    fn test_leading_and_trailing_star() {
        assert!(glob_match("*test*", "my_test_file"));
        assert!(glob_match("*test*", "test"));
        assert!(!glob_match("*test*", "tes"));
    }

    #[test]
    fn test_only_star() {
        assert!(glob_match("*", ""));
        assert!(glob_match("*", "anything"));
    }

    #[test]
    fn test_empty_cases() {
        assert!(glob_match("", ""));
        assert!(!glob_match("", "nonempty"));
    }

    #[test]
    fn test_consecutive_stars() {
        assert!(glob_match("a**b", "acb"));
        assert!(glob_match("a***b", "ab"));
    }

    #[test]
    fn test_trailing_question() {
        assert!(glob_match("file?", "file1"));
        assert!(!glob_match("file?", "file12"));
    }

    #[test]
    fn test_unicode_characters() {
        assert!(glob_match("こんにちは*", "こんにちは世界"));
        assert!(glob_match("こんにち?世界", "こんにちは世界"));
    }
}
