use regex::Regex;
use scraper::{Html, Selector};

pub fn download_page(page_url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(page_url).call()?.into_string()?;

    Ok(body)
}

/// Remove everything from page_body except the
/// HTML within the <body></body> HTML tags.
fn just_body_html_content(all_html: &str) -> String {
    let fragment = Html::parse_document(all_html);
    let selector = Selector::parse("body").unwrap();
    let body = fragment.select(&selector).next().unwrap();
    body.text().collect::<Vec<&str>>().join(" ")
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
