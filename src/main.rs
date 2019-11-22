extern crate rand;

use std::error::Error;
use std::io;

use rand::thread_rng;

mod cli;
mod data;

use data::{QuizResult, QuestionType, AnswerMatch, VocabItem};


const VOCAB_FILE: &str = "vocab.csv";
const LABELS_FILE: &str = "labels.csv";
const LABEL_MAP_FILE: &str = "label_mapping.csv";

fn quiz(quiz_items: Vec<&VocabItem>, question_type: QuestionType) -> QuizResult {
    let mut correct = 0;
    let mut rng = thread_rng();

    for item in &quiz_items {
        let (question, answer) = get_qa(&question_type, item);
        println!("Q: {}", question);
        let mut attempt = String::new();
        io::stdin().read_line(&mut attempt).expect("Unable to get user input");
        let (score, message) = match answer_match(&attempt, &answer) {
            AnswerMatch::Perfect => (1, "Correct".to_string()),
            AnswerMatch::Partial => (1, format!("Correct, full answer is '{}'", answer)),
            AnswerMatch::Wrong => (0, format!("Wrong, correct answer was {}", answer))
        };
        correct += score;
        println!("{}", message);
    }

    QuizResult {
        correct,
        wrong: quiz_items.len() - correct,
    }
}

fn get_qa(question_type: &QuestionType, item: &VocabItem) -> (String, String) {
    let no_transliterated_msg = "No transliterated version in data, crashing";
    match question_type {
        // TODO: Work out why we have to clone
        QuestionType::NativeToForeign => (item.in_native_lang.clone(),
                                          item.transliterated.clone().expect(no_transliterated_msg)),
        QuestionType::ForeignToNative => (item.transliterated.clone().expect(no_transliterated_msg),
                                          item.in_native_lang.clone()),
        QuestionType::Bidirectional => {
            if rand::random() {
                (item.in_native_lang.clone(), item.transliterated.clone().expect(no_transliterated_msg))
            } else {
                (item.transliterated.clone().expect(no_transliterated_msg), item.in_native_lang.clone())
            }
        }
    }
}


fn answer_match(attempt: &str, answer: &str) -> AnswerMatch {
    let attempt = attempt.trim().to_uppercase();
    let answer = answer.trim().to_uppercase();
    if  attempt == answer {
        return AnswerMatch::Perfect;
    } else if let Some(_) = answer.find("/") {
        // Answers with multiple options are in form translation1/translation2/...
        let matching_answer = answer.split("/")
            .find(|&ans| ans == &attempt);
        if matching_answer.is_some() {
            return AnswerMatch::Partial;
        }
    } else if let Some(i) = answer.find("(") {
        if answer.split_at(i).0 == attempt {
            return AnswerMatch::Partial;
        }
    }

    AnswerMatch::Wrong
}

fn quiz_subprogram(quiz_type: QuestionType) {
    let vocab = data::Vocab::load(
        VOCAB_FILE,
        LABELS_FILE,
        LABEL_MAP_FILE);
    const NUM_QUESTIONS: usize = 15;
    let result = quiz(vocab.random_items(NUM_QUESTIONS),
                      quiz_type);
    println!("{:?}", result);
}

fn main() {
    let matches = cli::run_app();

    match matches.subcommand() {
        ("quiz", Some(args)) => {
            let quiz_type = match args.value_of("type").unwrap() {
                "ntf" => QuestionType::NativeToForeign,
                "ftn" => QuestionType::ForeignToNative,
                "both" => QuestionType::Bidirectional,
                _ => unreachable!(),
            };
            quiz_subprogram(quiz_type);
        },
        ("vocab", Some(args)) => println!("{:?}", args),
        _ => unreachable!()
    };
}
