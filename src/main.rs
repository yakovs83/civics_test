use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Question {
    question: String,
    answers: Vec<String>,
}

impl Question {
    fn new(q: String, a: Vec<String>) -> Question {
        Question {
            question: q,
            answers: a,
        }
    }
}

fn load_questions(filename: &Path) -> Vec<Question> {
    let is_question = Regex::new(r"^\d{1,3}\.").unwrap();
    let reader = File::open(filename).map(|f| BufReader::new(f)).unwrap();
    let mut result: Vec<Question> = Vec::new();
    let mut acc: Vec<String> = Vec::new();

    for item in reader.lines() {
        let line = item.unwrap();
        match (is_question.is_match(&line), acc.is_empty()) {
            (true, false) => {
                let ans: Vec<String> = acc.drain(1..).collect();
                result.push(Question::new(acc[0].clone(), ans));
                acc[0] = line;
            }
            (false, true) => panic!(
                "Something went wrong for line\n{}\nresult is {:?}\nacc is {:?}",
                line, result, acc
            ),
            _ => acc.push(line),
        }
    }
    result
}

fn main() {
    let filename = Path::new(".\\data\\questions.txt");
    let mut result = load_questions(&filename);
    // let is_question = Regex::new(r"^\d{1,3}\.").unwrap();
    // let reader = File::open(filename).map(|f| BufReader::new(f)).unwrap();
    // let mut result: Vec<Question> = Vec::new();
    // let mut acc: Vec<String> = Vec::new();

    // for item in reader.lines() {
    //     let line = item.unwrap();
    //     match (is_question.is_match(&line), acc.is_empty()) {
    //         (true, false) => {
    //             let ans: Vec<String> = acc.drain(1..).collect();
    //             result.push(Question::new(acc[0].clone(), ans));
    //             acc[0] = line;
    //         }
    //         (false, true) => panic!(
    //             "Something went wrong for line\n{}\nresult is {:?}\nacc is {:?}",
    //             line, result, acc
    //         ),
    //         _ => acc.push(line),
    //     }
    // }
    let mut rng = thread_rng();
    result.shuffle(&mut rng);
    result.iter().for_each(|r| println!("{:?}", r));
}
