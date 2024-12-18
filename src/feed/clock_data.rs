use chrono::{DateTime, Utc};
use std::collections::HashMap;

fn get_timezone_for_place(place: &str) -> Option<chrono::FixedOffset> {
    let mut place_to_timezone = HashMap::new();
    place_to_timezone.insert("New York", chrono::FixedOffset::west_opt(4 * 3600));
    place_to_timezone.insert("London", chrono::FixedOffset::east_opt(0 * 3600));
    place_to_timezone.insert("Tokyo", chrono::FixedOffset::east_opt(9 * 3600));
    place_to_timezone.insert("Delhi", chrono::FixedOffset::east_opt(5 * 3600 + 1800));
    place_to_timezone.insert("Los Angeles", chrono::FixedOffset::west_opt(8 * 3600));
    // println!("---> clock_data.rs // get_timezone_for_place");
    place_to_timezone.get(place).and_then(|opt| *opt)
}

pub fn get_current_time_for_place(place: &str) -> Result<DateTime<chrono::FixedOffset>, String> {
    let now = Utc::now();
    // println!("---> clock_data.rs // get_current_time_for_place");
    match get_timezone_for_place(place) {
        Some(timezone) => Ok(now.with_timezone(&timezone)),
        None => Err(format!("Timezone not found for place {}", place)),
    }
}