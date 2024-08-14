use serde::Deserialize;
use serde::Deserializer;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub pages: Vec<Page>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Page {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Column {
    pub size: String,
    pub widgets: Vec<Widget>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Widget {
    #[serde(rename = "type")]
    pub widget_type: WidgetType,
    pub location: Option<String>,
    pub feeds: Option<Vec<Feed>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Feed {
    pub url: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum WidgetType {
    Weather(String),
    Calendar(String),
    Rss(String),
}

// Custom deserialization for WidgetType
impl<'de> Deserialize<'de> for WidgetType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("weather") {
            Ok(WidgetType::Weather(s))
        } else if s.starts_with("calendar") {
            Ok(WidgetType::Calendar(s))
        } else if s.starts_with("rss") {
            Ok(WidgetType::Rss(s))
        } else {
            Err(serde::de::Error::custom(format!("Invalid WeatherType: {}", s)))
        }
    }
}

impl fmt::Display for WidgetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetType::Weather(data) => write!(f, "Weather: {}", data),
            WidgetType::Calendar(data) => write!(f, "Calendar: {}", data),
            WidgetType::Rss(data) => write!(f, "RSS: {}", data),
        }
    }
}