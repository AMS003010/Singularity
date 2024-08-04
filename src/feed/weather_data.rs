use reqwest::Error;
use serde::Deserialize;
use thiserror::Error;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Error)]
pub enum WeatherError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("No geocoding data found")]
    NoGeocodingData,
    #[error("Error in reading HTML file")]
    NoHtmlToString,
    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
struct GeoResult {
    id: u64,
    name: String,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    feature_code: String,
    country_code: Option<String>,
    admin1_id: Option<u64>,
    admin2_id: Option<u64>,
    timezone: String,
    population: Option<u64>,
    country_id: Option<u64>,
    country: Option<String>,
    admin1: Option<String>,
    admin2: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeoResponse {
    results: Vec<GeoResult>,
    generationtime_ms: f64,
}

#[derive(Debug, Deserialize)]
struct HourlyUnits {
    time: String,
    temperature_2m: String,
    weather_code: String,
}

#[derive(Debug, Deserialize)]
pub struct HourlyDataUnit {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub weather_code: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherForecast {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: u64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    hourly_units: HourlyUnits,
    pub hourly: HourlyDataUnit,
}

async fn fetch_geocoding(place: String) -> Result<GeoResponse, WeatherError> {
    let start = Instant::now();
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}&count=10&language=en&format=json", place);
    
    // Make the GET request and get the response text
    let response = reqwest::get(&url).await?;
    let response_text = response.text().await?;
    
    // Print the raw response text for debugging
    //println!("Raw response: {}", response_text);

    let mut json_values: Value = serde_json::from_str(&response_text)?;

    if let Some(results) = json_values["results"].as_array_mut() {
        if results.len()>1 {
            results.truncate(1);
        }
    }
    
    // Attempt to deserialize the response text
    let geo_response: GeoResponse = serde_json::from_value(json_values)?;
    let duration = start.elapsed();
    println!("Geocoding API {:?}",duration);
    Ok(geo_response)
}

async fn fetch_weather_forecast(lat: f64, long: f64) -> Result<WeatherForecast, WeatherError> {
    let start = Instant::now();
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,weather_code&forecast_days=1", lat, long);
    let response = reqwest::get(&url).await?.json::<WeatherForecast>().await?;
    //println!("{:?}",response);
    let duration = start.elapsed();
    println!("Forecast API {:?}",duration);
    Ok(response)
}

pub async fn fetch_weather(loc: String) -> Result<WeatherForecast, WeatherError> {
    match fetch_geocoding(loc).await {
        Ok(response) => {
            // println!("DEBUG GOD: {:?}", response.results.first());
            if let Some(first_result) = response.results.first() {
                // println!("{} {}", first_result.latitude, first_result.longitude);
                match fetch_weather_forecast(first_result.latitude, first_result.longitude).await {
                    Ok(data) => Ok(data),
                    Err(e) => {
                        eprintln!("Error fetching weather forecast data: {}", e);
                        Err(e)
                    }
                }
            } else {
                eprintln!("No geocoding data found");
                Err(WeatherError::NoGeocodingData)
            }
        }
        Err(e) => {
            eprintln!("Error fetching geocoding data: {}", e);
            Err(e)
        }
    }
}

pub fn fetch_svg_for_weather_code(code: &i32) -> String {
    let mut weather_svg: HashMap<i32, &str> = HashMap::new();
    weather_svg.insert(0, "clear_day.html");
    weather_svg.insert(1, "clear_day.html");
    weather_svg.insert(2, "cloudy.html");
    weather_svg.insert(3, "overcast.html");
    weather_svg.insert(45, "fog.html");
    weather_svg.insert(48, "fog.html");
    weather_svg.insert(51, "drizzle.html");
    weather_svg.insert(53, "drizzle.html");
    weather_svg.insert(55, "drizzle.html");
    weather_svg.insert(56, "drizzle.html");
    weather_svg.insert(57, "drizzle.html");
    weather_svg.insert(61, "drizzle.html");
    weather_svg.insert(63, "moderate_rain.html");
    weather_svg.insert(65, "heavy_rain.html");
    weather_svg.insert(66, "heavy_rain.html");
    weather_svg.insert(67, "heavy_rain.html");
    weather_svg.insert(71, "snow.html");
    weather_svg.insert(73, "moderate_snow.html");
    weather_svg.insert(75, "heavy_snow.html");
    weather_svg.insert(77, "heavy_snow.html");
    weather_svg.insert(80, "drizzle.html");
    weather_svg.insert(81, "moderate_rain.html");
    weather_svg.insert(82, "heavy_rain.html");
    weather_svg.insert(85, "snow.html");
    weather_svg.insert(86, "snow.html");
    weather_svg.insert(95, "thunder.html");
    weather_svg.insert(96, "thunder.html");
    weather_svg.insert(99, "thunder.html");

    // Check if the weather code exists in the HashMap
    if let Some(&svg_name) = weather_svg.get(code) {
        format!("src/assets/svgs/{}", svg_name)
    } else {
        String::from("src/assets/svgs/thunder.html") // Or any default value or handling
    }
}