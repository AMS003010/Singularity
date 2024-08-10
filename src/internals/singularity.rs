use serde::Deserialize;

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
    pub widget_type: String,
    pub location: Option<String>,
    pub feeds: Option<Vec<Feed>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Feed {
    pub url: String,
    pub name: Option<String>,
}