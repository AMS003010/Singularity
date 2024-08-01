use crate::feed::weather_data::{fetch_weather, WeatherError};
use crate::internals::render::{insert_html, read_html_file, render_final_template, TempData};
use std::collections::HashMap;

pub async fn weather_widget_handler(loc: String) -> Result<String, WeatherError> {
    let mut weather_code: HashMap<i32, &str> = HashMap::new();
    weather_code.insert(0,"Clear Sky");
    weather_code.insert(1,"Mainly Clear");
    weather_code.insert(2,"Partly Cloudy");
    weather_code.insert(3,"Overcast");
    weather_code.insert(45,"Fog");
    weather_code.insert(48,"Rime Fog");
    weather_code.insert(51,"Drizzle");
    weather_code.insert(53,"Drizzle");
    weather_code.insert(55,"Drizzle");
    weather_code.insert(56,"Drizzle");
    weather_code.insert(57,"Drizzle");
    weather_code.insert(61,"Rain");
    weather_code.insert(63,"Moderate Rain");
    weather_code.insert(65,"Heavy Rain");
    weather_code.insert(66,"Freezing Rain");
    weather_code.insert(67,"Freezing Rain");
    weather_code.insert(71,"Snow");
    weather_code.insert(73,"Moderate Snow");
    weather_code.insert(75,"Heavy Snow");
    weather_code.insert(77,"Snow Grains");
    weather_code.insert(80,"Rain");
    weather_code.insert(81,"Moderate Rain");
    weather_code.insert(82,"Heavy Rain");
    weather_code.insert(85,"Snow");
    weather_code.insert(86,"Snow");
    weather_code.insert(95,"Thunderstorm");
    weather_code.insert(96,"Thunderstorm");
    weather_code.insert(99,"Thunderstorm");

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
                            template_data.insert("time2", TempData::Number(data.hourly.temperature_2m[4] as i32));
                            template_data.insert("temp2", TempData::Number(data.hourly.temperature_2m[4] as i32));
                            template_data.insert("time3", TempData::Number(data.hourly.temperature_2m[8] as i32));
                            template_data.insert("temp3", TempData::Number(data.hourly.temperature_2m[8] as i32));
                            template_data.insert("time4", TempData::Number(data.hourly.temperature_2m[12] as i32));
                            template_data.insert("temp4", TempData::Number(data.hourly.temperature_2m[12] as i32));
                            template_data.insert("time5", TempData::Number(data.hourly.temperature_2m[16] as i32));
                            template_data.insert("temp5", TempData::Number(data.hourly.temperature_2m[16] as i32));
                            template_data.insert("time6", TempData::Number(data.hourly.temperature_2m[20] as i32));
                            template_data.insert("temp6", TempData::Number(data.hourly.temperature_2m[20] as i32));

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
