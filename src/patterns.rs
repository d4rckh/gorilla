use std::{fmt::{Display, self}, vec};

use crate::char_sets;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  String(String),
  Repeat(u32, u32, u32),
  CharSet(String, usize)
}

impl Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::String(s) => write!(f, "string: {}", s),
      Token::Repeat(start, end, _) => write!(f, "repeat: {} -> {}", start, end),
      Token::CharSet(ch_set, _) => write!(f, "char_set: {}", ch_set)
    }
  }
}

pub fn tokenize_format_string(input: &str) -> Vec<Token> {
  let mut result: Vec<Token> = vec![];
  let mut inside_repeat = false;

  let mut cur = String::new();

  for character in input.chars() {
    if character == '{' && !inside_repeat {
      inside_repeat = !inside_repeat;
      result.push(
        Token::String(cur.clone())
      );
      cur.clear();
    } else if character == '}' {
      inside_repeat = !inside_repeat;
      let inside_len = cur.chars().collect::<Vec<char>>().len(); 
      if inside_len > 2 {
        let ch_start = cur.chars().next().unwrap();
        let ch_end = cur.chars().nth(2).unwrap();
        result.push(
          Token::Repeat(
            ch_start as u32, 
            ch_end as u32,
            ch_start as u32
          )
        );
      } else if inside_len == 1 {
        result.push(
          match cur.chars().next().unwrap() {
            'l' => Token::CharSet(char_sets::L_CH.to_owned(), 0),
            'u' => Token::CharSet(char_sets::U_CH.to_owned(), 0),
            'd' => Token::CharSet(char_sets::D_CH.to_owned(), 0),
            's' => Token::CharSet(char_sets::S_CH.to_owned(), 0),
            _ => Token::String(String::from(""))
          }
        )
      }
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
pub struct TokenIter {
  pub toks: Vec<Token>,
  repeat_len: usize,
  done: bool
}

pub fn token_iterator(tokens: &[Token]) -> TokenIter {
  TokenIter { 
    toks: tokens.to_owned(), done: false,
    repeat_len: tokens.iter()
      .filter(|e| matches!(e, Token::Repeat(_, _, _)) || matches!(e, Token::CharSet(_, _)))
      .count()
  }
}

impl Iterator for TokenIter {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done { 
      return None;
    }
    
    let mut result = String::new();
    let mut inc_next = true;

    let mut current_tok = 1;
    for tok in &mut self.toks {
      match tok {
        Token::String(s) => result.push_str(s),
        Token::Repeat(start, end, cur) => {
          result.push(char::from_u32(*cur).unwrap());          
        
          if !inc_next { continue }

          if cur != end {
            inc_next = false;
            *cur += 1;
            continue
          }
          
          *cur = *start;

          if current_tok == self.repeat_len {
            self.done = true;
            return Some(result)
          }

          current_tok += 1;        
        },
        Token::CharSet(ch_set, cur) => {
          result.push(ch_set.chars().nth(*cur).unwrap());

          if !inc_next { continue }

          if *cur != (ch_set.len() - 1 ) {
            inc_next = false;
            *cur += 1;
            continue;
          }

          *cur = 0;

          if current_tok == self.repeat_len {
            self.done = true;
            return Some(result)
          }

          current_tok += 1;
        }
      }
    }

    if self.repeat_len == 0 {
      self.done = true;
    }

    Some(result)
  }

}

impl TokenIter {
  pub fn calculate_total(&self) -> u128 {
    let mut result: u128 = 1;

    for tok in &self.toks {
      if let Token::Repeat(start, end, _) = tok {
        result *= (*end as u128) - (*start as u128) + 1
      }
      if let Token::CharSet(ch_set, _) = tok {
        result *= ch_set.len() as u128
      }
    }

    result
  }

  pub fn calculate_size(&self) -> u128 {
    let mut sample_str = String::new();

    for tok in &self.toks {
      match tok {
        Token::String(s) => sample_str.push_str(s),
        Token::Repeat(start, _, _) => sample_str.push(char::from_u32(*start).unwrap()),
        Token::CharSet(ch_set, _) => sample_str.push(ch_set.chars().next().unwrap())
      }
    }

    sample_str.push('\n'); // written on disk with a new line so we add a new line

    sample_str.len() as u128 * self.calculate_total()
  }
}
