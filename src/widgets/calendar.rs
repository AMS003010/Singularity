use crate::feed::calendar_data::{get_current_date, get_month, get_day_from_date};
use crate::internals::render::{read_html_file, render_final_template, TempData};
use crate::internals::singularity::WidgetError;
use crate::internals::cache::GenericWidgetCache;
use std::collections::HashMap;
use std::sync::Arc;
use actix_web::web;

// TODO: Add a feature to be able to highlight ceratin dates for events

pub async fn calendar_widget_handler(
    theme: String,
    _widget_theme: String,
    _widget_cache: web::Data<Arc<GenericWidgetCache>>
) -> Result<String, WidgetError> {
    // println!("---> calendar.rs // calendar_widget_handler");
    const WIDGET_NAME: &str = "calendar_widget";

    match _widget_cache.get(WIDGET_NAME).await {
        Ok(Some(cached_html)) => {
            println!("Cache hit for widget: {}", WIDGET_NAME);
            return Ok(cached_html);
        }
        Ok(None) => {
            println!("Cache miss for widget: {}", WIDGET_NAME);
        }
        Err(e) => {
            eprintln!("Cache retrieval error: {}", e);
        }
    }

    match read_html_file("src/assets/templates/calendar.html") {
        Ok(wid_html) => {
            let mut template_data: HashMap<String, TempData> = HashMap::new();

            // Injecting theme
            template_data.insert("widget_theme".to_string(),TempData::Text(theme.to_string()));
            template_data.insert("widgetHeading".to_string(),TempData::Text(_widget_theme.to_string()));

            let day_name_list = ["Sun","Mon", "Tue","Wed", "Thu", "Fri", "Sat"];

            let (year, month, day, _weekday) = get_current_date();
            let month_name = get_month(&month);

            template_data.insert("dateToday".to_string(),TempData::Text(day.to_string()));
            template_data.insert("monthToday".to_string(),TempData::Text(month_name.clone()));
            template_data.insert("yearToday".to_string(),TempData::Text(year.to_string()));

            if &month_name=="January" || &month_name=="March" || &month_name=="May" || &month_name=="July" || &month_name=="August" || &month_name=="October" || &month_name=="December" {
                template_data.insert("day29".to_string(),TempData::Number(29));
                template_data.insert("day30".to_string(),TempData::Number(30));
                template_data.insert("day31".to_string(),TempData::Number(31));
            } else if &month_name=="February" {
                template_data.insert("day29".to_string(),TempData::Text("  ".to_string()));
                template_data.insert("day30".to_string(),TempData::Text("  ".to_string()));
                template_data.insert("day31".to_string(),TempData::Text("  ".to_string()));
            }
            else {
                template_data.insert("day29".to_string(),TempData::Number(29));
                template_data.insert("day30".to_string(),TempData::Number(30));
                template_data.insert("day31".to_string(),TempData::Text("  ".to_string()));
            }

            // println!("{} {} {} {} {}",&year, &month, &day, &weekday, &_no_of_days);            
            // println!("{}",get_day_from_date(day, month, year));

            let day_name = get_day_from_date(1, month, year);

            let index = day_name_list.iter().position(|&x| x == day_name);
            match index {
                Some(i) => {
                    for j in 0..7 {
                        let day_number = format!("day_name{}",j);
                        template_data.insert(day_number, TempData::Text(day_name_list[(j+i)%7].to_string()));
                    }
                }
                None => {println!("No index found");}
            }
            let wid_html = render_final_template(wid_html, template_data);
            match _widget_cache.insert(WIDGET_NAME.to_string(), wid_html.clone()).await {
                Ok(_) => {
                    println!("Widget '{}' added to cache", WIDGET_NAME);
                }
                Err(e) => {
                    eprintln!("Failed to insert widget into cache: {}", e);
                }
            }
            Ok(wid_html)
        }
        Err(e) => {
            eprintln!("Error in reading widget HTML file: {}", e);
            Err(WidgetError::NoHtmlToString)
        }
    }
}