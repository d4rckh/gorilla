use regex::Regex;
use reqwest;

pub async fn download_page(page_url: &str) -> Result<String, reqwest::Error> {
  let res = reqwest::get(page_url).await?;
  let body = res.text().await?;

  Ok(body)
}

// pub fn remove_script_tags(page_body: &str) -> String {
//   let mut body_wo_scripts = String::new();


//   body_wo_scripts
// }

pub fn extract_words(page_body: &str) -> Vec<String> {
  let mut words: Vec<String> = vec![ ];
  
  let html_tag = Regex::new("<[^>]*>").unwrap();
  // let body_wo_scripts = remove_script_tags(&page_body);

  // println!("{body_wo_scripts}");

  for line in html_tag
    .replace_all(&page_body, "")
    .split("\n")
    .collect::<Vec<&str>>() {
    let trimmed_line = line.trim();
    if !trimmed_line.is_empty() {
      for word in trimmed_line.split(" ").collect::<Vec<&str>>() {
        let w = word.trim().to_owned();
        if !words.contains(&w) {
          words.push(w)
        } 
      }
    }
  }

  words
}