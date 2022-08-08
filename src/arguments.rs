use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    author="d4rckh", 
    version="v1.0", 
    about="a wordlist generator", 
    long_about="if you want to contribute to this project, check out the github repo: https://github.com/d4rckh/gorilla",
)]
pub struct ProgramArgs {
  #[clap (
    short='i',
    long="input",
    help="Specify the input file"
  )]
  pub file_input: Option<String>,

  #[clap(
      short='o', 
      long="output", 
      help="Specify the file in which the results will be saved"
  )]
  pub file_save: Option<String>,

  #[clap(
    short='m',
    long="mutation",
    help="Specify a way to mutate the words. Format is action:param1,param2"
  )]
  pub mutation_string: Vec<String>,

  #[clap(
    short='f',
    long="mutations-file",
    help="Specify a path to a yaml file"
  )]
  pub mutations_file: String,

  #[clap(
    short='k',
    long="keep-original",
    help="Keep the original word"
  )]
  pub keep_original: bool,

  #[clap(subcommand)]
  pub command: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // does testing things
    // list_mutations {    },
}
