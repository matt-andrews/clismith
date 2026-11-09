use noyalib::compat::serde_yaml;
use regex::Regex;
use serde::Serialize;
use std::{
    env,
    fs::File,
    io::{BufWriter, Error},
    path::Path,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File reading error occured")]
    IoError(std::io::Error),
    #[error("Json serialization error occured")]
    JsonError(serde_json::Error),
}

pub fn get_file(dir: &Path, file: &str) -> Result<String, Error> {
    let src = std::fs::read_to_string(dir.join(file));
    if let Err(e) = src {
        eprintln!(
            "failed to get file: {}",
            format_args!(
                "error: can't read {}/{}: {}",
                dir.display(),
                file,
                e.to_string()
            )
        );
        return Err(e);
    }

    Ok(src.unwrap())
}

pub fn deserialize_yaml_from_path<T: for<'de> serde_core::de::Deserialize<'de> + 'static>(
    dir: &Path,
    file: &str,
) -> std::result::Result<T, noyalib::compat::serde_yaml::Error> {
    let src = get_file(dir, file);

    if src.is_err() {
        return Err(noyalib::compat::serde_yaml::Error::from(
            noyalib::Error::Deserialize("failed to deserialize".to_string()),
        ));
    }

    serde_yaml::from_str(&src.unwrap())
}

pub fn deserialize_json_from_path<T: for<'de> serde_core::de::Deserialize<'de> + 'static>(
    dir: &Path,
    file: &str,
) -> std::result::Result<T, AppError> {
    let src = get_file(dir, file).map_err(AppError::IoError)?;
    let result = serde_json::from_str(&src).map_err(AppError::JsonError)?;
    Ok(result)
}

pub fn serialize_to_json<T>(obj: &T, output: &Path) -> anyhow::Result<()>
where
    T: Serialize,
{
    let file = File::create(output)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &obj)?;
    Ok(())
}

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
