use crate::feed::weather_data::{fetch_weather, WeatherError};
use crate::internals::render::{insert_html, read_html_file, render_final_template, TempData};
use std::collections::HashMap;

// enum WeatherCode {
//     // TODO: Add the weather codes like 3:Mainly clear
// }

pub async fn weather_widget_handler(loc: String) -> Result<String, WeatherError> {
    match fetch_weather(loc.clone()).await {

        // TODO: Error handling if api call goes wrong to return a fallback html

        Ok(data) => {
            match read_html_file("src/assets/templates/document.html") {
                Ok(outer_html) => {
                    match read_html_file("src/assets/templates/weather.html") {
                        Ok(inner_html) => {
                            let mut template_data = HashMap::new();
                            template_data.insert("place", TempData::Text(loc));
                            template_data.insert("presentTemp", TempData::Number(data.hourly.temperature_2m[0] as i32));
                            template_data.insert("presentWeather", TempData::Number(data.hourly.temperature_2m[0] as i32));
                            template_data.insert("time1", TempData::Number(data.hourly.temperature_2m[0] as i32));
                            template_data.insert("temp1", TempData::Number(data.hourly.temperature_2m[0] as i32));
                            template_data.insert("time2", TempData::Number(data.hourly.temperature_2m[1] as i32));
                            template_data.insert("temp2", TempData::Number(data.hourly.temperature_2m[1] as i32));
                            template_data.insert("time3", TempData::Number(data.hourly.temperature_2m[2] as i32));
                            template_data.insert("temp3", TempData::Number(data.hourly.temperature_2m[2] as i32));
                            template_data.insert("time4", TempData::Number(data.hourly.temperature_2m[3] as i32));
                            template_data.insert("temp4", TempData::Number(data.hourly.temperature_2m[3] as i32));
                            template_data.insert("time5", TempData::Number(data.hourly.temperature_2m[4] as i32));
                            template_data.insert("temp5", TempData::Number(data.hourly.temperature_2m[4] as i32));
                            template_data.insert("time6", TempData::Number(data.hourly.temperature_2m[5] as i32));
                            template_data.insert("temp6", TempData::Number(data.hourly.temperature_2m[5] as i32));

                            let inner_html = render_final_template(inner_html, template_data);
                            let final_html = insert_html(outer_html, inner_html);
                            // println!("{}", final_html);
                            Ok(final_html)
                        }
                        Err(e) => {
                            eprintln!("Error in reading weather HTML file: {}", e);
                            Err(WeatherError::NoHtmlToString)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error in reading main HTML file: {}", e);
                    Err(WeatherError::NoHtmlToString)
                }
            }
        }
        Err(e) => {
            eprintln!("Error in fetching weather: {}", e);
            Err(WeatherError::NoHtmlToString)
        }
    }
}
