use regex::{Regex, Captures};
use std::{fmt, collections::HashMap};

#[derive(Debug)]
enum TempData {
    Number(i32),
    Boolean(bool),
    Text(String)
}

impl fmt::Display for TempData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(x) => write!(f,"{}",x),
            Self::Number(x) => write!(f,"{}",x),
            Self::Boolean(x) => write!(f,"{}",x)
        }
    }
}

pub fn render_final_template(mut template: String, mut data: HashMap<&str, TempData>) -> String {
    let print_regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    template = print_regex.replace_all(&template, |caps: &Captures| {
        let key = caps.get(1).unwrap().as_str().trim();
        data[key].to_string()
    }).to_string();
    template = template.replace("{#", "<!--").replace("#}", "-->");
    template
}

pub fn insert_html(outer: String, inner: String) -> String {
    let placeholder_regex = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    let result = placeholder_regex.replace(&outer, |caps: &Captures| {
        let placeholder = caps.get(1).unwrap().as_str().trim();
        if placeholder.is_empty() {
            inner.clone()
        } else {
            inner.clone()
        }
    });
    result.to_string()
}