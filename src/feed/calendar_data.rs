use chrono::prelude::*;
use chrono::{NaiveDate, Weekday};
use std::collections::HashMap;

pub fn get_current_date() -> (i32,u32,u32,String) {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = local.month();
    let day = local.day();
    let weekday = local.format("%A").to_string();
    // println!("---> calendar_data.rs // get_current_date");
    (year, month, day, weekday)
}

pub fn get_day_from_date(day: u32, month: u32, year: i32) -> String {
    if let Some(date) = NaiveDate::from_ymd_opt(year,month,day) {
        let weekday: Weekday = date.weekday();
        // println!("---> calendar_data.rs // get_day_from_date");
        weekday.to_string()
    } else {
        "Invalid date".to_string()
    }
}

pub fn get_month(code: &u32) -> String {
    let mut month_map: HashMap<u32, &str> = HashMap::new();
    month_map.insert(1,"January");
    month_map.insert(2,"February");
    month_map.insert(3,"March");
    month_map.insert(4,"April");
    month_map.insert(5,"May");
    month_map.insert(6,"June");
    month_map.insert(7,"July");
    month_map.insert(8,"August");
    month_map.insert(9,"September");
    month_map.insert(10,"October");
    month_map.insert(11,"November");
    month_map.insert(12,"December");
    // println!("---> calendar_data.rs // get_month");
    month_map.get(code).unwrap_or(&"Unknown month").to_string()
}