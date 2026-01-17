use std::{
    fs::File,
    io::{self, BufWriter},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread::{self, JoinHandle},
    time::SystemTime,
    vec,
};

use colored::Colorize;
use crossbeam_channel::{Receiver, Sender};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

use crate::{
    logging,
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

pub fn printer_blocking(
    no_progress_bar: bool,
    start_time: SystemTime,
    output_separator: String,
    total_words: Arc<AtomicUsize>,
    file_save_path: Option<String>,
    show_header: bool,
    rx: Receiver<String>,
) {
    let mut printer_stats = PrinterStats { saved_words: 0 };

    let mut writer: Box<dyn io::Write> = if let Some(path) = file_save_path {
        let file = File::create(path).expect("Unable to create file");
        Box::new(BufWriter::new(file))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // Initial load
    let total_count = total_words.load(Ordering::Relaxed);

    let pb = ProgressBar::new(total_count as u64);

    // {spinner} = animated spinner
    // {bar:40.cyan/blue} = a 40-char wide bar colored cyan/blue
    // {pos}/{len} = current/total
    // {eta} = estimated time remaining
    pb.set_style(
            ProgressStyle::default_bar()
                .template("gorilla: (wrk) {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} (eta: {eta}) {msg}")
                .unwrap()
                .progress_chars("#> "),
        );

    if no_progress_bar {
        pb.set_draw_target(ProgressDrawTarget::hidden());
    } else {
        pb.set_draw_target(ProgressDrawTarget::stderr());
    }

    for mutated_word in rx {
        if show_header && printer_stats.saved_words < 5 {
            pb.suspend(|| {
                logging::info(&format!(
                    "(gen #{}) {}",
                    printer_stats.saved_words, mutated_word
                ));
            })
        }

        printer_stats.saved_words += 1;

        pb.inc(1);

        // Check for updates to total_words
        let current_total = total_words.load(Ordering::Relaxed) as u64;
        if current_total != pb.length().unwrap_or(0) {
            pb.set_length(current_total);
        }

        if let Err(e) = write!(writer, "{}{}", mutated_word, output_separator) {
            pb.suspend(|| {
                crate::logging::error(&format!("error writing: {}", e));
            });
            break;
        }
    }

    pb.finish_with_message("Done");
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
}

pub fn create_printer_channel() -> (Sender<String>, Receiver<String>) {
    crossbeam_channel::bounded::<String>(100)
}

pub fn printer_thread(
    no_progress_bar: bool,
    start_time: SystemTime,
    output_separator: String,
    total_words: Arc<AtomicUsize>,
    file_save_path: Option<String>,
    show_header: bool,
    rx: Receiver<String>,
) -> Vec<JoinHandle<()>> {
    let printer_handle = thread::spawn(move || {
        printer_blocking(
            no_progress_bar,
            start_time,
            output_separator,
            total_words,
            file_save_path,
            show_header,
            rx,
        );
    });

    vec![printer_handle]
}

#[cfg(test)]
mod tests {
    use super::distribute_token_iter_work;
    use crate::pattern::tokenize_format_string;

    #[test]
    fn token_iter_work_distribution() {
        let tokens = tokenize_format_string("{0-9}");
        let token_iters = distribute_token_iter_work(&tokens, 10);

        for thread_i in 0..10 {
            assert_eq!(token_iters[thread_i].current_index, thread_i as u128);
            assert_eq!(token_iters[thread_i].end_index, (thread_i + 1) as u128);
        }
    }

    #[test]
    fn token_iter_work_distribution_with_remaining() {
        let tokens = tokenize_format_string("{0-9}");
        let token_iters = distribute_token_iter_work(&tokens, 3);

        assert_eq!(token_iters[0].current_index, 0);
        assert_eq!(token_iters[0].end_index, 4);

        for thread_i in 1..3 {
            assert_eq!(
                token_iters[thread_i].current_index,
                (thread_i * 3 + 1) as u128
            );
            assert_eq!(token_iters[thread_i].end_index, (thread_i * 3 + 4) as u128);
        }
    }
}
