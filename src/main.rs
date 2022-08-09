mod arguments;
mod mutation;
mod formatting;
mod yaml_parser;
mod website_scraper;

mod tests;

use std::{
  fs::{File, self, OpenOptions}, 
  io::{BufReader, BufRead}
};

use clap::Parser;
use colored::Colorize;

use crate::{
  arguments::ProgramArgs, 
  mutation::{parse_mutation_string, MutationSet}, 
  yaml_parser::get_mutation_sets, 
  formatting::{tokenize_format_string, execute_format_string},
  website_scraper::{download_page, extract_words}
};

struct Gorilla {
  program_args: ProgramArgs,
  mutation_sets: Vec<MutationSet>,
  file_save: Option<File>,
  mutation_counter: u32,
  word_counter: u32
}

impl Gorilla {
  fn mutate_word(&mut self, word: String) {
    self.word_counter += 1;
    
    for mutation_set in &self.mutation_sets {
      let mut result = mutation_set.perform(&word);

      if self.program_args.keep_original { result.mutated_words.push(word.clone()); } 

      if let Some(save_file) = &mut self.file_save {
        result.save_to_file(save_file)
      }

      for s in result.mutated_words {
        if self.file_save.is_none() { 
          println!("{}", s);
        }
        self.mutation_counter += 1
      }
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

  let mut gorilla = Gorilla {
    program_args: ProgramArgs::parse(),
    mutation_sets: vec![ ],
    file_save: None,
    mutation_counter: 0,
    word_counter: 0
  };
  
  if gorilla.program_args.mutation_string.len() > 0 {
    gorilla.mutation_sets.push(
      MutationSet {
        mutations: parse_mutation_string(&gorilla.program_args.mutation_string) 
      }
    )
  }

  if let Some(mutations_file) = &gorilla.program_args.mutations_file { 
    let yaml_input = &fs::read_to_string(mutations_file).unwrap();
    gorilla.mutation_sets.append(&mut get_mutation_sets(yaml_input))
  }

  println!("gorilla: mutation sets summary");
  for mutation_set in &gorilla.mutation_sets {
    print!(" word");
    for mutation in &mutation_set.mutations {
      print!(" -> {}", mutation.to_string().blue());
    }
    println!()
  }

  if let Some(file_save) = &gorilla.program_args.file_save {
    println!("gorilla: using file {} as output", file_save.purple());
    gorilla.file_save = Some(OpenOptions::new()
      .append(true)
      .open(&file_save)
      .expect("Could not open file")
    )
  }

  if let Some(file_input) = &gorilla.program_args.file_input {
    println!("gorilla: reading words from {}", file_input.purple());

    let file_input = File::open(file_input).unwrap();
    let reader = BufReader::new(file_input);
    let words_iter = reader.lines();
    
    for (_, l) in words_iter.enumerate() {
      let line = l.unwrap();
      gorilla.mutate_word(line);
    }
  }

  if let Some(pattern_input) = &gorilla.program_args.pattern_input {
    println!("gorilla: generating words from a pattern {}", pattern_input.purple());
    let tokens = tokenize_format_string(pattern_input);
    let result = execute_format_string(&tokens);
    
    for word in result {
      gorilla.mutate_word(word)
    }
  }

  if let Some(website) = &gorilla.program_args.website_input {
    println!("gorilla: scraping words from a website {}", website.purple());
    let page_contents = download_page(website).await?;
    let words = extract_words(&page_contents);

    for word in words {
      gorilla.mutate_word(word)
    }
  }

  println!("gorilla: {}. {} words -> {} words", 
    "finished".green().bold(),
    gorilla.word_counter.to_string().red(), 
    gorilla.mutation_counter.to_string().green()
  );

  Ok(())
}
