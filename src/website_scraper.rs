use regex::Regex;
use scraper::{Html, Selector};

pub fn download_page(page_url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(page_url).call()?.body_mut().read_to_string()?;

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
/// We also will ignore any content between any and all <script> tags,
/// if there are any.
/// If no <body> tag is found, or there's any other error,
/// this function just silently returns the given
/// all_html
pub fn just_body_html_content(all_html: &str) -> String {
    // Parse the HTML
    let document = Html::parse_document(all_html);

    // Select the <body> tag
    let body_selector = Selector::parse("body").unwrap();
    if let Some(body_element) = document.select(&body_selector).next() {
        // Extract the inner HTML of <body>
        let mut body_html = body_element.inner_html();

        // Remove <script> elements
        let script_selector = Selector::parse("script").unwrap();
        for script in body_element.select(&script_selector) {
            let script_html = script.html();
            body_html = body_html.replace(&script_html, "");
        }

        return body_html;
    }

    // If no <body> tag is found, return the original HTML
    all_html.to_string()
}
