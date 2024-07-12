use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Glance {
    pub pages: Vec<Page>,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Deserialize)]
pub struct Column {
    pub size: String,
    pub widgets: Vec<Widget>,
}

#[derive(Debug, Deserialize)]
pub struct Widget {
    #[serde(rename = "type")]
    pub widget_type: String,
    pub limit: Option<u32>,
    #[serde(rename = "collapse-after")]
    pub collapse_after: Option<u32>,
    pub cache: Option<String>,
    pub feeds: Option<Vec<Feed>>,
    pub channels: Option<Vec<String>>,
    pub subreddit: Option<String>,
    pub location: Option<String>,
    pub stocks: Option<Vec<Stock>>,
}

#[derive(Debug, Deserialize)]
pub struct Feed {
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
}
