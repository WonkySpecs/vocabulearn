extern crate chrono;
extern crate rand;

use std::error::Error;
use std::io;
use std::io::*;
use std::collections::HashMap;
use std::process;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use rand::thread_rng;
use rand::seq::{SliceRandom, IteratorRandom};

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

fn quiz(quiz_items: Vec<&VocabItem>) -> QuizResult {
    let mut correct = 0;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to get user input");
    println!("{}", input);
    QuizResult {
        correct: 0,
        wrong: 0,
    }
}

fn main() {
    let vocab = Vocab::load(
        VOCAB_FILE,
        LABELS_FILE,
        LABEL_MAP_FILE);
    const NUM_QUESTIONS: usize = 15;
    let result = quiz(vocab.random_items(NUM_QUESTIONS));
    println!("{:?}", result);
}
