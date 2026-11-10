use regex::Regex;
use std::env;

pub fn env_or(key: &str, default: &str) -> String {
    env::var(key)
        .unwrap_or_else(|_| default.to_string())
        .to_string()
}

#[derive(Debug, PartialEq)]
pub enum CaseType {
    Pascal,
    Camel,
    Snake,
    Kebab,
    SingleWord,
    Unknown,
}

pub fn deconstruct_case(input: &str) -> (CaseType, Vec<String>) {
    if input.is_empty() {
        return (CaseType::Unknown, vec![]);
    }

    let snake_re = Regex::new(r"^[a-z0-9]+(_[a-z0-9]+)+$").unwrap();
    let kebab_re = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)+$").unwrap();

    // detect and split delimiters immediately
    if snake_re.is_match(input) {
        return (
            CaseType::Snake,
            input.split('_').map(|s| s.to_string()).collect(),
        );
    }
    if kebab_re.is_match(input) {
        return (
            CaseType::Kebab,
            input.split('-').map(|s| s.to_string()).collect(),
        );
    }

    // linear scan for camel and pascal casing
    let mut words = Vec::new();
    let mut current_word = String::new();
    let chars: Vec<char> = input.chars().collect();

    let first_is_uppercase = chars.first().is_some_and(|c| c.is_uppercase());
    let mut has_case_changes = false;

    for (i, &c) in chars.iter().enumerate() {
        // if character is uppercase and its not the very start of the string, split boundary
        if c.is_uppercase() && i > 0 {
            has_case_changes = true;
            words.push(current_word.to_lowercase());
            current_word = String::new();
        }
        current_word.push(c);
    }
    words.push(current_word.to_lowercase());

    // determine specific type
    let case_type = if has_case_changes {
        if first_is_uppercase {
            CaseType::Pascal
        } else {
            CaseType::Camel
        }
    } else if chars.iter().all(|c| c.is_alphanumeric()) {
        CaseType::SingleWord
    } else {
        CaseType::Unknown
    };

    (case_type, words)
}
