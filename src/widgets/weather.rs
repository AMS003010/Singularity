use crate::feed::weather_data::{fetch_weather, fetch_svg_for_weather_code};
use crate::internals::render::{read_html_file, render_final_template, TempData, insert_html_once, insert_html, hydrate_val_once};
use crate::internals::singularity::WidgetError;
use crate::internals::cache::GenericWidgetCache;
use crate::internals::singularity::Widget;
use std::collections::HashMap;
use std::sync::Arc;
use actix_web::web;

fn extract_time(timestamp: &str) -> &str {
    &timestamp[timestamp.len() - 5..]
}

fn final_svg_comp(code: &i32, svg_count: &mut i32) -> Result<String, WidgetError> {
    let path = fetch_svg_for_weather_code(code);
    match read_html_file(&path) {
        Ok(mut html) => {
            if *svg_count == 0 {
                html = hydrate_val_once(html, "svgSize".to_string(), "32".to_string());
            } else {
                html = hydrate_val_once(html, "svgSize".to_string(), "12".to_string());
            }
            *svg_count += 1;
            Ok(html)
        }
        Err(_) => {
            eprintln!("Error in fetching HTML file");
            Err(WidgetError::NoHtmlToString)
        }
    }
}

pub async fn weather_widget_handler(
    loc: String,
    _widget_theme: String,
    _widget_cache: web::Data<Arc<GenericWidgetCache>>,
    _widget: Widget,
) -> Result<String, WidgetError> {
    // println!("---> weather.rs // weather_widget_handler");

    // let location = if let Widget::Weather { config } = _widget {
    //     config.location
    // } else {
    //     return Err(WidgetError::NoGeocodingData);
    // };

    // println!("loca: {}",location);

    const WIDGET_NAME: &str = "weather_widget";

    match _widget_cache.get(WIDGET_NAME).await {
        Ok(Some(cached_html)) => {
            // Cache HIT
            return Ok(cached_html);
        }
        Ok(None) => {
            // Cache MISS
        }
        Err(e) => {
            eprintln!("Cache retrieval error: {}", e);
        }
    }

    let mut weather_code: HashMap<i32, &str> = HashMap::new();
    weather_code.insert(0, "Clear Sky");
    weather_code.insert(1, "Mainly Clear");
    weather_code.insert(2, "Partly Cloudy");
    weather_code.insert(3, "Overcast");
    weather_code.insert(45, "Fog");
    weather_code.insert(48, "Rime Fog");
    weather_code.insert(51, "Drizzle");
    weather_code.insert(53, "Drizzle");
    weather_code.insert(55, "Drizzle");
    weather_code.insert(56, "Drizzle");
    weather_code.insert(57, "Drizzle");
    weather_code.insert(61, "Rain");
    weather_code.insert(63, "Moderate Rain");
    weather_code.insert(65, "Heavy Rain");
    weather_code.insert(66, "Freezing Rain");
    weather_code.insert(67, "Freezing Rain");
    weather_code.insert(71, "Snow");
    weather_code.insert(73, "Moderate Snow");
    weather_code.insert(75, "Heavy Snow");
    weather_code.insert(77, "Snow Grains");
    weather_code.insert(80, "Rain");
    weather_code.insert(81, "Moderate Rain");
    weather_code.insert(82, "Heavy Rain");
    weather_code.insert(85, "Snow");
    weather_code.insert(86, "Snow");
    weather_code.insert(95, "Thunderstorm");
    weather_code.insert(96, "Thunderstorm");
    weather_code.insert(99, "Thunderstorm");

    match fetch_weather("Bengaluru".to_string()).await {
        Ok(data) => {
            match read_html_file("src/assets/templates/weather.html") {
                Ok(inner_html) => {
                    let mut template_data: HashMap<String, TempData> = HashMap::new();

                    template_data.insert("widget_theme".to_string(), TempData::Text(loc.to_string()));
                    template_data.insert("widgetHeading".to_string(), TempData::Text(_widget_theme.to_string()));
                    template_data.insert("place".to_string(), TempData::Text("Bengaluru".to_string()));

                    let present_weather_code = data.hourly.weather_code[0] as i32;
                    let weather_codes = [
                        data.hourly.weather_code[4] as i32,
                        data.hourly.weather_code[4] as i32,
                        data.hourly.weather_code[8] as i32,
                        data.hourly.weather_code[12] as i32,
                        data.hourly.weather_code[16] as i32,
                        data.hourly.weather_code[20] as i32,
                        data.hourly.weather_code[23] as i32,
                    ];

                    let mut temp_inner_html = inner_html;
                    let mut svg_count = 0;
                    for code in &weather_codes {
                        if let Ok(svg) = final_svg_comp(code, &mut svg_count) {
                            temp_inner_html = insert_html_once(temp_inner_html, svg);
                        } else {
                            eprintln!("Error in generating SVG for code: {}", code);
                        }
                    }

                    let present_weather = weather_code.get(&present_weather_code).unwrap_or(&"---");

                    let times = [
                        extract_time(&data.hourly.time[4]),
                        extract_time(&data.hourly.time[8]),
                        extract_time(&data.hourly.time[12]),
                        extract_time(&data.hourly.time[16]),
                        extract_time(&data.hourly.time[20]),
                        extract_time(&data.hourly.time[23]),
                    ];

                    for (i, time) in times.iter().enumerate() {
                        let time_key = format!("time{}", i + 1);
                        let temp_key = format!("temp{}", i + 1);
                        template_data.insert(time_key.clone(), TempData::Text(time.to_string()));
                        template_data.insert(temp_key.clone(), TempData::Number(data.hourly.temperature_2m[i * 4] as i32));
                    }

                    template_data.insert("presentTemp".to_string(), TempData::Number(data.hourly.temperature_2m[0] as i32));
                    template_data.insert("presentWeather".to_string(), TempData::Text(present_weather.to_string()));

                    let inner_html = render_final_template(temp_inner_html, template_data);
                    match _widget_cache.insert(WIDGET_NAME.to_string(), inner_html.clone()).await {
                        Ok(_) => {
                            // Inserted to Cache
                        }
                        Err(e) => {
                            eprintln!("Failed to insert widget into cache: {}", e);
                        }
                    }
                    Ok(inner_html)
                }
                Err(e) => {
                    eprintln!("Error in reading weather HTML file: {}", e);
                    Err(WidgetError::NoHtmlToString)
                }
            }
        }
        Err(e) => {

            /*---------------------------------------------------------------------------------------------
            *  Contains the Fallback HTML in case of network error ⚠️
            *  No cache needed here, cause it might return fallback HTML even when network is stable 🛜🌐
            *--------------------------------------------------------------------------------------------*/

            eprintln!("Error in fetching weather: {}", e);
            
            match read_html_file("src/assets/templates/weather.html") {
                Ok(fallback_html) => {
                    let mut template_data: HashMap<String, TempData> = HashMap::new();
                    let mut temp_inner_html = fallback_html;
                    
                    template_data.insert("widget_theme".to_string(), TempData::Text(loc.to_string()));
                    template_data.insert("widgetHeading".to_string(), TempData::Text(_widget_theme.to_string()));
                    template_data.insert("place".to_string(), TempData::Text("---".to_string()));
                    template_data.insert("presentWeather".to_string(), TempData::Text("Weather Unavailable ⚠️<br/><br/>".to_string()));
                    template_data.insert("presentTemp".to_string(), TempData::Text("<br/>503 Network Error🔌".to_string()));

                    temp_inner_html = insert_html(temp_inner_html, "&nbsp;".to_string());
                    temp_inner_html = insert_html(temp_inner_html, "---".to_string());
                    temp_inner_html = insert_html(temp_inner_html, "---".to_string());
                    temp_inner_html = insert_html(temp_inner_html, "---".to_string());
                    temp_inner_html = insert_html(temp_inner_html, "---".to_string());
                    temp_inner_html = insert_html(temp_inner_html, "---".to_string());
                    temp_inner_html = insert_html(temp_inner_html, "---".to_string());

                    let times = ["--:--"; 6];
                    let temps = [0; 6];

                    for (i, time) in times.iter().enumerate() {
                        let time_key = format!("time{}", i + 1);
                        let temp_key = format!("temp{}", i + 1);
                        template_data.insert(time_key, TempData::Text(time.to_string()));
                        template_data.insert(temp_key, TempData::Number(temps[i]));
                    }

                    let temp_inner_html = render_final_template(temp_inner_html, template_data);
                    Ok(temp_inner_html)
                }
                Err(_) => {
                    eprintln!("Failed to read fallback weather HTML template");
                    Err(WidgetError::NoHtmlToString)
                }
            }
        }
    }
}