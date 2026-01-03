mod arguments;
mod char_sets;
mod csv_parser;
mod formatting;
mod mutation;
mod patterns;
mod website_scraper;
mod yaml_parser;
mod threading;
mod tests;

use std::{
    fs::{self, File, OpenOptions}, io::{self, BufRead, BufReader, Write}, sync::mpsc, time::SystemTime
};

use clap::Parser;
use colored::Colorize;

use crate::{
    arguments::ProgramArgs, csv_parser::fmt_answers_from_csv, formatting::FormatFieldAnswer, mutation::{MutationSet, parse_mutation_string}, patterns::{calculate_sample_size_bytes, calculate_total_generations, token_iterator, tokenize_format_string}, threading::distribute_token_iter_work, website_scraper::{download_page, extract_words}, yaml_parser::{get_mutation_sets, parse_formatting_yaml}
};

struct Gorilla {
    program_args: ProgramArgs,
    mutation_sets: Vec<MutationSet>,
    file_save: Option<File>,
    mutation_counter: u32,
    word_counter: u32,
    pattern_threads: u128,
    start_time: SystemTime,
    output_separator: String,
}

impl Gorilla {
    fn run_mutations(&mut self, word: String) {
        self.word_counter += 1;

        for mutation_set in &self.mutation_sets {
            let mutation_result = mutation_set.perform(&word);

            if let Some(save_file) = &mut self.file_save {
                mutation_result.save_to_file(save_file)
            }

            for s in &mutation_result.mutated_words {
                self.mutation_counter += 1;

                if self.file_save.is_some() {
                    continue;
                }

                if self.program_args.timer {
                    eprint!(
                        "(in {:?}) ",
                        SystemTime::now()
                            .duration_since(self.start_time)
                            .expect("time may have gone backwards")
                    );
                }

                print!("{s}{}", self.output_separator)
            }
        }
    }
}

fn main() {
    let mut gorilla = Gorilla {
        program_args: ProgramArgs::parse(),
        mutation_sets: vec![],
        file_save: None,
        mutation_counter: 0,
        word_counter: 0,
        start_time: SystemTime::now(),
        output_separator: String::from('\n'),
        pattern_threads: 1
    };

    if gorilla.program_args.one_line {
        gorilla.output_separator = String::from(' ')
    }

    if let Some(pattern_threads) = gorilla.program_args.pattern_threads {
        gorilla.pattern_threads = pattern_threads;
    }

    if !gorilla.program_args.mutation_string.is_empty() {
        gorilla.mutation_sets.push(MutationSet {
            mutations: parse_mutation_string(&gorilla.program_args.mutation_string),
        })
    }

    if let Some(mutations_file) = &gorilla.program_args.mutations_file {
        let yaml_input = &fs::read_to_string(mutations_file).unwrap();
        gorilla
            .mutation_sets
            .append(&mut get_mutation_sets(yaml_input))
    }

    if gorilla.mutation_sets.is_empty() {
        eprintln!("gorilla: (warning) missing mutation sets");
        gorilla.mutation_sets.push(MutationSet::empty_set())
    } else {
        eprintln!("gorilla: mutation sets summary");
        for mutation_set in &gorilla.mutation_sets {
            eprint!(" {}", "word".dimmed());
            for mutation in &mutation_set.mutations {
                eprint!(" -> {}", mutation.to_string().blue());
            }
            eprintln!()
        }
    }

    if let Some(formatting_path) = &gorilla.program_args.from_formatting {
        let yaml_input = &fs::read_to_string(formatting_path)
            .expect("could not open file containing custom formats");
        let fmt_sets = parse_formatting_yaml(yaml_input);

        if let Some(csv_path) = &gorilla.program_args.csv {
            let answer_sets = fmt_answers_from_csv(csv_path);
            fmt_sets.check_answer_names(answer_sets.first().unwrap());

            for fmt_answers in answer_sets {
                for gen_word in fmt_sets.generate_words(fmt_answers) {
                    gorilla.run_mutations(gen_word);
                }
            }
        } else {
            let mut fmt_answers: Vec<FormatFieldAnswer> = Vec::new();

            for q in &fmt_sets.fields {
                let mut buffer = String::new();

                if let Some(question) = &q.question {
                    eprint!("(?) {}: ", question.blue())
                } else {
                    eprint!("(?) Fill in {}: ", q.name.blue())
                }
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut buffer).unwrap();

                fmt_answers.push(FormatFieldAnswer {
                    name: q.name.to_owned(),
                    answer: buffer.trim().to_owned(),
                })
            }

            // reset start time bcuz we dont want to time how much it took the user to
            // answer the questions
            gorilla.start_time = SystemTime::now();

            for gen_word in fmt_sets.generate_words(fmt_answers) {
                gorilla.run_mutations(gen_word);
            }
        }
    }

    // open file
    if let Some(file_save) = &gorilla.program_args.file_save {
        eprintln!("gorilla: using file {} as output", file_save.purple());
        gorilla.file_save = Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_save)
                .expect("Could not output file"),
        )
    }

    // file input
    if let Some(file_input) = &gorilla.program_args.file_input {
        eprintln!("gorilla: reading words from {}", file_input.purple());

        let file_input = File::open(file_input).unwrap();
        let reader = BufReader::new(file_input);
        let words_iter = reader.lines();

        for (_, l) in words_iter.enumerate() {
            let line = l.unwrap();
            gorilla.run_mutations(line);
        }
    }

    if let Some(pattern_input) = &gorilla.program_args.pattern_input {
        let tokens = tokenize_format_string(pattern_input);

        let total_words = calculate_total_generations(&tokens);
        let b_size = calculate_sample_size_bytes(&tokens);
        let mb_size = b_size / 1048576;
        let gb_size = b_size / 1073741824;
        let tb_size = b_size / 1099511627776;

        eprintln!(
            "gorilla: will generate {} words from a pattern {}",
            total_words,
            pattern_input.purple()
        );
        eprintln!("         sizes before mutations: {b_size} bytes / {mb_size} MB / {gb_size} GB / {tb_size} TB");


        let thread_iterators = distribute_token_iter_work(&tokens, gorilla.pattern_threads);

        for ac_toks in thread_iterators {
            for word in ac_toks {
                gorilla.run_mutations(word);
            }
        }

    }

    if let Some(website) = &gorilla.program_args.website_input {
        eprintln!(
            "gorilla: scraping words from a website {}",
            website.purple()
        );

        let page_contents = download_page(website).unwrap();
        let words = extract_words(&page_contents);

        for word in words {
            gorilla.run_mutations(word)
        }
    }

    if gorilla.program_args.one_line {
        println!()
    }

    let end_time = SystemTime::now();

    let runtime_dur = end_time
        .duration_since(gorilla.start_time)
        .expect("Clock may have gone backwards");

    eprintln!(
        "gorilla: {} in {runtime_dur:?}. {} words -> {} words",
        "finished".green().bold(),
        gorilla.word_counter.to_string().red(),
        gorilla.mutation_counter.to_string().green()
    );
}
