use std::{fmt::{Display, self}, vec};

#[derive(Clone)]
pub enum Token {
  String(String),
  Repeat(u32, u32)
}

impl Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::String(s) => write!(f, "string: {}", s),
      Token::Repeat(start, end) => write!(f, "repeat: {} -> {}", start, end)
    }
  }
}

pub fn tokenize_format_string(input: &str) -> Vec<Token> {
  let mut result: Vec<Token> = vec![];
  let mut inside_repeat = false;

  let mut cur = String::new();

  for character in input.chars() {
    if character == '{' {
      inside_repeat = !inside_repeat;
      result.push(
        Token::String(cur.clone())
      );
      cur.clear();
    } else if character == '}' {
      inside_repeat = !inside_repeat;
      let ch_start = cur.chars().nth(0).unwrap();
      let ch_end = cur.chars().nth(2).unwrap();
      result.push(
        Token::Repeat(
          ch_start as u32, 
          ch_end as u32
        )
      );
      cur.clear();
    } else { cur.push(character) };
  }

  result.push(
    Token::String(cur.clone())
  );
  cur.clear();

  // for token in &result{ println!("(debug) tokenized: {}", token) }

  result
}

pub fn execute_format_string(tokens: &Vec<Token>) -> Vec<String> {
  let mut token_values: Vec<Vec<Token>> = vec![ tokens.to_vec() ];

  loop {
    let mut found_repeat = false;
    let mut new_token_values: Vec<Vec<Token>> = vec![];

    for tokens in &token_values {
      for (token_index, token) in tokens.iter().enumerate() {
        match token {
          Token::Repeat(start, end) => {
            found_repeat = true;

            for i in *start..(*end+1) {
              let mut new_tokens = tokens.clone();
              new_tokens[token_index] = Token::String(
                char::from_u32(i).unwrap().to_string()
              );

              new_token_values.push(new_tokens)
            }

            break
          }
          _ => ()
        }
      }
    }

    if !found_repeat { break; } 
    else { token_values = new_token_values; }
  }

  let mut result: Vec<String> = vec![];

  let mut string_format = String::new();
  for tokens in &token_values {

    for token in tokens {
      match token {
        Token::String(a) => {
          let token_string = a.clone();
          string_format.push_str(&token_string)
        }
        _ => ()
      }
    }

    result.push(string_format.clone());
    string_format.clear();
  }

  result
}