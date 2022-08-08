mod arguments;
mod mutation;
mod formatting;

use std::{
  fs::{File, self}, 
  io::{ BufReader, BufRead }
};

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

use clap::Parser;
use colored::Colorize;
use mutation::MutationSet;

use crate::{
  arguments::{ProgramArgs}, 
  mutation::{ parse_mutation_string }
};

fn main() {
  let args = ProgramArgs::parse();
  
  let mut mutation_sets = vec![ ];

  if args.mutation_string.len() > 1 {
    mutation_sets.push(
      MutationSet {
        mutations: parse_mutation_string(&args.mutation_string) 
      }
    )
  }

  let docs = YamlLoader::load_from_str(
    &fs::read_to_string(&args.mutations_file).unwrap()
  ).unwrap();

  let doc = &docs[0];

  println!("gorilla: loading {} yaml mutations from {}", 
    doc["name"].as_str().unwrap().purple(),
    args.mutations_file.purple()
  );

  for mutation_set in doc["mutation_sets"].as_vec().unwrap() {
    let mut mutation_strings: Vec<String> = vec![];
    for yaml_mut_string in mutation_set.as_vec().unwrap() {
      mutation_strings.push(yaml_mut_string.as_str().unwrap().to_string());
    }

    mutation_sets.push( 
      MutationSet {
        mutations: parse_mutation_string(&mutation_strings)
      } 
    ) 
  }

  println!("gorilla: mutation sets summary");
  for mutation_set in &mutation_sets {
    print!(" word");
    for mutation in &mutation_set.mutations {
      print!(" -> {}", mutation.to_string().blue());
    }
    println!()
  }

  let mut word_counter = 0;
  let mut mutation_counter = 0;

  if let Some(file_input) = args.file_input {
    println!("gorilla: using file {} as input", file_input.purple());

    let file_input = File::open(&file_input).unwrap();
    let reader = BufReader::new(file_input);
    let words_iter = reader.lines();
    
    for (_, l) in words_iter.enumerate() {
      word_counter += 1;
      let line = l.unwrap();
      for mutation_set in &mutation_sets {
        let mut result = mutation_set.perform(&line);

        if args.keep_original { result.push(line.clone()); } 

        for s in result {
          println!("{}", s);
          mutation_counter += 1
        }
      }
    }
  }

  println!("gorilla: {}. {} words -> {} words", "finished".green().bold() ,word_counter.to_string().red(), mutation_counter.to_string().green())

}
