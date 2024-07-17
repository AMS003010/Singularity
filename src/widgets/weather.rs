use crate::feed::weather_data::fetch_weather;

pub async fn weather_widget_handler(loc: String) {
    if let Err(e) = fetch_weather(loc).await {
        eprintln!("Error in fetching weather: {}",e);
    }

    
}