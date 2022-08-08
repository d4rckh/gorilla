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
  pub times: u32,
  pub keep_original: bool
}

impl Mutation {
  pub fn perform(&self, input: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    if self.keep_original {
      result.push(input.to_owned());
    }

    match &self.action {
      Action::Prepend(s) => result.push(format!("{}{}", s, input)),
      Action::Append(s) => result.push(format!("{}{}", input, s)),
      Action::Replace(s, b) => {
        if input.contains(s) || !self.keep_original { result.push(input.replace(s, b)) }
      },
      Action::Reverse => result.push(input.chars().rev().collect()),
      Action::Clone => result.append(&mut vec![input.to_owned(), input.to_owned()]),
      Action::Wipe => result.push(String::new()),
      Action::Nothing => result.push(input.to_owned()),
    }

    return result
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

pub fn build_action(action: &str, arguments: Vec<&str>) -> Result<Action, MutationBuildError> {
  match action {
    "prepend" => Ok(Action::Prepend(arguments[0].to_owned())),
    "append" => Ok(Action::Append(arguments[0].to_owned())),
    "replace" => Ok(Action::Replace(arguments[0].to_owned(), arguments[1].to_owned())), 
    "reverse" => Ok(Action::Reverse), 
    "clone" => Ok(Action::Clone), 
    "wipe" => Ok(Action::Wipe), 
    "nothing" => Ok(Action::Nothing), 
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
    let mut mutation_options: &str = "";
    
    // check if user wants to run a mutation multiple times
    if mutation_action.contains(" ") {
      let mutation_action_split: Vec<&str> = mutation_action
        .split(" ")
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

    match build_action(mutation_action, mutation_split) {
      Ok(m) => {
        mutations.push(Mutation { action: m, times: mutation_runtimes, keep_original: mutation_options.contains("k") })
      },
      Err(e) => println!("warning: couldn't build mutation {} ({:?})", mutation_action, e)
    }
  }

  return mutations
}