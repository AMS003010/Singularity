use futures::future::join_all;
use regex::{Regex, Captures};
use std::{fs, fmt, collections::HashMap};
use std::io::{self, Read};
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;
use crate::widgets::weather::weather_widget_handler;
use crate::widgets::clock::clock_widget_handler;
use crate::widgets::calendar::calendar_widget_handler;
use crate::internals::singularity::{Config, WidgetError};
use actix_web::web::Data;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TempData {
    Number(i32),
    Boolean(bool),
    Text(String)
}

type WidgetHandler = fn(String, String) -> Pin<Box<dyn Future<Output = Result<String, WidgetError>> + Send>>;

impl fmt::Display for TempData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(x) => write!(f, "{}", x),
            Self::Number(x) => write!(f, "{}", x),
            Self::Boolean(x) => write!(f, "{}", x)
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
            format!("{{{{ {} }}}}", key)  // retain the placeholder if the key is not found
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
        result.replace_range(matched.range(),&inner);
        result
    } else {
        outer
    }
}

// TODO: Try removing the parameter for clock widget

// TODO: Going with a simple rendering method, find a better method for faster parse and render

// TODO: Adding a cache approach

pub async fn final_yaml_to_html_render(data_config: &Data<Config>, mut final_html: String) -> String {
    let start = Instant::now();
    let mut widget_map: HashMap<&str, WidgetHandler> = HashMap::new();

    widget_map.insert("clock", |s1: String, s2: String| Box::pin(clock_widget_handler(s1, s2)));
    widget_map.insert("weather", |s1: String, s2: String| Box::pin(weather_widget_handler(s1, s2)));
    widget_map.insert("calendar", |s1: String, s2: String| Box::pin(calendar_widget_handler(s1, s2)));

    match read_html_file("src/assets/templates/document.html") {
        Ok(doc_html) => {
            final_html = doc_html;
            if !data_config.pages.is_empty() {
                // Injecting theme
                let mut template_data: HashMap<String, TempData> = HashMap::new();
                template_data.insert("widget_theme".to_string(),TempData::Text(data_config.theme.to_string()));
                template_data.insert("theme_background_color".to_string(),TempData::Text(data_config.theme_background_color.to_string()));
                template_data.insert("footerTheme".to_string(),TempData::Text(data_config.footer.to_string()));
                final_html = render_final_template(final_html, template_data);

                for page in &data_config.pages {
                    if !page.columns.is_empty() {
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
                                            let func = widget_map.get(widget.widget_type.as_str()).unwrap();
                                            async move {
                                                // Add widget_heading as an additional argument
                                                let mut widget_html = func(data_config.theme.to_string(), data_config.widget_heading.to_string()).await?;
                                                if row_index != column.widgets.len() - 1 {
                                                    widget_html = format!("{}{}", widget_html, "[[ Content ]]");
                                                }
                                                Ok::<String, WidgetError>(widget_html)
                                            }
                                        }).collect();

                                    // Execute all widget futures in parallel
                                    match join_all(widget_futures).await.into_iter().collect::<Result<Vec<_>, _>>() {
                                        Ok(widget_htmls) => {
                                            for widget_html in widget_htmls {
                                                final_html = insert_html(final_html, widget_html);
                                            }
                                        }
                                        Err(e) => println!("Error rendering widgets: {}", e),
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error in page HTML file: {}", e);
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
            eprintln!("Error in main HTML file: {}", e);
            final_html
        }
    }
}
