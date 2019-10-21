extern crate chrono;

use std::error::Error;
use std::io;
use chrono::{ DateTime, Utc };
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
    WordType, WordArea, Group,
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

fn read_vocab() -> Result<(), Box<dyn Error>> { 
    let mut rdr = csv::Reader::from_path(VOCAB_FILE)?;
    for result in rdr.deserialize() {
        let item: VocabItem = result?;
        println!("{:?}", item);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_vocab() {
        println!("Error: {}", err);
    }
}
