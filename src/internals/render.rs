use regex::{Regex, Captures};
use std::{fs, fmt, collections::HashMap};
use std::io::{self, Read};
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;
use crate::widgets::weather::weather_widget_handler;
use crate::widgets::clock::clock_widget_handler;
use crate::internals::singularity::{Config, WidgetError};
use actix_web::web::Data;

#[derive(Debug)]
pub enum TempData {
    Number(i32),
    Boolean(bool),
    Text(String)
}

type WidgetHandler = fn(String) -> Pin<Box<dyn Future<Output = Result<String, WidgetError>> + Send>>;

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

//TODO: Try removing the parameter for clock widget

// TODO: Going with a simple rendering method, find a better method for faster parse and render

pub async fn final_yaml_to_html_render(data_config: &Data<Config>, mut final_html: String) -> String {
    let start = std::time::Instant::now();
    let mut widget_map: HashMap<&str, WidgetHandler> = HashMap::new();

    widget_map.insert("clock", |s: String| Box::pin(clock_widget_handler(s)));
    widget_map.insert("weather", |s: String| Box::pin(weather_widget_handler(s)));

    match read_html_file("src/assets/templates/document.html") {
        Ok(doc_html) => {
            final_html = doc_html;
            if !data_config.pages.is_empty() {
                for page in &data_config.pages {
                    if !page.columns.is_empty() {
                        for column in &page.columns {
                            match read_html_file("src/assets/templates/page.html") {
                                Ok(page_html) => {
                                    final_html = insert_html(final_html, page_html);
                                    for widget in &column.widgets {
                                        if let Some(func) = widget_map.get(widget.widget_type.as_str()) {
                                            match func("Bengaluru".to_string()).await {
                                                Ok(widget_html) => {
                                                    let widget_html = format!("{}{}", widget_html, "[[ Content ]]");
                                                    final_html = insert_html(final_html, widget_html);
                                                }
                                                Err(e) => println!("Error in render function: {}", e),
                                            }
                                        }
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
            final_html
        }
        Err(e) => {
            eprintln!("Error in main HTML file: {}", e);
            final_html
        }
    }
}
