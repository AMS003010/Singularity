use crate::feed::youtube_data::get_youtube_vids_for_a_channel;
use crate::internals::render::{read_html_file, render_final_template, TempData, insert_html};
use crate::internals::singularity::WidgetError;
use crate::internals::cache::GenericWidgetCache;
use crate::internals::singularity::Widget;
use std::collections::HashMap;
use std::sync::Arc;
use actix_web::web;
use chrono::{DateTime, Utc};

pub fn time_ago(published: &str) -> String {
    let published_date = DateTime::parse_from_rfc3339(published)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());
    
    let now = Utc::now();
    let duration = now.signed_duration_since(published_date);
    
    let seconds = duration.num_seconds().abs();
    let minutes = duration.num_minutes().abs();
    let hours = duration.num_hours().abs();
    let days = duration.num_days().abs();
    let months = (days as f64 / 30.0).abs().floor() as i64;
    
    if seconds < 60 {
        format!("{}s", seconds)
    } else if minutes < 60 {
        format!("{}min", minutes)
    } else if hours < 24 {
        format!("{}h", hours)
    } else if days < 30 {
        format!("{}d", days)
    } else {
        format!("{}mo", months)
    }
}

pub async fn youtube_widget_handler(
    theme: String,
    _widget_theme: String,
    _widget_cache: web::Data<Arc<GenericWidgetCache>>,
    widget: Widget,
) -> Result<String, WidgetError> {

    const WIDGET_NAME: &str = "yt_widget";

    match _widget_cache.get(WIDGET_NAME).await {
        Ok(Some(cached_html)) => {
            // Cache HIT
            return Ok(cached_html);
        }
        Ok(None) => {
            // Cache MISS
        }
        Err(e) => {
            eprintln!("Cache retrieval error: {}", e);
        }
    }

    let channel_ids = if let Widget::Youtube { config } = widget {
        config.channels
    } else {
        return Err(WidgetError::NoHtmlToString);
    };

    if channel_ids.is_empty() {
        return Err(WidgetError::NoHtmlToString);
    }

    let count = std::cmp::max(15 / channel_ids.len(), 2);

    let video_html = match read_html_file("src/assets/templates/youtube-vid.html") {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error reading video HTML file: {}", e);
            return Err(WidgetError::NoHtmlToString);
        }
    };

    let mut full_entries = String::new();

    for channel_id in channel_ids.iter() {
        let data = match get_youtube_vids_for_a_channel(channel_id.to_string()).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error fetching YouTube videos for channel {}: {}", channel_id, e);
                continue;
            }
        };

        for entry in data.entries.iter().take(count) {
            let mut template_data: HashMap<String, TempData> = HashMap::new();

            let ago = time_ago(&entry.published);

            template_data.insert("yt_channel_link".to_string(),TempData::Text(data.author.uri.clone()));
            template_data.insert("yt_channel_title".to_string(),TempData::Text(data.title.clone()));
            template_data.insert("yt_video_post_duration".to_string(),TempData::Text(ago.to_string()));

            if let Some(media_group) = &entry.media_group {
                if let (Some(thumbnail), Some(content)) = (&media_group.thumbnail, &media_group.content) {
                    template_data.insert("yt_thumbnail_link".to_string(),TempData::Text(thumbnail.url.clone()));
                    template_data.insert("yt_video_link".to_string(),TempData::Text(content.url.clone()));
                } else {
                    eprintln!("Missing thumbnail or content for entry: {:?}", entry);
                    continue;
                }
            } else {
                eprintln!("Missing media group for entry: {:?}", entry);
                continue;
            }

            template_data.insert("yt_video_title".to_string(),TempData::Text(entry.title.clone()));

            let template = render_final_template(video_html.clone(), template_data);
            full_entries.push_str(&template);
        }
    }

    let main_html = match read_html_file("src/assets/templates/youtube.html") {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error reading main HTML file: {}", e);
            return Err(WidgetError::NoHtmlToString);
        }
    };

    let final_render_html = insert_html(main_html, full_entries);

    // Inject theme and widget heading
    let mut template_data: HashMap<String, TempData> = HashMap::new();
    template_data.insert("widget_theme".to_string(), TempData::Text(theme));
    template_data.insert("widgetHeading".to_string(), TempData::Text(_widget_theme));

    let rendered_html = render_final_template(final_render_html, template_data);
    match _widget_cache.insert(WIDGET_NAME.to_string(), rendered_html.clone()).await {
        Ok(_) => {
            // Inserted to Cache
        }
        Err(e) => {
            eprintln!("Failed to insert widget into cache: {}", e);
        }
    }
    Ok(rendered_html)
}
