use std::collections::HashSet;

fn has_unique_characters(input: &str) -> bool {
    let mut seen_chars = HashSet::new();

    for c in input.to_lowercase().chars() {
        if seen_chars.contains(&c) {
            return false;
        }

        seen_chars.insert(c);
    }

    true
}

fn main() {
    let test_cases = vec![
        "abcd",
        "abCdefAaf",
        "Aabcd",
        "HelloWorld",
        "Unique",
        "Rust",
    ];

    for test in test_cases {
        println!("\"{}\" состоит из уникальных символов: {}", test, has_unique_characters(test));
    }
}
