extern crate chrono;
extern crate rand;
extern crate clap;

use std::error::Error;
use std::io;
use std::io::*;
use std::collections::HashMap;
use std::process;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use rand::thread_rng;
use rand::seq::IteratorRandom;
use clap::{Arg, App, SubCommand};

const VOCAB_FILE: &str = "vocab.csv";
const LABELS_FILE: &str = "labels.csv";
const LABEL_MAP_FILE: &str = "label_mapping.csv";

#[derive(Deserialize, Debug)]
struct VocabItem {
    id: usize,
    in_native_lang: String,
    transliterated: Option<String>,
    in_original_lang: Option<String>,
    time_added: DateTime<Utc>,
}


#[derive(Deserialize, Debug)]
enum LabelType {
    WordType,
    WordArea,
    Group,
}

#[derive(Deserialize, Debug)]
struct Label {
    id: usize,
    display_name: String,
    label_type: LabelType,
}

#[derive(Deserialize, Debug)]
struct ItemLabel {
    item_id: usize,
    label_id: usize,
}

struct Vocab {
    vocab: HashMap<usize, VocabItem>,
    labels: HashMap<usize, Label>,
    label_mapping: Vec<ItemLabel>,
}

impl Vocab {
    fn load(vocab_filename: &str, labels_filename: &str, mapping_filename: &str) -> Vocab {
        let input = read_vocab_file();
        let vocab = match input {
            Err(e) => {
                println!("Error loading vocab: {}", e);
                process::exit(1);
            }
            Ok(i) => to_id_map(i)
        };

        let input = read_labels_file();
        let labels = match input {
            Err(e) => {
                println!("Error reading labels file: {:?}", e);
                process::exit(1);
            }
            Ok(i) => to_id_map(i)
        };

        let input = read_vocab_labels_file();
        let label_mapping = match input {
            Err(e) => {
                println!("Error reading labels file: {:?}", e);
                process::exit(1);
            }
            Ok(i) => i
        };
        Vocab {
            vocab,
            labels,
            label_mapping,
        }
    }

    fn random_items(&self, n: usize) -> Vec<&VocabItem> {
        let mut rng = thread_rng();
        self.vocab.values().choose_multiple(&mut rng, n)
    }
}

#[derive(Debug)]
struct QuizResult {
    correct: usize,
    wrong: usize,
}

enum QuestionType {
    NativeToForeign,
    ForeignToNative,
    Bidirectional,
}

enum AnswerMatch {
    Perfect,
    Partial,
    Wrong,
}

trait HasId {
    fn id(&self) -> usize;
}

impl HasId for VocabItem {
    fn id(&self) -> usize {
        self.id
    }
}

impl HasId for Label {
    fn id(&self) -> usize {
        self.id
    }
}

fn read_vocab_file() -> Result<Vec<VocabItem>> {
    let mut rdr = csv::Reader::from_path(VOCAB_FILE)?;
    Ok(rdr.deserialize()
        .map(|r| r.unwrap())
        .collect())
}

fn read_labels_file() -> Result<Vec<Label>> {
    let mut rdr = csv::Reader::from_path(LABELS_FILE)?;
    Ok(rdr.deserialize()
        .map(|r| r.unwrap())
        .collect())
}

fn read_vocab_labels_file() -> Result<Vec<ItemLabel>> {
    let mut rdr = csv::Reader::from_path(LABEL_MAP_FILE)?;
    Ok(rdr.deserialize()
        .map(|r| r.unwrap())
        .collect())
}

fn to_id_map<T: HasId>(items: Vec<T>) -> HashMap<usize, T> {
    items.into_iter()
        .map(|i| (i.id(), i))
        .collect()
}

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
    let vocab = Vocab::load(
        VOCAB_FILE,
        LABELS_FILE,
        LABEL_MAP_FILE);
    const NUM_QUESTIONS: usize = 15;
    let result = quiz(vocab.random_items(NUM_QUESTIONS),
                      quiz_type);
    println!("{:?}", result);
}

fn main() {
    let matches = App::new("Vocabulearn")
        .version("0.1")
        .author("Will Taylor")
        .subcommand(SubCommand::with_name("quiz")
            .about("Run a quiz")
            .arg(Arg::with_name("type")
                .long("type")
                .short("t")
                .possible_values(&["ntf", "ftn", "both"])
                .default_value("both")
                .takes_value(true)))
        .get_matches();

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
        _ => unreachable!()
    };
}
