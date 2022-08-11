use std::{fmt::{Display, self}, fs, io::Write, cmp::Ordering};

use crate::formatting::{tokenize_format_string, token_iterator};

#[derive(Debug)]
pub enum Action {
  Prepend(String),
  Append(String), 
  Replace(String, String),
  LowercaseAll,
  UppercaseAll,
  RemoveFirstLetter,
  RemoveLastLetter,
  Reverse,
  
  // more debugging related
  Clone,
  Wipe,
  Remove,
  Nothing,

  // conditional, same effect as wipe if conditions are not met
  // bool indicates if the condition should be negated
  IfCharacterLength(bool, Ordering, usize),
  IfContains(bool, String)
}

#[derive(Debug)]
pub enum MutationBuildError {
  ActionDoesNotExist,
  MissingArguments,
  InvalidArgument(String)
}

pub struct Mutation {
  pub action: Action,
  pub times: usize,
  pub keep_original: bool
}

pub struct MutationSet {
  pub mutations: Vec<Mutation>
}

pub struct MutationResult {
  pub original_word: String,
  pub mutated_words: Vec<String>
}

impl MutationResult {
  pub fn save_to_file(&self, file: &mut fs::File) {
    for mutated in &self.mutated_words {
      let log_entry = format!("{}\n", mutated);
      file.write_all(log_entry.as_bytes()).expect("write failed");  
    }
  }
}

impl MutationSet {
  pub fn perform(&self, mutation_result: &mut MutationResult, word: &str) {
    let mut result: Vec<String> = vec![ word.to_owned() ];

    for mutation in &self.mutations {
      let mut new_result: Vec<String> = vec![ ];
      for s in &result {
        mutation.perform(&mut new_result, s)
      }
      result = new_result    
    }

    mutation_result.mutated_words = result
  }

  pub fn empty_set() -> MutationSet {
    let mutation = Mutation {
      action: Action::Nothing,
      times: 1,
      keep_original: false
    };
    MutationSet { mutations: vec![ mutation ] }
  }
}

impl Mutation {
  pub fn perform(&self, result: &mut Vec<String>, input: &str) {
    if self.keep_original {
      result.push(input.to_owned());
    }

    match &self.action {
      Action::Prepend(s) => {
        for word in token_iterator(&tokenize_format_string(s)) {
          result.push(format!("{}{}", word.repeat(self.times), input))
        }
      },
      Action::Append(s) => {
        for word in token_iterator(&tokenize_format_string(s)) {
          result.push(format!("{}{}", input, word.repeat(self.times)))
        }
      },
      Action::Replace(s, b) => {
        if input.contains(s) || !self.keep_original { result.push(input.replace(s, b)) }
      },
      Action::RemoveFirstLetter => {
        let mut chrs = input.chars();
        for _ in 0..self.times { chrs.next(); }
        result.push(chrs.as_str().to_string())
      },
      Action::RemoveLastLetter => {
        let mut chrs = input.chars();
        for _ in 0..self.times { chrs.next_back(); }
        result.push(chrs.as_str().to_string())
      },
      Action::IfCharacterLength(not, ord, number) => {
        if (input.len().cmp(number) == *ord) != *not {
          result.push(input.to_owned())
        }
      },
      Action::IfContains(not, string) => {
        if input.contains(string) != *not {
          result.push(input.to_owned())
        }
      },
      Action::Reverse => result.push(input.chars().rev().collect()),
      Action::UppercaseAll => result.push(input.to_uppercase()),
      Action::LowercaseAll => result.push(input.to_lowercase()),
      Action::Clone => result.append(&mut vec![input.to_owned(), input.to_owned()]),
      Action::Wipe => result.push(String::new()),
      Action::Nothing => result.push(input.to_owned()),
      Action::Remove => (),
    }
  }
}

impl Display for Mutation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.times > 1 { write!(f, "{}x ", self.times )?; }

    match &self.action {
      Action::Prepend(s) => write!(f, "prepend: {}", s),
      Action::Append(s) => write!(f, "append: {}", s),
      Action::Replace(s, b) => write!(f, "replace: {} -> {}", s, b),
      Action::Reverse => write!(f, "reverse"),
      Action::RemoveFirstLetter => write!(f, "remove 1st letter"),
      Action::RemoveLastLetter => write!(f, "remove last letter"),
      Action::Clone => write!(f, "clone"),
      Action::Wipe => write!(f, "wipe"),
      Action::Nothing => write!(f, "nothing"),
      Action::UppercaseAll => write!(f, "uppercase all"),
      Action::LowercaseAll => write!(f, "lowercase all"),
      Action::Remove => write!(f, "remove"),

      Action::IfCharacterLength(not, ord, number) => write!(f, "if length {:?} {} = {}", ord, number, !not),
      Action::IfContains(not, string) => write!(f, "if contains {} = {}", string, !not)
    }?;

    if self.keep_original {
      write!(f, " (keeping original)")
    } else {
      write!(f, "")
    }
  }
} 

// pub fn build_mutation(action: Action, repeat: Option<u32>) -> mutation {
//   mutation { action }
// }

macro_rules! check_action_args {
  ($action:expr, $requiredArgs:expr, $actualArgs:expr) => {
    if $actualArgs > ($requiredArgs-1) {
      Ok($action)
    } else {
      Err(MutationBuildError::MissingArguments)
    }
  };
}

impl Action {
  pub fn from_string(action: &str, arguments: Vec<&str>, options: &str) -> Result<Action, MutationBuildError> {
    let argc = arguments.len();
  
    match action {
      "prepend" => {
        check_action_args!(Action::Prepend(arguments[0].to_owned()), 1, argc)
      },
      "append" => {
        check_action_args!(Action::Append(arguments[0].to_owned()), 1, argc)
      },
      "replace" => {
        check_action_args!(Action::Replace(arguments[0].to_owned(), arguments[1].to_owned()), 2, argc)
      }, 
      "if_length" => {
        check_action_args!({
          let arg_chrs: Vec<char> = arguments[0].chars().collect();
          let first_chr = arg_chrs.first().unwrap();
          
          let ordering = match first_chr {
            '>' => Ordering::Greater,
            '<' => Ordering::Less,
            '=' => Ordering::Equal,
            _ => {
              return Err(MutationBuildError::InvalidArgument(String::from("missing operator")))
            }
          };
  
          let mut number_chrs = arguments[0].chars();
          number_chrs.next();
          let number: usize = number_chrs.as_str().parse().unwrap();
  
          Action::IfCharacterLength(options.contains('!'), ordering, number)
        }, 1, argc)
      },
      "if_contains" => {
        check_action_args!(Action::IfContains(options.contains('!'), arguments[0].to_owned()), 1, argc)
      },
      "reverse" => Ok(Action::Reverse), 
      "clone" => Ok(Action::Clone), 
      "wipe" => Ok(Action::Wipe), 
      "nothing" => Ok(Action::Nothing), 
      "uppercase_all" => Ok(Action::UppercaseAll), 
      "lowercase_all" => Ok(Action::LowercaseAll), 
      "remove_last_letter" => Ok(Action::RemoveLastLetter), 
      "remove_first_letter" => Ok(Action::RemoveFirstLetter), 
      "remove" => Ok(Action::Remove), 
      _ => Err(MutationBuildError::ActionDoesNotExist),
    }
  }  
}

pub fn parse_mutation_string(mutation_strings: &Vec<String>) -> Vec<Mutation> {
  let mut mutations: Vec<Mutation> = vec![];

  for mutation_string in mutation_strings {
    let mut mutation_split: Vec<&str> = mutation_string
      .split(':')
      .into_iter()
      .map(|x| x.trim())
      .collect();
    let mut mutation_action = mutation_split[0].trim();
    let mut mutation_runtimes: usize = 1;
    let mut mutation_options: &str = "";
    
    if mutation_action.contains(' ') {
      let mutation_action_split: Vec<&str> = mutation_action
        .split(' ')
        .collect();
      if mutation_action_split.len() > 1 {
        mutation_action = mutation_action_split.last().unwrap();
        match mutation_action_split[0].parse() {
          Ok(f) => mutation_runtimes = f,
          Err(_) => {
            mutation_options = mutation_action_split[0]
          }
        }
      }
      if mutation_action_split.len() > 2 {
        mutation_options = mutation_action_split[1];
      }
    }

    mutation_split.remove(0);

    match Action::from_string(mutation_action, mutation_split, mutation_options) {
      Ok(m) => {
        mutations.push(Mutation { action: m, times: mutation_runtimes, keep_original: mutation_options.contains('k') })
      },
      Err(e) => println!("warning: couldn't build mutation {} ({:?})", mutation_action, e)
    }
  }

  mutations
}
