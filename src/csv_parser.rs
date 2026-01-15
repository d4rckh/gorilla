use colored::Colorize;
use std::io::Read;

use crate::formatting::FormatFieldAnswer;

pub fn fmt_answers_from_csv(csv_path: &str) -> Vec<Vec<FormatFieldAnswer>> {
    eprintln!("parsing {} csv file", csv_path.purple());
    let mut rdr = csv::Reader::from_path(csv_path).unwrap();
    parse_csv_reader(&mut rdr)
}

fn parse_csv_reader<R: Read>(rdr: &mut csv::Reader<R>) -> Vec<Vec<FormatFieldAnswer>> {
    let mut answers: Vec<Vec<FormatFieldAnswer>> = Vec::new();
    
    let headers: Vec<String> = rdr.headers().unwrap().iter().map(|h| h.to_owned()).collect();

    for result in rdr.records() {
        let row = result.unwrap();
        let mut row_answers: Vec<FormatFieldAnswer> = Vec::new();

        for (answer_no, answer) in row.iter().enumerate() {
            if answer_no < headers.len() {
                let name = headers[answer_no].to_owned();
                let answer = answer.to_owned();
                row_answers.push(FormatFieldAnswer { name, answer });
            }
        }
        answers.push(row_answers);
    }

    answers
}

#[cfg(test)]
mod tests {
    use super::parse_csv_reader;


    #[test]
    fn test_csv_parsing() {
        let csv_data = "name,age\njeff,20\nbob,30";
        let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
        
        let result = parse_csv_reader(&mut rdr);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0][0].name, "name");
        assert_eq!(result[0][0].answer, "jeff");
        assert_eq!(result[0][1].name, "age");
        assert_eq!(result[0][1].answer, "20");
    }
}
