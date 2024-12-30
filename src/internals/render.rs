use futures::future::join_all;
use regex::{Regex, Captures};
use std::{fs, fmt, collections::HashMap};
use std::io::{self, Read};
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;
use std::sync::Arc;
use crate::widgets::weather::weather_widget_handler;
use crate::widgets::clock::clock_widget_handler;
use crate::widgets::calendar::calendar_widget_handler;
use crate::widgets::header::header_widget_handler;
use crate::internals::singularity::Widget;
use crate::internals::singularity::{Config, WidgetError};
use crate::internals::cache::GenericWidgetCache;
use actix_web::web::Data;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TempData {
    Number(i32),
    Boolean(bool),
    Text(String),
}

type WidgetHandler = fn(
    String,
    String,
    Data<Arc<GenericWidgetCache>>,
    Widget
) -> Pin<Box<dyn Future<Output = Result<String, WidgetError>> + Send>>;

impl fmt::Display for TempData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(x) => write!(f, "{}", x),
            Self::Number(x) => write!(f, "{}", x),
            Self::Boolean(x) => write!(f, "{}", x),
        }
    }
}

pub fn read_html_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn render_final_template(template: String, data: HashMap<String, TempData>) -> String {
    let print_regex = Regex::new(r"\{\{\s*(.*?)\s*\}\}").unwrap();
    let result = print_regex.replace_all(&template, |caps: &Captures| {
        let key = caps.get(1).unwrap().as_str().trim();
        if let Some(value) = data.get(key) {
            value.to_string()
        } else {
            format!("{{{{ {} }}}}", key)
        }
    }).to_string();
    result.replace("{#", "<!--").replace("#}", "-->")
}

pub fn hydrate_val_once(template: String, pattern: String, val: String) -> String {
    let escaped_pattern = regex::escape(&pattern);
    let placeholder = format!(r"\{{\{{\s*{}\s*\}}\}}", escaped_pattern);
    let print_regex = Regex::new(&placeholder).unwrap();
    print_regex.replace(&template, val.as_str()).to_string()
}

pub fn insert_html(outer: String, inner: String) -> String {
    let placeholder_regex = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    let result = placeholder_regex.replace(&outer, |_: &Captures| {
        inner.clone()
    });
    result.to_string()
}

pub fn insert_html_once(outer: String, inner: String) -> String {
    let placeholder_regex = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    if let Some(matched) = placeholder_regex.find(&outer) {
        let mut result = outer.clone();
        result.replace_range(matched.range(), &inner);
        result
    } else {
        outer
    }
}

pub async fn final_yaml_to_html_render(
    data_config: &Data<Config>,
    mut final_html: String,
    widget_cache: &Data<Arc<GenericWidgetCache>>,
    render_page_name: String,
) -> String {
    let start = Instant::now();
    let mut widget_map: HashMap<&str, WidgetHandler> = HashMap::new();

    widget_map.insert("clock", |s1, s2, cache, widget| Box::pin(clock_widget_handler(s1, s2, cache, widget)));
    widget_map.insert("weather", |s1, s2, cache, widget| Box::pin(weather_widget_handler(s1, s2, cache, widget)));
    widget_map.insert("calendar", |s1, s2, cache, widget| Box::pin(calendar_widget_handler(s1, s2, cache, widget)));

    match read_html_file("src/assets/templates/document.html") {
        Ok(doc_html) => {
            final_html = doc_html;
            if !data_config.pages.is_empty() {

                // Injecting theme
                let mut template_data: HashMap<String, TempData> = HashMap::new();
                template_data.insert("widget_theme".to_string(), TempData::Text(data_config.theme.to_string()));
                template_data.insert("theme_background_color".to_string(), TempData::Text(data_config.theme_background_color.to_string()));
                template_data.insert("footerTheme".to_string(), TempData::Text(data_config.footer.to_string()));
                template_data.insert("page_title".to_string(), TempData::Text(render_page_name.to_string()));
                final_html = render_final_template(final_html, template_data);

                // Injecting page links
                let mut link_final_render = String::new();
                for page in &data_config.pages {
                    let link_template: String;
                    if page.name == render_page_name {
                        link_template = format!("<a href=\"/pages/{}\" class=\"page-selected font-bold hover:text-white cursor-pointer text-gray-700\"><span class=\"shuffle cursor-pointer h-[100%]\">{}</span></a>", page.name, page.name);
                    } else {
                        link_template = format!("<a href=\"/pages/{}\" class=\"font-bold hover:text-white cursor-pointer text-gray-700\"><span class=\"shuffle cursor-pointer h-[100%]\">{}</span></a>", page.name, page.name);
                    }
                    link_final_render.push_str(&link_template);
                }
                final_html = insert_html_once(final_html, link_final_render);

                // Injecting each page
                for page in &data_config.pages {

                    if page.name == render_page_name {

                        // Injectiong the header widget
                        match page.header_widget {
                            Some(true) => {
                                let header_widget_render = match header_widget_handler(
                                    data_config.theme.to_string(),
                                    widget_cache.clone()
                                ).await {
                                    Ok(rendered_html) => rendered_html,
                                    Err(e) => {
                                        eprintln!("Error rendering header widget: {}", e);
                                        String::new() // fallback HTML for Header widget
                                    }
                                };
                                final_html = insert_html_once(final_html, header_widget_render);
                                let port = format!("http://127.0.0.1:{}/stats", data_config.clone().port.unwrap_or(8080).to_string());
                                final_html = hydrate_val_once(final_html, "singularity_link".to_string(), port); 
                            },
                            Some(false) => {
                                final_html = insert_html_once(final_html, " ".to_string());
                            },
                            None => {
                                final_html = insert_html_once(final_html, " ".to_string());
                            }
                        }

                        if !page.columns.is_empty() {

                            // Injecting each column
                            for (col_index, column) in page.columns.iter().enumerate() {
                                match read_html_file("src/assets/templates/column.html") {
                                    Ok(mut col_html) => {
                                        if col_index != page.columns.len() - 1 {
                                            col_html = format!("{}{}", col_html, "[[ SURPRISE ]]");
                                        }
                                        final_html = insert_html(final_html, col_html);

                                        // Collect all widget futures
                                        let widget_futures: Vec<_> = column.widgets.iter()
                                            .enumerate()
                                            .map(|(row_index, widget)| {
                                                let widget_type = match widget {
                                                    Widget::Clock => "clock",
                                                    Widget::Calendar => "calendar",
                                                    Widget::Weather { config: _ } => "weather",
                                                    Widget::Youtube { config: _ } => "youtube",
                                                    Widget::Rss { config: _ } => "rss",
                                                };

                                                let func = widget_map.get(widget_type).unwrap();
                                                let widget_clone = widget.clone(); // Clone the widget to own it
                                                async move {
                                                    let mut widget_html = func(
                                                        data_config.theme.to_string(),
                                                        data_config.widget_heading.to_string(),
                                                        widget_cache.clone(),
                                                        widget_clone,  // Pass the owned widget
                                                    )
                                                    .await?;

                                                    if row_index != column.widgets.len() - 1 {
                                                        widget_html = format!("{}{}", widget_html, "[[ Content ]]");
                                                    }

                                                    Ok::<String, WidgetError>(widget_html)
                                                }
                                            })
                                            .collect();

                                        // Execute all widget futures in parallel
                                        match join_all(widget_futures).await.into_iter().collect::<Result<Vec<_>, _>>() {
                                            Ok(widget_htmls) => {
                                                for widget_html in widget_htmls {
                                                    final_html = insert_html(final_html, widget_html);
                                                }
                                            }
                                            Err(e) => eprintln!("Error rendering widgets: {}", e),
                                        }
                                    }
                                    Err(e) => eprintln!("Error in page HTML file: {}", e),
                                }
                            }
                        }
                    }
                }
            }
            let duration = start.elapsed();
            println!("♾️  Rendered in {:?}", duration);
            final_html
        }
        Err(e) => {
            eprintln!("Couldn't find Src folder ⚠️: {}", e);
            final_html
        }
    }
}
