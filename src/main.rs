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
const LABEL_MAP_FILE: &str = "label_map.csv";

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

fn read_vocab_file() -> Result<Vec<VocabItem>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(VOCAB_FILE)?;
    Ok(rdr.deserialize()
        .map(Result::unwrap)
        .collect())
}

fn parse_input_vocab(items: Vec<VocabItem>) -> HashMap<usize, VocabItem> {
    items.into_iter()
        .map(|i| (i.id, i))
        .collect()
}

fn read_labels_file() -> Result<Vec<Label>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(LABELS_FILE)?;
    Ok(rdr.deserialize()
        .map(Result::unwrap)
        .collect())
}

fn parse_input_labels(items: Vec<Label>) -> HashMap<usize, Label> {
    items.into_iter()
        .map(|i| (i.id, i))
        .collect()
}

fn read_vocab_labels_file() -> Result<Vec<ItemLabel>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(LABEL_MAP_FILE)?;
    Ok(rdr.deserialize()
        .map(Result::unwrap)
        .collect())
}

fn parse_label_to_item_map(items: Vec<ItemLabel>) -> HashMap<usize, usize> {
    items.into_iter()
        .map(|i| (i.label_id, i.item_id))
        .collect()
}

fn load_data() -> (HashMap<usize, VocabItem>, HashMap<usize, Label>, HashMap<usize, usize>) {
    let input = read_vocab_file();
    let vocab = match input {
        Err(e) => {
            println!("Error loading vocab: {}", e);
            process::exit(1);
        }
        Ok(i) => parse_input_vocab(i)
    };
    println!("{:?}", vocab);

    let input = read_labels_file();
    let labels = match input {
        Err(e) => {
            println!("Error reading labels file: {:?}", e);
            // process::exit(1);
            HashMap::new()
        }
        Ok(i) => parse_input_labels(i)
    };

    let input = read_vocab_labels_file();
    let vocab_labels = match input {
        Err(e) => {
            println!("Error reading labels file: {:?}", e);
            // process::exit(1);
            HashMap::new()
        }
        Ok(i) => parse_label_to_item_map(i)
    };
    (vocab, labels, vocab_labels)
}

fn main() {
    let (vocab,
        labels,
        vocab_labels) = load_data();
    println!("{:?}, {:?}, {:?}", vocab, labels, vocab_labels);
}
