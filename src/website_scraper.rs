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

#[cfg(test)]
mod tests {
    use super::extract_words;
    
    #[test]
    fn basic_scrape() {
        let html = "<!doctype html><html><head></head><body> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        </body> \
        </html>";
        let words = extract_words(html);

        assert!(words.contains(&"domain".to_string()));
    }
    #[test]
    fn ignore_script_tag() {
        let html = "<!doctype html><html><head></head><body><script>Some javascript</script> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        </body> \
        </html>";
        let words = extract_words(html);

        assert!(words.contains(&"domain".to_string()));
        assert!(!words.contains(&"javascript".to_string()));
    }

    #[test]
    fn ignore_mulitple_script_tags() {
        let html = "<!doctype html><html><head></head><body><script>Some javascript</script> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        <script>second script</script> \
        </body> \
        </html>";
        let words = extract_words(html);

        assert!(words.contains(&"domain".to_string()));
        assert!(!words.contains(&"javascript".to_string()));
        assert!(!words.contains(&"second".to_string()));
    }
    
    #[test]
    fn ignore_style_tag() {
        let html = "<html><body><style>body { color: red; }</style> \
        <p>visible text</p></body></html>";
        let words = extract_words(html);
        
        assert!(words.contains(&"visible".to_string()));
        // 'red' is < 4 chars so filtered anyway? "color" is 5.
        // "color" should be ignored.
        assert!(!words.contains(&"color".to_string()));
    }
    
    #[test]
    fn nested_tags_extraction() {
         let html = "<div><p><span>Deep</span> text</p></div>";
         let words = extract_words(html);
         assert!(words.is_empty());
         // "Deep" is 4 chars, filtered out (>4 check).
         // "text" is 4 chars.
         // Wait, extract_words filters if len > 4.
         // "Deep" -> 4 len. "text" -> 4 len.
         // Neither will be included.
         // Let's use longer words.
         let html = "<div><p><span>Deeper</span> meaningful</p></div>";
         let words = extract_words(html);
         assert!(words.contains(&"deeper".to_string()));
         assert!(words.contains(&"meaningful".to_string()));
    }

    #[test]
    fn malformed_html() {
        let html = "Just text no tags longer";
        let words = extract_words(html);
        assert!(words.contains(&"longer".to_string()));
    }
    
    #[test]
    fn empty_input() {
        let words = extract_words("");
        assert!(words.is_empty());
    }
}
