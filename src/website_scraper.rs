use scraper::Html;
use std::collections::BTreeSet;

pub fn download_page(page_url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(page_url).call()?.body_mut().read_to_string()?;

    Ok(body)
}

pub fn extract_words(page_body: &str) -> Vec<String> {
    let document = Html::parse_document(page_body);
    let mut words_set = BTreeSet::new();

    let mut clean_text = String::new();
    traverse_extract(document.root_element(), &mut clean_text);

    for word in clean_text.split_whitespace() {
        let filtered_word: String = word.chars().filter(|c| c.is_alphabetic()).collect();
        if filtered_word.len() > 4 {
            words_set.insert(filtered_word.to_lowercase());
        }
    }

    words_set.into_iter().collect()
}

fn traverse_extract(element: scraper::ElementRef, output: &mut String) {
    for node in element.children() {
        if let Some(el) = scraper::ElementRef::wrap(node) {
             let tag_name = el.value().name();
             if tag_name != "script" && tag_name != "style" {
                 traverse_extract(el, output);
             }
        } else if let Some(text) = node.value().as_text() {
            output.push_str(&text);
            output.push(' ');
        }
    }
}
