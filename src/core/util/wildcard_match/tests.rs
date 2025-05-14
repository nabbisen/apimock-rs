use crate::core::util::wildcard_match::wildcard_match;

#[test]
fn tests() {
    let test_cases = vec![
        ("a*b?d*", "abbd", true),
        ("a*b?d*", "aabcd", true),
        ("a*b?d*", "acbd", true),
        ("a*b?d*", "abbbbbd", true),
        ("abc", "abc", true),
        ("abc", "abd", false),
        ("a*cd", "abcd", true),
        ("a?c", "abc", true),
        ("a*c?d", "acbd", true),
        ("a?c", "ab", false),
        ("a*d", "abbd", false),
        ("", "", true),
        ("abc", "", false),
        ("a*cd", "abcd", true),
        ("a*b", "abbb", true),
        ("a*cd", "acbd", true),
    ];

    for (pattern, text, expected) in test_cases {
        let result = wildcard_match(pattern, text);
        println!(
            "Pattern: '{}', Text: '{}', Expected: {}, Got: {}",
            pattern, text, expected, result
        );
        assert_eq!(result, expected);
    }
}
