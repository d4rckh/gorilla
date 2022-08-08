extern crate yaml_rust;
use yaml_rust::{YamlLoader};

use colored::Colorize;

use crate::{ 
  mutation::{ MutationSet, parse_mutation_string }
};

pub fn get_mutation_sets(yaml_input: &str) -> Vec<MutationSet> {
  let mut result: Vec<MutationSet> = vec![ ];

  let docs = YamlLoader::load_from_str(
    yaml_input
  ).unwrap();

  let doc = &docs[0];

  println!("gorilla: loading {} yaml mutations", 
    doc["name"].as_str().unwrap().purple()
  );

  for mutation_set in doc["mutation_sets"].as_vec().unwrap() {
    let mut mutation_strings: Vec<String> = vec![];
    for yaml_mut_string in mutation_set.as_vec().unwrap() {
      mutation_strings.push(yaml_mut_string.as_str().unwrap().to_string());
    }

    result.push( 
      MutationSet {
        mutations: parse_mutation_string(&mutation_strings)
      } 
    ) 
  }

  result
}