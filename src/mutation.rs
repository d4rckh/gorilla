use std::fmt::{Display, self};

#[derive(Debug)]
pub enum Action {
  Prepend(String),
  Append(String),
  Replace(String, String),
  Clone,
  Reverse,
  Wipe,
  Nothing
}

#[derive(Debug)]
pub enum MutationBuildError {
  ActionDoesNotExist
}

pub struct Mutation {
  pub action: Action,
  pub times: u32
}

impl Mutation {
  pub fn perform(&self, input: &str) -> Vec<String> {
    match &self.action {
      Action::Prepend(s) => vec![ format!("{}{}", s, input) ],
      Action::Append(s) => vec![ format!("{}{}", input, s) ],
      Action::Replace(s, b) => vec![ input.replace(s, b) ],
      Action::Reverse => vec![ input.chars().rev().collect() ],
      Action::Clone => vec![ input.to_owned(), input.to_owned() ],
      Action::Wipe => vec![ String::new() ],
      Action::Nothing => vec![ input.to_owned() ],
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
      Action::Clone => write!(f, "clone"),
      Action::Wipe => write!(f, "wipe"),
      Action::Nothing => write!(f, "nothing"),
    }
  }
} 

macro_rules! build_mutation {
  ($x:expr) => { Mutation { action: $x, times: 1 } };
  ($x:expr, $y:expr) => { Mutation { action: $x, times: $y } };
  () => { mutation { action: Action::Nothing, times: 1 } };
}

// pub fn build_mutation(action: Action, repeat: Option<u32>) -> mutation {
//   mutation { action }
// }

pub fn generate_mutation(action: &str, arguments: Vec<&str>, times: u32) -> Result<Mutation, MutationBuildError> {
  match action {
    "prepend" => Ok(build_mutation!(Action::Prepend(arguments[0].to_owned()), times)),
    "append" => Ok(build_mutation!(Action::Append(arguments[0].to_owned()), times)),
    "reverse" => Ok(build_mutation!(Action::Reverse, times)), 
    "clone" => Ok(build_mutation!(Action::Clone, times)), 
    "wipe" => Ok(build_mutation!(Action::Wipe, times)), 
    "replace" => Ok(build_mutation!(Action::Replace(arguments[0].to_owned(), arguments[1].to_owned()), times)), 
    "nothing" => Ok(build_mutation!(Action::Nothing, times)), 
    _ => Err(MutationBuildError::ActionDoesNotExist),
  }
}

pub fn mutate_word(mutations: &Vec<Mutation>, word: &str) -> Vec<String> {
  let mut result = vec![ word.to_string() ];

  for mutation in mutations {
    for _ in 0..mutation.times {
      let mut new_result: Vec<String> = vec![ ];
      for s in result {
          for s1 in mutation.perform(&s) {
              new_result.push(s1);
          }
      }
      result = new_result    
    }
  }

  result
}

pub fn mutate_words(mutations: &Vec<Mutation>, words: &Vec<String>) -> Vec<String> {
  let mut result: Vec<String> = vec![];
  for word in words {
    for res in mutate_word(mutations, &word) {
      result.push(res)
    }
  }
  return result
}

pub fn parse_mutation_string(mutation_strings: &Vec<String>) -> Vec<Mutation> {
  let mut mutations: Vec<Mutation> = vec![];

  for mutation_string in mutation_strings {
    let mut mutation_split: Vec<&str> = mutation_string
      .split(":")
      .into_iter()
      .map(|x| x.trim())
      .collect();
    let mut mutation_action = mutation_split[0].trim();
    let mut mutation_runtimes: u32 = 1;
    
    // check if user wants to run a mutation multiple times
    if mutation_action.contains(" ") {
      let mutation_action_split: Vec<&str> = mutation_action
        .split(" ")
        .collect();
      if mutation_action_split.len() > 1 {
        mutation_action = mutation_action_split[1];
        mutation_runtimes = mutation_action_split[0].parse().unwrap();
      }
    }

    mutation_split.remove(0);

    match generate_mutation(mutation_action, mutation_split, mutation_runtimes) {
      Ok(m) => mutations.push(m),
      Err(e) => println!("warning: couldn't build mutation {} ({:?})", mutation_action, e)
    }
  }

  return mutations
}