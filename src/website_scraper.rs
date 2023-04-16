use markup5ever::interface::tree_builder::TreeSink;
use regex::Regex;
use scraper::{Html, Selector};

pub fn download_page(page_url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(page_url).call()?.into_string()?;

    Ok(body)
}

pub fn extract_words(page_body: &str) -> Vec<String> {
    let page_body = just_body_html_content(page_body);

    let mut words: Vec<String> = vec![];

    let html_tag = Regex::new("<[^>]*>").unwrap();

    for line in html_tag
        .replace_all(&page_body, "")
        .split(' ')
        .collect::<Vec<&str>>()
    {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() {
            for word in trimmed_line.split(' ').collect::<Vec<&str>>() {
                let w = word.trim().to_owned();
                if words.contains(&w) {
                    continue;
                }
                words.push(w)
            }
        }
    }

    words
}

/// Remove everything from page_body except the
/// HTML within the <body></body> HTML tags.
/// If no <body> tag is found, or there's any other error,
/// this function just silently returns the given
/// all_html
pub fn just_body_html_content(all_html: &str) -> String {
    let mut fragment = Html::parse_document(all_html);

    let script_selector = Selector::parse("script").unwrap();
    let script_element = fragment.select(&script_selector).next().unwrap();
    fragment.remove_from_parent(&script_element.id());

    let body_selector = match Selector::parse("body") {
        Ok(body_selector) => body_selector,
        Err(_e) => return all_html.to_string(),
    };
    let body = match fragment.select(&body_selector).next() {
        Some(body) => body,
        None => return all_html.to_string(),
    };

    body.text().collect::<Vec<&str>>().join(" ")
}
