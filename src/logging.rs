use std::{fs, io::Write};
// use chrono::{prelude::*, Duration};

use colored::*;

// use crate::{FuzzResult, ProgramArgs};

// pub fn format_datetime(dt: DateTime<Local>, long: bool) -> String { 
//     if long {
//         return dt.format("%Y-%m-%d %H:%M:%S").to_string();
//     } else {
//         return dt.format("%H:%M:%S").to_string();    
//     }
// }

// pub fn format_duration(duration: Duration) -> String {
//     let seconds = duration.num_seconds();
//     let ms = duration.num_milliseconds();
//     if seconds < 2 {
//         return format!("{ms}ms")
//     } else {
//         return format!("{seconds}s")
//     }
// }

// impl FuzzResult {
//     pub fn print(&self) {
//         let status_string = String::from(self.status_code.as_str());

//         let mut status_display: ColoredString = status_string.green(); 

//         if self.status_code.is_client_error() || 
//             self.status_code.is_server_error() {
//             status_display = status_string.red()
//         } else if self.status_code.is_informational() ||
//             self.status_code.is_redirection() {
//             status_display = status_string.blue()
//         }
    
//         println!("({}) ({}) code {} size {}: {} ({})",
//             format_datetime(self.time, false).dimmed(),
//             format_duration(self.request_duration).dimmed(),
//             status_display.bold(),
//             self.body_length.to_string().cyan(),
//             self.url,
//             self.fuzz_word.purple().bold()
//         );
//     }

//     pub fn save(&self, file: &mut fs::File) {
//         let log_entry = format!("({}) {} {}\n", 
//             format_datetime(self.time, true),
//             self.status_code.as_u16(), 
//             self.fuzz_word
//         );
//         file.write_all(log_entry.as_bytes()).expect("write failed");
//     }
// }


// pub fn print_fuzz_result(prog_args: &ProgramArgs, fuzz_result: &FuzzResult) -> bool {
//     let status_string = String::from(fuzz_result.status_code.as_str());

//     if ( !prog_args.status_codes.contains(&status_string) && 
//         prog_args.status_codes.len() > 0 ) || (prog_args.exclude_status_codes.contains(&status_string)) 
//     { return false; }

//     fuzz_result.print();

//     true
// }

// pub fn print_args(prog_args: &ProgramArgs) {
//     print!("gorilla: fuzzing {} using {}, ", 
//         prog_args.url.blue(), 
//         prog_args.wordlist.blue());
//     if prog_args.status_codes.len() > 0 {
//         print!("only displaying requests that return {}",
//             prog_args.status_codes.join(", ").blue())
//     } else {
//         print!("displaying all requests")
//     }
//     if prog_args.exclude_status_codes.len() > 0 {
//         print!(", excluding {}",
//             prog_args.exclude_status_codes.join(", ").blue())
//     }
//     println!(".")
// }
