extern crate chrono;
extern crate rand;

use std::io::Result;
use std::collections::HashMap;
use std::process;

use rand::seq::IteratorRandom;
use rand::thread_rng;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VocabItem {
    id: usize,
    pub in_native_lang: String,
    pub transliterated: Option<String>,
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

pub struct Vocab {
    vocab: HashMap<usize, VocabItem>,
    labels: HashMap<usize, Label>,
    label_mapping: Vec<ItemLabel>,
}

impl Vocab {
    pub fn load(vocab_filename: &str, labels_filename: &str, mapping_filename: &str) -> Vocab {
        let input = deserialize_csv_file(vocab_filename);
        let vocab = match input {
            Err(e) => {
                println!("Error loading vocab: {}", e);
                process::exit(1);
            }
            Ok(i) => to_id_map(i)
        };

        let input = deserialize_csv_file(labels_filename);
        let labels = match input {
            Err(e) => {
                println!("Error reading labels file: {:?}", e);
                process::exit(1);
            }
            Ok(i) => to_id_map(i)
        };

        let input = deserialize_csv_file(mapping_filename);
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

    pub fn random_items(&self, n: usize) -> Vec<&VocabItem> {
        let mut rng = thread_rng();
        self.vocab.values().choose_multiple(&mut rng, n)
    }
}

#[derive(Debug)]
pub struct QuizResult {
    pub correct: usize,
    pub wrong: usize,
}

pub enum QuestionType {
    NativeToForeign,
    ForeignToNative,
    Bidirectional,
}

pub enum AnswerMatch {
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

fn deserialize_csv_file<T> (filename: &str) -> Result<Vec<T>> where
    for<'de> T: Deserialize<'de> {
    let mut rdr = csv::Reader::from_path(filename)?;
    Ok(rdr.deserialize()
        .map(|r| r.unwrap())
        .collect())
}

fn to_id_map<T: HasId>(items: Vec<T>) -> HashMap<usize, T> {
    items.into_iter()
        .map(|i| (i.id(), i))
        .collect()
}
