mod arguments;
mod mutation;
mod formatting;
mod yaml_parser;
mod tests;

use std::{
  fs::{File, self, OpenOptions}, 
  io::{ BufReader, BufRead }
};

use clap::Parser;
use colored::Colorize;
use mutation::MutationSet;

use crate::{
  arguments::{ProgramArgs}, 
  mutation::{ parse_mutation_string }, yaml_parser::get_mutation_sets
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

  if let Some(mutations_file) = args.mutations_file { 
    let yaml_input = &fs::read_to_string(&mutations_file).unwrap();
    mutation_sets.append(&mut get_mutation_sets(yaml_input))
  }

  println!("gorilla: mutation sets summary");
  for mutation_set in &mutation_sets {
    print!(" word");
    for mutation in &mutation_set.mutations {
      print!(" -> {}", mutation.to_string().blue());
    }
    println!()
  }

  let mut save_file_option = None;

  if let Some(file_save) = args.file_save {
    println!("gorilla: using file {} as output", file_save.purple());
    save_file_option = Some(OpenOptions::new()
      .append(true)
      .open(&file_save)
      .expect("Could not open file")
    )
  }

  let mut word_counter = 0;
  let mut mutation_counter = 0;

  if let Some(file_input) = args.file_input {
    println!("gorilla: reading words from {}", file_input.purple());

    let file_input = File::open(&file_input).unwrap();
    let reader = BufReader::new(file_input);
    let words_iter = reader.lines();
    
    for (_, l) in words_iter.enumerate() {
      word_counter += 1;
      let line = l.unwrap();
      for mutation_set in &mutation_sets {
        let mut result = mutation_set.perform(&line);

        if args.keep_original { result.mutated_words.push(line.clone()); } 

        if let Some(save_file) = &mut save_file_option {
          result.save_to_file(save_file)
        }

        for s in result.mutated_words {
          if save_file_option.is_none() { 
            println!("{}", s);
          }
          mutation_counter += 1
        }
      }
    }
  }

  println!("gorilla: {}. {} words -> {} words", 
    "finished".green().bold(),
    word_counter.to_string().red(), 
    mutation_counter.to_string().green()
  )

}
