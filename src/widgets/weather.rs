use crate::feed::weather_data::{fetch_weather, WeatherError};
use crate::internals::render::{insert_html, read_html_file};

// enum WeatherCode {
//     // TODO: Add the weather codes like 3:Mainly clear
// }

pub async fn weather_widget_handler(loc: String) -> Result<String, WeatherError> {
    match fetch_weather(loc.clone()).await {
        Ok(data) => {
            match read_html_file("src/assets/templates/document.html") {
                Ok(outer_html) => {
                    let inner_html = format!(
                        "<div><h1>Weather Data for {}</h1><p>{:?}</p></div>",
                        loc, data
                    );
                    let final_html = insert_html(outer_html, inner_html);
                    println!("{}", final_html);
                    Ok(final_html)
                }
                Err(e) => {
                    eprintln!("Error in reading HTML file: {}", e);
                    Err(WeatherError::NoHtmlToString)
                }
            }
        }
        Err(e) => {
            eprintln!("Error in fetching weather: {}", e);
            Err(e)
        }
    }
}