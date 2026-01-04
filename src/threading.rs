use std::{
    fs::File,
    io::{self, stdout, BufWriter},
    thread::{self, JoinHandle},
    time::SystemTime,
    vec,
};

use crossbeam_channel::Sender;
use colored::Colorize;

use crate::{mutation::MutationSet, patterns::{
    Token, TokenIter, calculate_total_generations, token_iterator_from_start_end
}};

pub fn distribute_token_iter_work(tokens: &Vec<Token>, threads_n: u128) -> Vec<TokenIter> {
    let mut result: Vec<TokenIter> = vec![];

    let total_generations = calculate_total_generations(tokens);

    let base = total_generations / threads_n;
    let rem = total_generations % threads_n;

    let mut start_i = 0;

    for thread_i in 0..threads_n {
        let end_i = start_i + base + (if thread_i < rem { 1 } else { 0 });

        result.push(token_iterator_from_start_end(tokens, start_i, end_i));

        start_i = end_i;
    }

    result
}

pub struct PrinterStats {
    saved_words: u128,
}

pub fn run_mutations(sets: &Vec<MutationSet>, word: &String, sender: Sender<String>) {
    for mutation_set in sets {
        let mutation_result = mutation_set.perform(word);

        for s in mutation_result.mutated_words {
            let _ = sender.send(s);
        }
    }
}

pub fn printer_thread(
    timer: bool,
    start_time: SystemTime,
    output_separator: String,
    file_save_path: Option<String>,
) -> (Sender<String>, JoinHandle<()>) {
    let (tx, rx) = crossbeam_channel::bounded::<String>(1000);

    let handle = thread::spawn(move || {
        let mut printer_stats = PrinterStats { saved_words: 0 };

        let mut writer: Box<dyn io::Write> = if let Some(path) = file_save_path {
            let file = File::create(path).expect("Unable to create file");
            Box::new(BufWriter::new(file))
        } else {
            Box::new(BufWriter::new(stdout()))
        };

        for mutated_word in rx {
            printer_stats.saved_words += 1;

            if timer {
                let elapsed = SystemTime::now()
                    .duration_since(start_time)
                    .unwrap_or_default();
                eprint!("(in {:?}) ", elapsed);
            }

            // write! macro works for any type implementing std::io::Write
            if let Err(e) = write!(writer, "{}{}", mutated_word, output_separator) {
                eprintln!("Error writing: {}", e);
                break;
            }
        }

        // Ensure the last bits are written to disk
        let _ = writer.flush();

        let end_time = SystemTime::now();

        let runtime_dur = end_time
            .duration_since(start_time)
            .expect("Clock may have gone backwards");

        eprintln!(
            "gorilla: {} in {runtime_dur:?}. total {} words",
            "finished".green().bold(),
            printer_stats.saved_words
        );
    });

    (tx, handle)
}
