use regex::{Regex, Captures};
use std::{fs, fmt, collections::HashMap};
use std::io::{self, Read};

#[derive(Debug)]
pub enum TempData {
    Number(i32),
    Boolean(bool),
    Text(String)
}

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

// pub async fn final_yaml_to_html_render() -> Result<String, WeatherError> {
//     let mut widget_map = HashMap::new()
// }