use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub method: String,
    pub url: String,
    pub ip: String,
    pub date: String,
    pub vulnerability_type: Option<String>,
}

pub fn process_log_file(file_path: &str, patterns: &HashMap<String, String>) -> Vec<LogEntry> {
    let log_content = fs::read_to_string(file_path).unwrap_or_default();
    let re = Regex::new(r#"(?P<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}) - - \[(?P<datetime>[^\]]+)\] "(?P<method>[A-Z]+) (?P<urlpath>[^\s]+) HTTP/1\.1" (?P<httpstatus>\d{3}) \d+"#).unwrap();
    
    let mut results = Vec::new();
    
    for cap in re.captures_iter(&log_content) {
        let ip = &cap["ip"];
        let date = &cap["datetime"];
        let method = &cap["method"];
        let url = &cap["urlpath"];
        let mut vulnerabilities = Vec::new();
        
        for (vuln_type, pattern) in patterns {
            let regex = Regex::new(pattern).unwrap();
            if regex.is_match(url) {
                vulnerabilities.push(vuln_type.clone());
            }
        }

        results.push(LogEntry {
            method: method.to_string(),
            url: url.to_string(),
            ip: ip.to_string(),
            date: date.to_string(),
            vulnerability_type: if vulnerabilities.is_empty() { None } else { Some(vulnerabilities.join(", ")) },
        });
    }
    
    results
}
