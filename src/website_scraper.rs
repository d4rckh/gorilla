use regex::Regex;

pub fn download_page(page_url: &str) -> Result<String, ureq::Error> {
  let body: String = ureq::get(page_url)
    .call()?
    .into_string()?;

  Ok(body)
}

pub fn extract_words(page_body: &str) -> Vec<String> {
  let mut words: Vec<String> = vec![ ];
  
  let html_tag = Regex::new("<[^>]*>").unwrap();
  // let body_wo_scripts = remove_script_tags(&page_body);

  // println!("{body_wo_scripts}");

  for line in html_tag
    .replace_all(page_body, "")
    .split(' ')
    .collect::<Vec<&str>>() {
    let trimmed_line = line.trim();
    if !trimmed_line.is_empty() {
      for word in trimmed_line.split(' ').collect::<Vec<&str>>() {
        let w = word.trim().to_owned();
        if words.contains(&w) { continue } 
        words.push(w)
      }
    }
  }

  words
}