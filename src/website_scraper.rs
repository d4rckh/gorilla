use scraper::{Html, Selector};
use std::collections::BTreeSet;

pub fn download_page(page_url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(page_url).call()?.body_mut().read_to_string()?;

    Ok(body)
}

pub fn extract_words(page_body: &str) -> Vec<String> {
    let page_body = just_body_html_content(page_body);
    let document = Html::parse_fragment(&page_body);
    let text_content: String = document.root_element().text().collect();
    let mut words_set = BTreeSet::new();

    for word in text_content.split_whitespace() {
        let filtered_word: String = word.chars().filter(|c| c.is_alphabetic()).collect();
        if filtered_word.len() > 4 {
            words_set.insert(filtered_word.to_lowercase());
        }
    }

    words_set.into_iter().collect()
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
