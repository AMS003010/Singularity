use crate::feed::clock_data::{get_current_time_for_place};
use crate::internals::render::{read_html_file, render_final_template, TempData};
use crate::internals::singularity::WidgetError;
use crate::internals::cache::GenericWidgetCache;
use crate::internals::singularity::Widget;
use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, FixedOffset};
use actix_web::web;

fn extract_offset(time: &DateTime<FixedOffset>) -> String {
    let mut formatted = time.format("%z").to_string();
    formatted.insert(3,':');
    formatted
}

fn extract_time(time: &DateTime<FixedOffset>) -> String {
    time.format("%H:%M").to_string()
}

pub async fn clock_widget_handler(
    theme: String,
    _widget_theme: String,
    _widget_cache: web::Data<Arc<GenericWidgetCache>>,
    _widget: Widget,
) -> Result<String, WidgetError> {
    // const WIDGET_NAME: &str = "clock_widget";

    // match _widget_cache.get(WIDGET_NAME).await {
    //     Ok(Some(cached_html)) => {
    //         // Cache HIT
    //         return Ok(cached_html);
    //     }
    //     Ok(None) => {
    //         // Cache MISS
    //     }
    //     Err(e) => {
    //         eprintln!("Cache retrieval error: {}", e);
    //     }
    // }

    let places = ["London", "Tokyo", "Delhi", "Los Angeles"];
    match read_html_file("src/assets/templates/clock.html") {
        Ok(inner_html) => {
            let mut template_data: HashMap<String, TempData> = HashMap::new();

            // Inject theme and widget heading
            template_data.insert("widget_theme".to_string(), TempData::Text(theme));
            template_data.insert("widgetHeading".to_string(), TempData::Text(_widget_theme));

            let mut count = 1;
            for place in places.iter() {
                match get_current_time_for_place(place) {
                    Ok(time) => {
                        let _place = format!("place{}", count);
                        let _time = format!("time{}", count);
                        let _offset = format!("offset{}", count);
                        count += 1;

                        template_data.insert(_place, TempData::Text(place.to_string()));
                        template_data.insert(_time, TempData::Text(extract_time(&time)));
                        template_data.insert(_offset, TempData::Text(extract_offset(&time)));
                    }
                    Err(e) => {
                        eprintln!("Error in getting time for place '{}': {}", place, e);
                        return Err(WidgetError::NoTimeZone);
                    }
                }
            }

            // Render final HTML
            let rendered_html = render_final_template(inner_html, template_data);

            // match _widget_cache.insert(WIDGET_NAME.to_string(), rendered_html.clone()).await {
            //     Ok(_) => {
            //         // Inserted to Cache
            //     }
            //     Err(e) => {
            //         eprintln!("Failed to insert widget into cache: {}", e);
            //     }
            // }

            Ok(rendered_html)
        }
        Err(e) => {
            eprintln!("Error in reading clock HTML file: {}", e);
            Err(WidgetError::NoHtmlToString)
        }
    }
}
