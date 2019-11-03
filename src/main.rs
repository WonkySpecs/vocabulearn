extern crate chrono;

use std::error::Error;
use std::io;
use std::collections::HashMap;
use std::process;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use csv::DeserializeRecordsIter;

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
    label_mapping: Vec<ItemLabel>
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
            label_mapping
        }
    }
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

fn read_vocab_file() -> Result<Vec<VocabItem>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(VOCAB_FILE)?;
    Ok(rdr.deserialize()
        .map(Result::unwrap)
        .collect())
}

fn read_labels_file() -> Result<Vec<Label>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(LABELS_FILE)?;
    Ok(rdr.deserialize()
        .map(Result::unwrap)
        .collect())
}

fn read_vocab_labels_file() -> Result<Vec<ItemLabel>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(LABEL_MAP_FILE)?;
    Ok(rdr.deserialize()
        .map(Result::unwrap)
        .collect())
}

fn to_id_map<T: HasId>(items: Vec<T>) -> HashMap<usize, T> {
    items.into_iter()
        .map(|i| (i.id(), i))
        .collect()
}

fn main() {
    let vocab = Vocab::load(VOCAB_FILE, LABELS_FILE, LABEL_MAP_FILE);
    println!("{:?}\n{:?}\n{:?}\n", vocab.vocab, vocab.labels, vocab.label_mapping);
}
