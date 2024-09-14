use std::fs;
use crate::model::record_merging::MergeRule;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum MergeRuleReadingError {
    #[error("Error reading file.")]
    FileError,
    #[error("Error parsing json.")]
    JsonParsingError
}

pub fn read_merge_rules(path: &str) -> Result<Vec<MergeRule>, MergeRuleReadingError> {

    let file_contents = fs::read_to_string(path).map_err(|_| MergeRuleReadingError::FileError)?;

    let rules: Vec<MergeRule> = serde_json::from_str(&file_contents).map_err(|_| MergeRuleReadingError::JsonParsingError)?;

    Ok(rules)
}
