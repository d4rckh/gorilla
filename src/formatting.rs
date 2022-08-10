use std::{fmt::{Display, self}, vec};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  String(String),
  Repeat(u32, u32, u32)
}

impl Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::String(s) => write!(f, "string: {}", s),
      Token::Repeat(start, end, _) => write!(f, "repeat: {} -> {}", start, end)
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
      let ch_start = cur.chars().next().unwrap();
      let ch_end = cur.chars().nth(2).unwrap();
      result.push(
        Token::Repeat(
          ch_start as u32, 
          ch_end as u32,
          ch_start as u32
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
pub struct TokenIter {
  pub toks: Vec<Token>,
  repeat_len: usize,
  done: bool
}

pub fn token_iterator(tokens: &[Token]) -> TokenIter {
  TokenIter { 
    toks: tokens.to_owned(), done: false,
    repeat_len: tokens.iter()
      .filter(|e| matches!(e, Token::Repeat(_, _, _)))
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
          
          inc_next = true;
          *cur = *start;

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
    }

    result
  }

  pub fn calculate_size(&self) -> u128 {
    let mut sample_str = String::new();

    for tok in &self.toks {
      match tok {
        Token::String(s) => sample_str.push_str(s),
        Token::Repeat(start, _, _) => sample_str.push(char::from_u32(*start).unwrap())
      }
    }

    sample_str.push('\n'); // written on disk with a new line so we add a new line

    sample_str.len() as u128 * self.calculate_total()
  }
}

// pub fn execute_format_string(tokens: &Vec<Token>) -> Vec<String> {
//   let mut token_values: Vec<Vec<Token>> = vec![ tokens.to_vec() ];

//   loop {
//     let mut found_repeat = false;
//     let mut new_token_values: Vec<Vec<Token>> = vec![];

//     for tokens in &token_values {
//       for (token_index, token) in tokens.iter().enumerate() {
//         match token {
//           Token::Repeat(start, end) => {
//             found_repeat = true;

//             for i in *start..(*end+1) {
//               let mut new_tokens = tokens.clone();
//               new_tokens[token_index] = Token::String(
//                 char::from_u32(i).unwrap().to_string()
//               );

//               new_token_values.push(new_tokens)
//             }

//             break
//           }
//           _ => ()
//         }
//       }
//     }

//     if !found_repeat { break; } 
//     else { token_values = new_token_values; }
//   }

//   let mut result: Vec<String> = vec![];

//   let mut string_format = String::new();
//   for tokens in &token_values {

//     for token in tokens {
//       match token {
//         Token::String(a) => {
//           let token_string = a.clone();
//           string_format.push_str(&token_string)
//         }
//         _ => ()
//       }
//     }

//     result.push(string_format.clone());
//     string_format.clear();
//   }

//   result
// }