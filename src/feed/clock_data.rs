use chrono::{DateTime, Utc};
use chrono::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimeError {
    #[error("No timezone found")]
    NoTimeZone,
    #[error("Error in reading HTML file")]
    NoHtmlToString,
}

fn get_timezone_for_place(place: &str) -> Option<chrono::FixedOffset> {
    let mut place_to_timezone = HashMap::new();
    place_to_timezone.insert("New York", chrono::FixedOffset::west_opt(5 * 3600));
    place_to_timezone.insert("London", chrono::FixedOffset::east_opt(0 * 3600));
    place_to_timezone.insert("Tokyo", chrono::FixedOffset::east_opt(9 * 3600));
    place_to_timezone.insert("Delhi", chrono::FixedOffset::east_opt(5 * 3600 + 1800));
    place_to_timezone.insert("Los Angeles", chrono::FixedOffset::west_opt(8 * 3600));

    place_to_timezone.get(place).and_then(|opt| *opt)
}

pub fn get_current_time_for_place(place: &str) -> Result<DateTime<chrono::FixedOffset>, String> {
    let now = Utc::now();
    match get_timezone_for_place(place) {
        Some(timezone) => Ok(now.with_timezone(&timezone)),
        None => Err(format!("Timezone not found for place {}", place)),
    }
}

pub fn get_now_time() -> String {
    let now = Local::now().time();
    let hour = now.hour();
    let minute = now.minute();
    format!("{}:{}",hour,minute)
}