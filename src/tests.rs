#[cfg(test)]
mod token_tests {
  use crate::formatting::{tokenize_format_string, Token, execute_format_string};

  #[test]
  fn tokenize_string_repeat() {
    let tokens = tokenize_format_string("hello{0-9}world");
    assert_eq!(tokens[1], Token::Repeat(48, 57))
  }

  #[test]
  fn tokenize_string_string() {
    let tokens = tokenize_format_string("hello{0-9}world");
    assert_eq!(tokens[2], Token::String(String::from("world")))
  }

  #[test]
  fn tokenize_execute_letters() {
    let tokens = tokenize_format_string("{a-z}{A-Z}");
    let result = execute_format_string(&tokens);
    assert_eq!(result.len(), 26*26)
  }

  #[test]
  fn tokenize_execute_ascii() {
    let tokens = tokenize_format_string("{ -~}");
    let result = execute_format_string(&tokens);
    assert_eq!(result.len(), 95)
  }
}

#[cfg(test)]
mod mutation_tests {
  use crate::mutation::{Mutation, Action, MutationSet};

  #[test]
  fn basic_mutations() {
    let mutation_set = MutationSet { 
      mutations: vec![
        Mutation {
          action: Action::Reverse,
          times: 1,
          keep_original: false
        },
        Mutation {
          action: Action::Append(String::from("abc")),
          times: 1,
          keep_original: false
        },
        Mutation {
          action: Action::Prepend(String::from("abc")),
          times: 1,
          keep_original: false
        }
      ] 
    };
    assert_eq!(mutation_set.perform("word"), vec!["abcdrowabc"])
  }

  #[test]
  fn advanced_mutation() {
    let mutation_set = MutationSet { 
      mutations: vec![
        Mutation {
          action: Action::Append(String::from("{0-9}")),
          times: 1,
          keep_original: false
        }
      ] 
    };
    assert_eq!(mutation_set.perform("word"), vec![
      "word0", "word1", "word2", "word3", "word4", 
      "word5", "word6", "word7", "word8",  "word9" 
    ])
  }
}

#[cfg(test)]
mod yaml_test {
  use crate::yaml_parser::get_mutation_sets;

  #[test]
  fn yaml_parse_test() {
    let mutation_sets = get_mutation_sets("name: alphabet
mutation_sets:
  - [ wipe, \"append:{a-z}\" ] # => a, b, c, ..., z");
  
    assert_eq!(mutation_sets[0].perform("word").len(), 26); 
  }
}