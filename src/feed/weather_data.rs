use reqwest::Error;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeatherError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("No geocoding data found")]
    NoGeocodingData,
}

#[derive(Debug, Deserialize)]
struct GeoResult {
    id: u64,
    name: String,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    feature_code: String,
    country_code: String,
    admin1_id: u64,
    admin2_id: u64,
    timezone: String,
    population: u64,
    country_id: u64,
    country: String,
    admin1: String,
    admin2: String,
}

#[derive(Debug, Deserialize)]
struct GeoResponse {
    results: Vec<GeoResult>,
    generationtime_ms: f64,
}

#[derive(Debug, Deserialize)]
struct DailyUnits {
    time: String,
    weather_code: String,
    temperature_2m_max: String,
}

#[derive(Debug, Deserialize)]
struct DailyDataUnit {
    time: Vec<String>,
    weather_code: Vec<u64>,
    temperature_2m_max: Vec<f64>,
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
    daily_units: DailyUnits,
    daily: DailyDataUnit,
}

async fn fetch_geocoding(place: String) -> Result<GeoResponse, WeatherError> {
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}&count=10&language=en&format=json", place);
    let response = reqwest::get(&url).await?.json::<GeoResponse>().await?;
    Ok(response)
}

async fn fetch_weather_forecast(lat: f64, long: f64) -> Result<WeatherForecast, WeatherError> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=weather_code,temperature_2m_max", lat, long);
    let response = reqwest::get(&url).await?.json::<WeatherForecast>().await?;
    Ok(response)
}

pub async fn fetch_weather(loc: String) -> Result<WeatherForecast, WeatherError> {
    match fetch_geocoding(loc).await {
        Ok(response) => {
            if let Some(first_result) = response.results.first() {
                println!("{} {}", first_result.latitude, first_result.longitude);
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
