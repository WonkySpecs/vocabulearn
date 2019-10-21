extern crate chrono;

use std::error::Error;
use std::io;
use std::collections::HashMap;
use std::process;

use chrono::{DateTime, Utc};
use serde::Deserialize;

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

fn main() {
    let input = read_vocab_file();
    let vocab = match input {
        Err(e) => {
            println!("Error loading vocab: {}", e);
            process::exit(1);
        }
        Ok(i) => parse_input_vocab(i)
    };
    println!("{:?}", vocab);
}
