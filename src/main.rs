mod arguments;
mod mutation;

use std::{
  fs::File, 
  io::{ BufReader, BufRead }
};

use clap::Parser;
use colored::Colorize;
use mutation::MutationSet;

use crate::{
  arguments::{ProgramArgs}, 
  mutation::{ parse_mutation_string }
};

fn main() {
  let args = ProgramArgs::parse();
  
  let mutation_set = MutationSet {
    mutations: parse_mutation_string(&args.mutation_string) 
  };

  if mutation_set.mutations.len() < 1  {
    println!("warning: mutation set empty");
  } else {
    for mutation in &mutation_set.mutations {
      println!("  - {}", mutation.to_string().blue());
    }
  }

  if let Some(file_input) = args.file_input {
    println!("gorilla: using file {} as input", file_input.purple().bold());

    let file_input = File::open(&file_input).unwrap();
    let reader = BufReader::new(file_input);
    let words_iter = reader.lines();
    
    for (_, l) in words_iter.enumerate() {
      let line = l.unwrap();
      let mut result = mutation_set.perform(&line);

      if args.keep_original { result.push(line.clone()); } 

      for s in result {
        println!("{}", s);
      }
    }
  }

}
