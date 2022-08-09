#[cfg(test)]
mod token_tests {
  use crate::formatting::{tokenize_format_string, Token, Tokens};

  #[test]
  fn tokenize_string_repeat() {
    let tokens = tokenize_format_string("hello{0-9}world");
    assert_eq!(tokens[1], Token::Repeat(48, 57, 48))
  }

  #[test]
  fn tokenize_string_string() {
    let tokens = tokenize_format_string("hello{0-9}world");
    assert_eq!(tokens[2], Token::String(String::from("world")))
  }

  #[test]
  fn tokenize_execute_letters() {
    let ac_toks = Tokens { 
      toks: tokenize_format_string("{a-z}{a-z}")
    };
    let total = ac_toks.calculate_total();
    let result: Vec<String> = ac_toks.take(total).collect();
    
    assert_eq!(result.len(), 26*26)
  }

  #[test]
  fn tokenize_execute_ascii() {
    let ac_toks = Tokens { 
      toks: tokenize_format_string("{ -~}")
    };
    let total = ac_toks.calculate_total();
    let result: Vec<String> = ac_toks.take(total).collect();
    
    assert_eq!(result.len(), 95)
  }
}

#[cfg(test)]
mod mutation_tests {
  use crate::mutation::{Mutation, Action, MutationSet, MutationResult};

  #[test]
  fn basic_mutations() {
    let mut mutation_result = MutationResult {
      original_word: String::from("word"),
      mutated_words: vec![]
    };

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

    mutation_set.perform(&mut mutation_result, "word");

    assert_eq!(mutation_result.mutated_words, vec!["abcdrowabc"])
  }

  #[test]
  fn advanced_mutation() {

    let mut mutation_result = MutationResult {
      original_word: String::from("word"),
      mutated_words: vec![]
    };

    let mutation_set = MutationSet { 
      mutations: vec![
        Mutation {
          action: Action::Append(String::from("{0-9}")),
          times: 1,
          keep_original: false
        }
      ] 
    };

    mutation_set.perform(&mut mutation_result, "word");

    assert_eq!(mutation_result.mutated_words, vec![
      "word0", "word1", "word2", "word3", "word4", 
      "word5", "word6", "word7", "word8",  "word9" 
    ])
  }
}

#[cfg(test)]
mod yaml_test {
  use crate::{yaml_parser::get_mutation_sets, mutation::MutationResult};

  #[test]
  fn yaml_parse_test() {
    let mut mutation_result = MutationResult {
      original_word: String::from("word"),
      mutated_words: vec![]
    };

    let mutation_sets = get_mutation_sets("name: alphabet
mutation_sets:
  - [ wipe, \"append:{a-z}\" ] # => a, b, c, ..., z");

    mutation_sets[0].perform(&mut mutation_result, "word");

    assert_eq!(mutation_result.mutated_words.len(), 26); 
  }
}