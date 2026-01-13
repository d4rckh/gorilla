use std::{
    fs::File,
    io::{self, BufWriter},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::SystemTime,
    vec,
};

use colored::Colorize;
use crossbeam_channel::Sender;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

use crate::{
    mutation::MutationSet,
    pattern::{Token, TokenIter, calculate_total_generations, token_iterator_from_start_end},
};

pub fn distribute_token_iter_work(tokens: &[Token], threads_n: u128) -> Vec<TokenIter> {
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

pub fn run_mutations(sets: &Vec<MutationSet>, word: &str, sender: Sender<String>) {
    for mutation_set in sets {
        let mutation_result = mutation_set.perform(word);

        for s in mutation_result.mutated_words {
            let _ = sender.send(s);
        }
    }
}

pub fn printer_thread(
    no_progress_bar: bool,
    start_time: SystemTime,
    output_separator: String,
    total_words: Arc<Mutex<usize>>, // We need to read this to set the bar length
    file_save_path: Option<String>,
) -> (Sender<String>, Vec<JoinHandle<()>>) {
    let (tx, rx) = crossbeam_channel::bounded::<String>(100);

    let printer_handle = thread::spawn(move || {
        let mut printer_stats = PrinterStats { saved_words: 0 };

        let mut writer: Box<dyn io::Write> = if let Some(path) = file_save_path {
            let file = File::create(path).expect("Unable to create file");
            Box::new(BufWriter::new(file))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        };

        let total_count = *total_words.lock().unwrap();

        let pb = ProgressBar::new(total_count as u64);

        // {spinner} = animated spinner
        // {bar:40.cyan/blue} = a 40-char wide bar colored cyan/blue
        // {pos}/{len} = current/total
        // {eta} = estimated time remaining
        pb.set_style(ProgressStyle::default_bar()
            .template("gorilla: (wrk) {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#> "));

        if no_progress_bar {
            pb.set_draw_target(ProgressDrawTarget::hidden());
        } else {
            pb.set_draw_target(ProgressDrawTarget::stderr());
        }

        for mutated_word in rx {
            printer_stats.saved_words += 1;

            pb.inc(1);

            // pb.set_length(*total_words.lock().unwrap() as u64);

            if let Err(e) = write!(writer, "{}{}", mutated_word, output_separator) {
                pb.suspend(|| {
                    crate::logging::error(&format!("error writing: {}", e));
                });
                break;
            }
        }

        // 5. Cleanup
        pb.finish_with_message("Done writing");
        let _ = writer.flush();

        let end_time = SystemTime::now();
        let runtime_dur = end_time.duration_since(start_time).unwrap_or_default();

        // You might not need this anymore since the Progress Bar shows time,
        // but kept it as per your original logic:
        crate::logging::success(&format!(
            "{} in {runtime_dur:?}. total {} words",
            "finished".green().bold(),
            printer_stats.saved_words
        ));
    });

    (tx, vec![printer_handle])
}
