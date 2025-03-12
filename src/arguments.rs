use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "a wordlist generator",
    long_about = "if you want to contribute to this project, check out the github repo: https://github.com/d4rckh/gorilla"
)]
pub struct ProgramArgs {
    #[clap(short = 'i', long = "from-file", help = "Specify the input file")]
    pub file_input: Option<String>,

    #[clap(short = 'l', long = "one-line", help = "Print the output on one line")]
    pub one_line: bool,

    #[clap(
        short = 't',
        long = "timer",
        help = "Show the amount of time it took to mutate/compute a word"
    )]
    pub timer: bool,

    #[clap(
        short = 'p',
        long = "from-pattern",
        help = "Generate words from a pattern"
    )]
    pub pattern_input: Option<String>,

    #[clap(
        short = 'q',
        long = "from-formatting",
        help = "Generate words from custom formatting applied"
    )]
    pub from_formatting: Option<String>,

    #[clap(
        short = 'c',
        long = "with-csv",
        help = "Use a CSV as input for formatting fields"
    )]
    pub csv: Option<String>,

    #[clap(
        short = 'w',
        long = "from-website",
        help = "Spider a website and generate a wordlist from it's page contents"
    )]
    pub website_input: Option<String>,

    #[clap(
        short = 'o',
        long = "output-file",
        help = "Specify the file in which the results will be saved"
    )]
    pub file_save: Option<String>,

    #[clap(
        short = 'm',
        long = "mutation",
        help = "Specify a way to mutate the words. Format is action:param1:param2"
    )]
    pub mutation_string: Vec<String>,

    #[clap(
        short = 'f',
        long = "mutations-file",
        help = "Specify a path to a yaml file"
    )]
    pub mutations_file: Option<String>,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // does testing things
    // list_mutations {    },
}
