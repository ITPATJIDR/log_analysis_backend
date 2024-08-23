use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LogAnalysisResult {
    pub log_path: String,
    pub results: Vec<LogEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub method: String,
    pub url: String,
    pub ip: String,
    pub type_: String,
    pub date: String,
}

#[derive(Deserialize)]
pub struct Patterns {
    pub patterns: HashMap<String, String>,
}