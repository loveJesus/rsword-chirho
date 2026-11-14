// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Tests for vpl2mod_chirho's referenced-line parser.

use super::*;

fn split_chirho(key_chirho: &str, text_chirho: &str) -> Option<(String, String)> {
    Some((key_chirho.to_string(), text_chirho.to_string()))
}

#[test]
fn test_reference_ends_after_the_verse_number_chirho() {
    assert_eq!(parse_referenced_line_chirho("Genesis 1:1 In the beginning God created."), split_chirho("Genesis 1:1", "In the beginning God created."));
    assert_eq!(parse_referenced_line_chirho("Gen 1:1 In the beginning"), split_chirho("Gen 1:1", "In the beginning"));
    assert_eq!(parse_referenced_line_chirho("1 John 3:16 Hereby perceive we the love of God"), split_chirho("1 John 3:16", "Hereby perceive we the love of God"));
    assert_eq!(parse_referenced_line_chirho("Psalms 119:105 Thy word is a lamp: and a light"), split_chirho("Psalms 119:105", "Thy word is a lamp: and a light"));
}

#[test]
fn test_tab_separated_bare_and_unreferenced_lines_chirho() {
    assert_eq!(parse_referenced_line_chirho("Gen.1.1\tIn the beginning"), split_chirho("Gen.1.1", "In the beginning"));
    assert_eq!(parse_referenced_line_chirho("John 11:35"), split_chirho("John 11:35", ""));
    assert_eq!(parse_referenced_line_chirho("In the beginning God created"), None);
}

#[test]
fn test_parsed_references_resolve_to_kjv_verses_chirho() {
    for line_chirho in ["Genesis 1:1 In the beginning", "John 3:16 For God so loved the world", "1 John 3:16 Hereby"] {
        let (key_chirho, _) = parse_referenced_line_chirho(line_chirho).expect("a reference");
        assert!(parse_verse_key_chirho(&key_chirho, kjv_chirho()).is_some(), "{key_chirho} did not resolve");
    }
}
