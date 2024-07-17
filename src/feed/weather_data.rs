use reqwest::Error;
use serde::Deserialize;

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
struct HourlyUnits {
    time: String,
    temperature_2m: String,
    weather_code: String,
}

#[derive(Debug, Deserialize)]
struct HourlyDataUnit {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    weather_code: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct WeatherForecast {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: u64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    hourly_units: HourlyUnits,
    hourly: HourlyDataUnit,
}

async fn fetch_geocoding(place: String) -> Result<GeoResponse, Error> {
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}&count=10&language=en&format=json",place);
    let response = reqwest::get(url).await?.json::<GeoResponse>().await?;
    Ok(response)
}

async fn fetch_weather_forecast(lat: f64, long:f64) -> Result<WeatherForecast, Error> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,weather_code",lat,long);
    let response = reqwest::get(url).await?.json::<WeatherForecast>().await?;
    Ok(response)
}

pub async fn fetch_weather(loc: String) -> Result<(), Error> {
    match fetch_geocoding(loc).await {
        Ok(response) => {
            if let Some(first_result) = response.results.first() {
                println!("{} {}",first_result.latitude,first_result.longitude);

                // TODO: Look after the case of more than 1 search result from the GeoCoding Response

                match fetch_weather_forecast(response.results[0].latitude, response.results[0].longitude).await {
                    Ok(data) => {
                        println!("{:?}",data);
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("Error fetching weather forecast data: {}",e);
                        Err(e)
                    }
                }
            } else {
                eprintln!("No geocoding data found");
                Ok(())
            }
        }
        Err(e) => {
            eprintln!("Error fetching geocoding data: {}",e);
            Err(e)
        }
    }
} 