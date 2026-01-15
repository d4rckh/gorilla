use std::{
    fmt::{self, Display},
    vec,
};

use crate::char_sets;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    String(String),
    NumRange(u32, u32),
    CharSet(String),
    CharRange(u32, u32),
    Strings(Vec<String>),
}

impl Token {
    fn range(&self) -> u128 {
        match &self {
            Token::String(_) => 1,
            Token::NumRange(start, end) => (end - start + 1) as u128,
            Token::CharSet(chars) => chars.len() as u128,
            Token::CharRange(start, end) => (end - start + 1) as u128,
            Token::Strings(strings) => strings.len() as u128,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::String(s) => write!(f, "string: {}", s),
            Token::NumRange(start, end) => write!(f, "num_range: {} -> {}", start, end),
            Token::CharSet(ch_set) => write!(f, "char_set: {}", ch_set),
            Token::CharRange(start, end) => write!(f, "char_range: {} -> {}", start, end),
            Token::Strings(strings) => write!(f, "strings: {}", strings.join(" / ")),
        }
    }
}

fn parse_inner_brackets(cur: &str) -> Option<Token> {
    let inside_len = cur.chars().collect::<Vec<char>>().len();

    if cur.contains(',') {
        return Some(Token::Strings(
            cur.split(",").map(|x| x.trim().to_owned()).collect(),
        ));
    } else if inside_len >= 4 && cur.contains('-') {
        let start_num = cur.split('-').next().unwrap();
        let end_num = cur.split('-').nth(1).unwrap();

        return Some(Token::CharRange(
            start_num.parse::<u32>().unwrap(),
            end_num.parse::<u32>().unwrap(),
        ));
    } else if inside_len > 2 && cur.contains('-') {
        let ch_start = cur.chars().next().unwrap();
        let ch_end = cur.chars().nth(2).unwrap();

        return Some(Token::NumRange(ch_start as u32, ch_end as u32));
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
            return Some(Token::CharSet(combined_charset));
        }
    }

    None
}

pub fn tokenize_format_string(input: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut inside_brackets = false;

    let mut cur = String::new();

    for character in input.chars() {
        if character == '{' {
            if inside_brackets {
                cur.push('{');
            }
            if !cur.is_empty() {
                if let Some(Token::String(s)) = result.last_mut() {
                    s.push_str(&cur);
                } else {
                    result.push(Token::String(cur.clone()));
                }
                cur.clear();
            }
            inside_brackets = true;
        } else if character == '}' {
            if !inside_brackets {
                cur.push('}');
                continue;
            }

            inside_brackets = false;

            // if the contains for the brackets aren't valid we will just consider
            // the contents including the brackets to be a string
            if let Some(token) = parse_inner_brackets(&cur) {
                result.push(token);
            } else if let Some(Token::String(s)) = result.last_mut() {
                s.push_str(&format!("{{{}}}", cur));
            } else {
                result.push(Token::String(format!("{{{}}}", cur)));
            }
            cur.clear();
        } else {
            cur.push(character)
        };
    }

    if inside_brackets {
        cur.push('{');
    }

    if let Some(Token::String(s)) = result.last_mut() {
        s.push_str(&cur);
    } else {
        result.push(Token::String(cur));
    }

    // for token in &result{ println!("(debug) tokenized: {}", token) }

    result
}

pub struct TokenIter {
    pub toks: Vec<Token>,
    indices: Vec<usize>,
    pub current_index: u128,
    pub end_index: u128,
}

impl Display for TokenIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "token_iter[{} tokens, {} -> {}]",
            self.toks.len(),
            self.current_index,
            self.end_index
        )
    }
}

pub fn token_iterator_from_start_end(tokens: &[Token], start: u128, end: u128) -> TokenIter {
    TokenIter {
        toks: tokens.to_owned(),
        current_index: start,
        end_index: end,
        indices: vec![0usize; tokens.len()],
    }
}

pub fn token_iterator(tokens: &[Token]) -> TokenIter {
    let mut iter = TokenIter {
        toks: tokens.to_owned(),
        current_index: 0,
        end_index: 0,
        indices: vec![0usize; tokens.len()],
    };
    iter.end_index = calculate_total_generations(&iter.toks);
    iter
}

pub fn calculate_total_generations(tokens: &[Token]) -> u128 {
    tokens.iter().fold(1, |acc, tok| acc * tok.range())
}

impl Iterator for TokenIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.end_index {
            return None;
        }

        let mut result = String::new();
        let mut temp_index = self.current_index;

        // rebuild self.indices
        for i in (0..self.toks.len()).rev() {
            let range = self.toks.get(i).unwrap().range();
            self.indices[i] = (temp_index % range) as usize;
            temp_index /= range;
        }

        for (i, tok) in self.toks.iter().enumerate() {
            match tok {
                Token::String(s) => result.push_str(s),
                Token::NumRange(start, _) => {
                    let c = char::from_u32(start + self.indices[i] as u32).unwrap();
                    result.push(c);
                }
                Token::CharSet(chars) => {
                    let c = chars.chars().nth(self.indices[i]).unwrap();
                    result.push(c);
                }
                Token::CharRange(start, _) => {
                    result.push_str(&(start + self.indices[i] as u32).to_string());
                }
                Token::Strings(strings) => {
                    result.push_str(strings.get(self.indices[i]).unwrap());
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
            Token::NumRange(start, end) => {
                sample_str.push(char::from_u32((start + end) / 2).unwrap())
            }
            Token::CharSet(ch_set) => sample_str.push(ch_set.chars().next().unwrap()),
            Token::CharRange(start, _) => sample_str.push_str(&start.to_string()),
            Token::Strings(strings) => {
                for _ in 0..(strings.iter().fold(0usize, |p, c| p + c.len()) / strings.len()) {
                    sample_str.push('X');
                }
            }
        }
    }

    sample_str.push('\n'); // written on disk with a new line so we add a new line

    sample_str.len() as u128 * calculate_total_generations(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_string_repeat() {
        let tokens = tokenize_format_string("hello{0-9}world");
        assert_eq!(tokens[1], Token::NumRange('0' as u32, '9' as u32))
    }

    #[test]
    fn tokenize_string_string() {
        let tokens = tokenize_format_string("hello{0-9}world");
        assert_eq!(tokens[2], Token::String(String::from("world")))
    }

    #[test]
    fn properly_tokenize_double_brackets() {
        let tokens = tokenize_format_string("{{0-9}}");

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::String("{".to_string()));
        assert_eq!(tokens[2], Token::String("}".to_string()))
    }

    #[test]
    fn properly_tokenize_double_brackets_2() {
        let tokens = tokenize_format_string("{{{}}{0-9}}{}}}");

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::String("{{{}}".to_string()));
        assert_eq!(tokens[2], Token::String("}{}}}".to_string()))
    }

    #[test]
    fn tokenize_execute_letters() {
        let ac_toks = token_iterator(&tokenize_format_string("{a-z}{a-z}"));
        let result: Vec<String> = ac_toks.collect();

        assert_eq!(result.len(), 26 * 26)
    }

    #[test]
    fn tokenize_execute_ascii() {
        let ac_toks = token_iterator(&tokenize_format_string("{ -~}"));
        let result: Vec<String> = ac_toks.collect();

        assert_eq!(result.len(), 95)
    }

    #[test]
    fn sample_size_calculation() {
        let tokens = tokenize_format_string("{0-9}");
        // 10 chars + 10 newlines = 20 bytes?
        // calculate_sample_size_bytes implementation:
        // sample_str len * total generations
        // sample_str for {0-9} is one char (mid point). + newline = 2 bytes.
        // total generations = 10.
        // total = 20.
        assert_eq!(calculate_sample_size_bytes(&tokens), 20);
    }

    #[test]
    fn inner_bracket_parsing() {
        use super::parse_inner_brackets;

        if let Some(Token::Strings(s)) = parse_inner_brackets("Jan,Feb") {
            assert_eq!(s, vec!["Jan", "Feb"]);
        } else {
            panic!("Failed to parse comma list");
        }

        // 10-20 parses as CharRange because inside_len >= 4 and contains '-'
        // and parse_inner_brackets uses parse::<u32> for both parts.
        if let Some(Token::CharRange(s, e)) = parse_inner_brackets("10-20") {
            assert_eq!(s, 10);
            assert_eq!(e, 20);
        } else {
            panic!("Failed to parse num range (expecting CharRange for 10-20)");
        }
    }

    #[test]
    fn tokenize_edge_cases() {
        let empty = tokenize_format_string("{}");
        assert_eq!(empty[0], Token::String("{}".to_string()));

        let unbalanced = tokenize_format_string("{{}");
        // expecting "{{" to be treated as literal "{" inside cur, but parsed as "{{}"
        // Logic result was String("{{}")
        assert_eq!(unbalanced[0], Token::String("{{}".to_string()));
    }
}
