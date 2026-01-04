mod arguments;
mod char_sets;
mod csv_parser;
mod formatting;
mod mutation;
mod patterns;
mod tests;
mod threading;
mod website_scraper;
mod yaml_parser;

use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use crossbeam_channel::Sender;
use std::time::SystemTime;

use clap::Parser;
use colored::Colorize;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::threading::run_mutations;
use crate::{
    arguments::ProgramArgs,
    csv_parser::fmt_answers_from_csv,
    formatting::FormatFieldAnswer,
    mutation::{parse_mutation_string, MutationSet},
    patterns::{calculate_sample_size_bytes, calculate_total_generations, tokenize_format_string},
    threading::distribute_token_iter_work,
    website_scraper::{download_page, extract_words},
    yaml_parser::{get_mutation_sets, parse_formatting_yaml},
};

struct Gorilla {
    program_args: ProgramArgs,
    mutation_sets: Vec<MutationSet>,
    pattern_threads: u128,
    start_time: SystemTime,
    output_separator: String,
    sender: Option<Sender<String>>,
}

fn main() {
    let mut gorilla = Gorilla {
        program_args: ProgramArgs::parse(),
        mutation_sets: vec![],
        start_time: SystemTime::now(),
        output_separator: String::from('\n'),
        pattern_threads: 1,
        sender: None,
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

    let (tx, printer_handle) = threading::printer_thread(
        gorilla.program_args.timer,
        gorilla.start_time,
        gorilla.output_separator.clone(),
        gorilla.program_args.file_save.clone(),
    );

    gorilla.sender = Some(tx);

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
                    run_mutations(
                        &gorilla.mutation_sets,
                        &gen_word,
                        gorilla.sender.clone().unwrap(),
                    );
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
                run_mutations(
                    &gorilla.mutation_sets,
                    &gen_word,
                    gorilla.sender.clone().unwrap(),
                );
            }
        }
    }

    // file input
    if let Some(file_input) = &gorilla.program_args.file_input {
        eprintln!("gorilla: reading words from {}", file_input.purple());

        let file_input = File::open(file_input).unwrap();
        let reader = BufReader::new(file_input);

        reader
            .lines()
            .map_while(Result::ok)
            .par_bridge()
            .for_each(|l| {
                run_mutations(
                    &gorilla.mutation_sets,
                    &l,
                    gorilla.sender.clone().unwrap(),
                );
            });
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
        eprintln!(
            "         sizes before mutations: {} bytes / {} MB / {} GB / {} TB",
            b_size.to_string().red(),
            mb_size,
            gb_size,
            tb_size
        );
        eprintln!(
            "         --pattern-threads {} {}",
            gorilla.pattern_threads.to_string().green(),
            "(total pattern threads)".to_string().dimmed()
        );

        let thread_iterators = distribute_token_iter_work(&tokens, gorilla.pattern_threads);

        let mut handles = vec![];

        for token_iter in thread_iterators {
            let mutation_sets = gorilla.mutation_sets.clone();
            let tx = gorilla.sender.clone().unwrap();

            let handle = std::thread::spawn(move || {
                // Because 'iter' is owned and 'mut', we can use it as an Iterator
                for word in token_iter {
                    run_mutations(&mutation_sets, &word, tx.clone());
                }
            });
            handles.push(handle);
        }

        // Wait for pattern generation to finish before moving on
        for handle in handles {
            let _ = handle.join();
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
            run_mutations(
                &gorilla.mutation_sets,
                &word,
                gorilla.sender.clone().unwrap(),
            );
        }
    }

    if gorilla.program_args.one_line {
        println!()
    }

    drop(gorilla.sender.take());

    let _ = printer_handle.join();
}
