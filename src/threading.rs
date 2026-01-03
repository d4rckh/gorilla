use std::{
    fmt::Write, fs::File, io::{self, BufWriter, stdout}, sync::{
        Arc, atomic::{AtomicBool, Ordering}, mpsc::{self, Sender}
    }, thread::{self, JoinHandle}, time::SystemTime, vec
};

use crate::{
    patterns::{calculate_total_generations, token_iterator_from_start_end, Token, TokenIter},
};

pub fn distribute_token_iter_work(tokens: &Vec<Token>, threads_n: u128) -> Vec<TokenIter> {
    let mut result: Vec<TokenIter> = vec![];

    let total_generations = calculate_total_generations(&tokens);

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
    saved_words: u128
}

pub fn printer_thread(
    timer: bool,
    start_time: SystemTime,
    output_separator: String,
    file_save_path: Option<String>,
) -> (Sender<String>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::spawn(move || {
        let mut writer: Box<dyn io::Write> = if let Some(path) = file_save_path {
            let file = File::create(path).expect("Unable to create file");
            Box::new(BufWriter::new(file))
        } else {
            Box::new(BufWriter::new(stdout()))
        };

        for mutated_word in rx {
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
    });

    (tx, handle)
}