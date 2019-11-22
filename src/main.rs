extern crate rand;
extern crate clap;

use std::io;

mod cli;
mod data;

use data::{QuizResult, QuestionType, AnswerMatch, VocabItem};

const VOCAB_FILE: &str = "vocab.csv";
const LABELS_FILE: &str = "labels.csv";
const LABEL_MAP_FILE: &str = "label_mapping.csv";

fn main() {
    let matches = cli::run_app();

    match matches.subcommand() {
        (cli::QUIZ, Some(args)) => {
            let quiz_type = match args.value_of("type").unwrap() {
                "ntf" => QuestionType::NativeToForeign,
                "ftn" => QuestionType::ForeignToNative,
                "both" => QuestionType::Bidirectional,
                _ => unreachable!(),
            };
            quiz_subprogram(quiz_type, args.value_of("num-questions").unwrap().parse().unwrap());
        },
        (cli::VOCAB, Some(args)) => vocab_subprogram(args),
        _ => unreachable!()
    };
}

fn quiz_subprogram(quiz_type: QuestionType, num_questions: usize) {
    let vocab = load_vocab();
    let result = quiz(vocab.random_items(num_questions),
                      quiz_type);
    println!("{:?}", result);
}

fn quiz(quiz_items: Vec<&VocabItem>, question_type: QuestionType) -> QuizResult {
    let mut correct = 0;

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
fn vocab_subprogram(args: &clap::ArgMatches) {
    match args.subcommand() {
        ("add", Some(args)) => {
            let parse_arg = |arg| args.value_of(arg).unwrap();
            add_vocab_item(parse_arg("native"), parse_arg("transliterated"));
        },
        _ => unreachable!(),
    };
}

fn add_vocab_item(native: &str, transliterated: &str) {
    println!("{} = {}", native, transliterated);
}


fn load_vocab() -> data::Vocab {
    data::Vocab::load(
        VOCAB_FILE,
        LABELS_FILE,
        LABEL_MAP_FILE)
}


