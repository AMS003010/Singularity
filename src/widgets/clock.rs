use crate::feed::clock_data::{get_now_time, get_current_time_for_place, TimeError};
use crate::internals::render::{insert_html, read_html_file, render_final_template, TempData};
use std::collections::HashMap;

fn extract_offset(timestamp: &str) -> &str {
    &timestamp[timestamp.len() - 6..]
}

fn extract_time(timestamp: &str) -> &str {
    &timestamp[10..16]
}

pub async fn clock_widget_handler() -> Result<String, TimeError> {
    let places = ["London", "Tokyo", "Delhi", "Los Angeles"];
    match read_html_file("src/assets/templates/document.html") {
        Ok(outer_html) => {
            match read_html_file("src/assets/templates/clock.html") {
                Ok(inner_html) => {
                    let mut template_data: HashMap<String, TempData> = HashMap::new();
                    
                    let maintime = get_now_time();
                    template_data.insert("nowtime".to_string(), TempData::Text(maintime));
                    
                    let mut count = 1;
                    for place in places.iter() {
                        match get_current_time_for_place(place) {
                            Ok(time) => {
                                let _place = format!("place{}", count);
                                let _time = format!("time{}", count);
                                let _offset = format!("offset{}",count);
                                count += 1;
                                template_data.insert(_place, TempData::Text(place.to_string()));
                                template_data.insert(_time, TempData::Text(extract_time(&time.to_string()).to_string()));
                                template_data.insert(_offset, TempData::Text(extract_offset(&time.to_string()).to_string()));
                            }
                            Err(e) => {
                                eprintln!("Error in getting time/timezone: {}", e);
                                return Err(TimeError::NoTimeZone);
                            }
                        }
                    }
                    let inner_html = render_final_template(inner_html, template_data);
                    let final_html = insert_html(outer_html, inner_html);
                    Ok(final_html)
                }
                Err(e) => {
                    eprintln!("Error in reading clock HTML file: {}", e);
                    Err(TimeError::NoHtmlToString)
                }
            }
        }
        Err(e) => {
            eprintln!("Error in main HTML file: {}", e);
            Err(TimeError::NoHtmlToString)
        }
    }
}