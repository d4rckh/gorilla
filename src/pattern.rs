use std::{
    fmt::{self, Display},
    vec,
};

use crate::char_sets;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    String(String),
    Repeat(u32, u32),
    CharSet(String),
    Numbers(u32, u32),
}

impl Token {
    fn range(&self) -> u128 {
        match &self {
            Token::String(_) => 1,
            Token::Repeat(start, end) => (end - start + 1) as u128,
            Token::CharSet(chars) => chars.len() as u128,
            Token::Numbers(start, end) => (end - start + 1) as u128,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::String(s) => write!(f, "string: {}", s),
            Token::Repeat(start, end) => write!(f, "repeat: {} -> {}", start, end),
            Token::CharSet(ch_set) => write!(f, "char_set: {}", ch_set),
            Token::Numbers(start, end) => write!(f, "numbers: {} -> {}", start, end),
        }
    }
}

pub fn tokenize_format_string(input: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut inside_repeat = false;

    let mut cur = String::new();

    for character in input.chars() {
        if character == '{' {
            if !cur.is_empty() {
                if inside_repeat {
                    result.push(Token::String("{".to_owned()));
                }
                result.push(Token::String(cur.clone()));
                cur.clear();
            }
            inside_repeat = true;
        } else if character == '}' {
            inside_repeat = !inside_repeat;
            let inside_len = cur.chars().collect::<Vec<char>>().len();
            if inside_len >= 4 && cur.contains('-') {
                let start_num = cur.split('-').next().unwrap();
                let end_num = cur.split('-').nth(1).unwrap();
                result.push(Token::Numbers(
                    start_num.parse::<u32>().unwrap(),
                    end_num.parse::<u32>().unwrap(),
                ))
            } else if inside_len > 2 && cur.contains('-') {
                let ch_start = cur.chars().next().unwrap();
                let ch_end = cur.chars().nth(2).unwrap();
                result.push(Token::Repeat(ch_start as u32, ch_end as u32));
            } else {
                // Combine character sets for multi-charset tokens
                let mut combined_charset = String::new();
                for ch in cur.chars() {
                    match ch {
                        'l' => combined_charset.push_str(char_sets::L_CH),
                        'u' => combined_charset.push_str(char_sets::U_CH),
                        'd' => combined_charset.push_str(char_sets::D_CH),
                        's' => combined_charset.push_str(char_sets::S_CH),
                        _ => {} // Ignore unsupported characters
                    }
                }

                if !combined_charset.is_empty() {
                    result.push(Token::CharSet(combined_charset));
                } else if !cur.is_empty() {
                    // Fallback if no valid charsets found
                    result.push(Token::String(cur.clone()));
                }
            }
            cur.clear();
        } else {
            cur.push(character)
        };
    }

    if inside_repeat {
        result.push(Token::String("{".to_owned()));
    }

    if !cur.is_empty() {
        result.push(Token::String(cur));
    }

    // for token in &result{ println!("(debug) tokenized: {}", token) }

    result
}
pub struct TokenIter {
    pub toks: Vec<Token>,
    pub current_index: u128,
    pub end_index: u128,
}

impl Display for TokenIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.current_index, self.end_index)
    }
}

pub fn token_iterator_from_start_end(tokens: &[Token], start: u128, end: u128) -> TokenIter {
    TokenIter {
        toks: tokens.to_owned(),
        current_index: start,
        end_index: end,
    }
}

pub fn token_iterator(tokens: &[Token]) -> TokenIter {
    let mut iter = TokenIter {
        toks: tokens.to_owned(),
        current_index: 0,
        end_index: 0,
    };
    iter.end_index = calculate_total_generations(&iter.toks);
    iter
}

pub fn calculate_total_generations(tokens: &[Token]) -> u128 {
    tokens.iter().fold(1, |acc, tok| {
        acc * match tok {
            Token::String(_) => 1,
            Token::Repeat(start, end) => (end - start + 1) as u128,
            Token::CharSet(chars) => chars.len() as u128,
            Token::Numbers(start, end) => (end - start + 1) as u128,
        }
    })
}

impl Iterator for TokenIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.end_index {
            return None;
        }

        let mut result = String::new();
        let mut temp_index = self.current_index;

        let mut indices = vec![0usize; self.toks.len()];

        for i in (0..self.toks.len()).rev() {
            let range = self.toks.get(i).unwrap().range();
            indices[i] = (temp_index % range) as usize;
            temp_index /= range;
        }

        for (i, tok) in self.toks.iter().enumerate() {
            match tok {
                Token::String(s) => result.push_str(s),
                Token::Repeat(start, _) => {
                    let c = char::from_u32(start + indices[i] as u32).unwrap();
                    result.push(c);
                }
                Token::CharSet(chars) => {
                    let c = chars.chars().nth(indices[i]).unwrap();
                    result.push(c);
                }
                Token::Numbers(start, _) => {
                    result.push_str(&(start + indices[i] as u32).to_string());
                }
            }
        }

        self.current_index += 1;
        Some(result)
    }
}

pub fn calculate_sample_size_bytes(tokens: &Vec<Token>) -> u128 {
    let mut sample_str = String::new();

    for tok in tokens {
        match tok {
            Token::String(s) => sample_str.push_str(s),
            Token::Repeat(start, _) => sample_str.push(char::from_u32(*start).unwrap()),
            Token::CharSet(ch_set) => sample_str.push(ch_set.chars().next().unwrap()),
            Token::Numbers(start, _) => sample_str.push_str(&start.to_string()),
        }
    }

    sample_str.push('\n'); // written on disk with a new line so we add a new line

    sample_str.len() as u128 * calculate_total_generations(tokens)
}
