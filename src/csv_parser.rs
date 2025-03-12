use colored::Colorize;

use crate::formatting::FormatFieldAnswer;

pub fn fmt_answers_from_csv(csv_path: &str) -> Vec<Vec<FormatFieldAnswer>> {
    let mut answers: Vec<Vec<FormatFieldAnswer>> = Vec::new();

    let mut rdr = csv::Reader::from_path(csv_path).unwrap();

    eprintln!("gorilla: parsing {} csv file", csv_path.purple());

    let mut headers: Vec<String> = Vec::new();

    for header in rdr.headers().unwrap() {
        headers.push(header.to_owned())
    }

    for result in rdr.records() {
        let row = result.unwrap();

        let mut row_answers: Vec<FormatFieldAnswer> = Vec::new();
        let mut column_no = 0;

        for answer in row.iter() {
            let name = headers[column_no].to_owned();
            let answer = answer.to_owned();
            row_answers.push(FormatFieldAnswer { name, answer });
            column_no += 1
        }

        answers.push(row_answers);
    }

    answers
}
