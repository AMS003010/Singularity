use serde::Deserialize;
use serde::Deserializer;
use serde_json::{self};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WidgetError {
    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("No geocoding data found")]
    NoGeocodingData,
    #[error("Error in reading HTML file")]
    NoHtmlToString,
    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("No timezone found")]
    NoTimeZone,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub theme: String,
    pub theme_background_color: String,
    pub widget_heading: String,
    pub footer: String,
    pub cache: Option<String>,
    pub pages: Vec<Page>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Page {
    pub name: String,
    #[serde(rename = "header-widget")]
    pub header_widget: Option<bool>,
    pub columns: Vec<Column>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Column {
    pub size: String,
    pub widgets: Vec<Widget>,
}

// Individual Widget Structs
#[derive(Debug, Deserialize, Clone)]
pub struct WeatherConfig {
    pub location: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RssConfig {
    pub feeds: Vec<Feed>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct YoutubeConfig {
    pub channels: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ClockConfig;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CalendarConfig;

// Main Widget Enum
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Widget {
    #[serde(rename = "weather")]
    Weather {
        #[serde(flatten)]
        config: WeatherConfig,
    },
    #[serde(rename = "clock")]
    Clock,
    #[serde(rename = "calendar")]
    Calendar,
    #[serde(rename = "youtube")]
    Youtube {
        #[serde(flatten)]
        config: YoutubeConfig,
    },
    #[serde(rename = "rss")]
    Rss {
        #[serde(flatten)]
        config: RssConfig,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct Feed {
    pub url: String,
    #[serde(rename = "title")]
    pub name: Option<String>,
}

impl Widget {
    pub fn clock_config(&self) -> Option<ClockConfig> {
        match self {
            Widget::Clock => Some(ClockConfig),
            _ => None,
        }
    }

    pub fn calendar_config(&self) -> Option<CalendarConfig> {
        match self {
            Widget::Calendar => Some(CalendarConfig),
            _ => None,
        }
    }
}