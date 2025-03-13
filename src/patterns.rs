use std::{
    fmt::{self, Display},
    vec,
};

use crate::char_sets;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    String(String),
    Repeat(u32, u32, u32),
    CharSet(String, usize),
    Numbers(u32, u32, u32),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::String(s) => write!(f, "string: {}", s),
            Token::Repeat(start, end, _) => write!(f, "repeat: {} -> {}", start, end),
            Token::CharSet(ch_set, _) => write!(f, "char_set: {}", ch_set),
            Token::Numbers(start, end, _) => write!(f, "numbers: {} -> {}", start, end),
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
            if !cur.is_empty() {
                result.push(Token::String(cur.clone()));
                cur.clear();
            }
        } else if character == '}' {
            inside_repeat = !inside_repeat;
            let inside_len = cur.chars().collect::<Vec<char>>().len();
            if inside_len >= 4 && cur.contains('-') {
                let start_num = cur.split('-').nth(0).unwrap();
                let end_num = cur.split('-').nth(1).unwrap();
                result.push(Token::Numbers(
                    start_num.parse::<u32>().unwrap(),
                    end_num.parse::<u32>().unwrap(),
                    0,
                ))
            } else if inside_len > 2 && cur.contains('-') {
                let ch_start = cur.chars().next().unwrap();
                let ch_end = cur.chars().nth(2).unwrap();
                result.push(Token::Repeat(
                    ch_start as u32,
                    ch_end as u32,
                    ch_start as u32,
                ));
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
                    result.push(Token::CharSet(combined_charset, 0));
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

    if !cur.is_empty() {
        result.push(Token::String(cur));
    }

    // for token in &result{ println!("(debug) tokenized: {}", token) }

    result
}
pub struct TokenIter {
    pub toks: Vec<Token>,
    repeat_len: usize,
    done: bool,
}

pub fn token_iterator(tokens: &[Token]) -> TokenIter {
    TokenIter {
        toks: tokens.to_owned(),
        done: false,
        repeat_len: tokens
            .iter()
            .filter(|e| {
                matches!(e, Token::Repeat(_, _, _))
                    || matches!(e, Token::CharSet(_, _))
                    || matches!(e, Token::Numbers(_, _, _))
            })
            .count(),
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
                Token::Numbers(start, end, cur) => {
                    result.push_str(&(*start + *cur).to_string());

                    if !inc_next {
                        continue;
                    }

                    if *cur + *start != *end {
                        inc_next = false;
                        *cur += 1;
                        continue;
                    }

                    *cur = 0;

                    if current_tok == self.repeat_len {
                        self.done = true;
                        return Some(result);
                    }

                    current_tok += 1;
                }
                Token::Repeat(start, end, cur) => {
                    result.push(char::from_u32(*cur).unwrap());

                    if !inc_next {
                        continue;
                    }

                    if cur != end {
                        inc_next = false;
                        *cur += 1;
                        continue;
                    }

                    *cur = *start;

                    if current_tok == self.repeat_len {
                        self.done = true;
                        return Some(result);
                    }

                    current_tok += 1;
                }
                Token::CharSet(ch_set, cur) => {
                    result.push(ch_set.chars().nth(*cur).unwrap());

                    if !inc_next {
                        continue;
                    }

                    if *cur != (ch_set.len() - 1) {
                        inc_next = false;
                        *cur += 1;
                        continue;
                    }

                    *cur = 0;

                    if current_tok == self.repeat_len {
                        self.done = true;
                        return Some(result);
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
            if let Token::Numbers(start, end, _) = tok {
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
                Token::Repeat(start, _, _) => sample_str.push(char::from_u32(*start).unwrap()),
                Token::CharSet(ch_set, _) => sample_str.push(ch_set.chars().next().unwrap()),
                Token::Numbers(start, _, _) => sample_str.push_str(&start.to_string()),
            }
        }

        sample_str.push('\n'); // written on disk with a new line so we add a new line

        sample_str.len() as u128 * self.calculate_total()
    }
}
