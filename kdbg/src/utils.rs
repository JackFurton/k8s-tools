use chrono::{DateTime, Utc};

/// Calculate age from timestamp (e.g., "2m", "5h", "3d")
pub fn calculate_age(timestamp: &str) -> String {
    let parsed = DateTime::parse_from_rfc3339(timestamp);
    if parsed.is_err() {
        return "unknown".to_string();
    }

    let created = parsed.unwrap().with_timezone(&Utc);
    let now = Utc::now();
    let duration = now.signed_duration_since(created);

    let seconds = duration.num_seconds();
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m", seconds / 60)
    } else if seconds < 86400 {
        format!("{}h", seconds / 3600)
    } else {
        format!("{}d", seconds / 86400)
    }
}

/// Get plugin directory path
pub fn get_plugin_dir() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(home).join(".kdbg").join("plugins")
}
