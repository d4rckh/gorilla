extern crate yaml_rust;
use yaml_rust::{YamlLoader};

use colored::Colorize;

use crate::{ 
  mutation::{ MutationSet, parse_mutation_string }, formatting::{FormattingSets, FormatField, FormatSet, FormatPart}
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

pub fn parse_formatting_yaml(yaml_input: &str) -> FormattingSets {
  let docs = YamlLoader::load_from_str(
    yaml_input
  ).unwrap();

  let doc = &docs[0];
  let formatting_name = doc["name"].as_str().unwrap();

  println!("gorilla: loading {} formatting sets", 
    formatting_name.purple()
  );

  let mut format_sets: Vec<FormatSet> = Vec::new();
  let mut format_fields: Vec<FormatField> = Vec::new();

  for yaml_format_field in doc["fields"].as_vec().unwrap() {
    let fields = yaml_format_field.as_vec().unwrap();
    let mut question: Option<String> = None;

    if fields.len() > 1 {
      question = fields[1].as_str().map(str::to_owned)
    }

    format_fields.push(
      FormatField {
        name: fields[0].as_str().unwrap().to_owned(),
        question
      }
    )
  }

  for yaml_format_set in doc["formatting_sets"].as_vec().unwrap() {
    
    let mut format_set = FormatSet::new();

    for fmt_part in yaml_format_set.as_vec().unwrap() {
      if let Some(fmt_str) = fmt_part.as_str() {
        format_set.parts.push(
          FormatPart {
            text: fmt_str.to_owned(),
            mutations: MutationSet::empty_set()
          }
        )
      }      
      if let Some(fmt_str) = fmt_part.as_vec() {
        let txt = fmt_str[0].as_str().unwrap();

        let mut mutation_strings = Vec::new();

        if let Some(yaml_mut_vec) = fmt_str[1].as_vec() { 
          for yaml_mut_str in yaml_mut_vec {
            if let Some(fmt_str) = yaml_mut_str.as_str() {
              mutation_strings.push(fmt_str.to_owned())
            }
          }
        }

        if let Some(yaml_mut_str) = fmt_str[1].as_str() { 
          mutation_strings.push(yaml_mut_str.to_owned())
        }

        format_set.parts.push(
          FormatPart {
            text: txt.to_owned(),
            mutations: MutationSet { mutations: parse_mutation_string(&mutation_strings) }
          }
        )
      }
    }
  
    format_sets.push(format_set)
  }

  FormattingSets {
    name: formatting_name.to_owned(),
    fields: format_fields,
    sets: format_sets
  }
}