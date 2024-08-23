use rocket::serde::json::Json;
use serde::Deserialize;
use std::collections::HashMap;
use crate::services::log_service::process_log_file;

#[derive(Debug, Deserialize)]
pub struct LogRequest {
    pub file_path: String,
}

#[post("/analyze", format = "json", data = "<log_request>")]
pub fn analyze_log(log_request: Json<LogRequest>) -> Json<Vec<crate::services::log_service::LogEntry>> {
    // Load patterns from pattern.json
    let patterns: HashMap<String, String> = serde_json::from_str(include_str!("../pattern.json")).unwrap();
    
    let results = process_log_file(&log_request.file_path, &patterns);
    Json(results)
}

