use crate::feed::weather_data::{fetch_weather, WeatherError};
// use crate::internals::render::{insert_html, read_html_file};

// enum WeatherCode {
//     // TODO: Add the weather codes like 3:Mainly clear
// }

pub async fn weather_widget_handler(loc: String) -> Result<String, WeatherError> {
    match fetch_weather(loc.clone()).await {
        Ok(data) => {
            // render_final_template();
            let html_content = format!(
                "<html><body><h1>Weather Data for {}</h1><p>{:#?}</p></body></html>",
                loc, data
            );
            Ok(html_content)
        }
        Err(e) => {
            eprintln!("Error in fetching weather: {}", e);
            Err(e)
        }
    }
}
