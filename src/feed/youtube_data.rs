use hyper::{Body, Client, Uri};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use quick_xml::de::{from_str, DeError};
use crate::internals::singularity::WidgetError;

impl From<DeError> for WidgetError {
    fn from(err: DeError) -> Self {
        WidgetError::XmlParse(err.to_string())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "feed")]
pub struct Feed {
    id: String,
    #[serde(rename = "channelId", default)]
    pub yt_channel_id: Option<String>,
    pub title: String,
    #[serde(rename = "author")]
    pub author: Author,
    published: String,
    #[serde(default)]
    #[serde(rename = "entry")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    id: String,
    pub title: String,
    pub published: String,
    #[serde(rename = "group")]
    pub media_group: Option<MediaGroup>,
}

#[derive(Debug, Deserialize)]
pub struct MediaGroup {
    #[serde(rename = "title")]
    title: Option<String>,
    #[serde(rename = "content")]
    pub content: Option<MediaContent>,
    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<MediaThumbnail>,
    #[serde(rename = "community")]
    community: Option<MediaCommunity>,
}

#[derive(Debug, Deserialize)]
pub struct MediaContent {
    #[serde(rename = "@url")]
    pub url: String,
    #[serde(rename = "@width")]
    width: u32,
    #[serde(rename = "@height")]
    height: u32,
}

#[derive(Debug, Deserialize)]
pub struct MediaThumbnail {
    #[serde(rename = "@url")]
    pub url: String,
    #[serde(rename = "@width")]
    width: u32,
    #[serde(rename = "@height")]
    height: u32,
}

#[derive(Debug, Deserialize)]
struct MediaCommunity {
    #[serde(rename = "starRating")]
    star_rating: Option<StarRating>,
    #[serde(rename = "statistics")]
    statistics: Option<Statistics>,
}

#[derive(Debug, Deserialize)]
struct StarRating {
    #[serde(rename = "@count")]
    count: u64,
}

#[derive(Debug, Deserialize)]
struct Statistics {
    #[serde(rename = "@views")]
    views: u64,
}

pub async fn get_youtube_vids_for_a_channel(channel_id: String) -> Result<Feed, WidgetError> {
    let url = format!(
        "https://www.youtube.com/feeds/videos.xml?channel_id={}",
        channel_id
    );

    let uri: Uri = url.parse().unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let res = client.get(uri).await?;
    let body = hyper::body::to_bytes(res.into_body()).await?;
    let response_text = String::from_utf8(body.to_vec())
        .map_err(|e| WidgetError::Utf8Error(e.to_string()))?;

    // println!("{:#?}", response_text);
    let feed: Feed = from_str(&response_text)?;

    Ok(feed)
}