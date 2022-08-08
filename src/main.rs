mod arguments;
mod mutation;

use std::{
  fs::File, 
  io::{ BufReader, BufRead }
};

use clap::Parser;
use colored::Colorize;

use crate::{
  arguments::{ProgramArgs}, 
  mutation::{ parse_mutation_string, mutate_word, mutate_words }
};

fn main() {
  let args = ProgramArgs::parse();
  
  let mutations = parse_mutation_string(&args.mutation_string);
  let mutations1 = parse_mutation_string(&args.mutation1_string);
  let mutations2 = parse_mutation_string(&args.mutation2_string);

  if mutations.len() < 1 && mutations1.len() < 1 && mutations2.len() < 1  {
    println!("warning: no mutations specified");
  }
  
  if mutations.len() > 0 {
    println!("gorilla: default mutation set summary");
    for mutation in &mutations {
      println!("    | {}", mutation.to_string().blue());
    }
  }

  if mutations1.len() > 0 {
    println!("gorilla: first mutation set summary");
    for mutation in &mutations1 {
      println!("    | {}", mutation.to_string().blue());
    }
  }

  if mutations2.len() > 0 {
    println!("gorilla: second mutation set summary");
    for mutation in &mutations2 {
      println!("    | {}", mutation.to_string().blue());
    }
  }

  if let Some(file_input) = args.file_input {
    println!("gorilla: using file {} as input", file_input.purple().bold());

    let file_input = File::open(&file_input).unwrap();
    let reader = BufReader::new(file_input);
    let words_iter = reader.lines();
    
    for (i, l) in words_iter.enumerate() {
      let line = l.unwrap();
      let default_result = mutate_word(&mutations, &line);
      let result = mutate_words(&mutations1, &default_result);

      if mutations2.len() > 0 {
        let result2 = mutate_words(&mutations2, &default_result);
        for s in result2 {
          println!("{}. {} -> {}", i, line, s);
        }
  
      }

      for s in result {
        println!("{}. {} -> {}", i, line, s);
      }
    }
  }

}
