extern crate chrono;
extern crate rand;

use std::io::Result;
use std::collections::HashMap;
use std::process;
use std::fs;

use rand::seq::IteratorRandom;
use rand::thread_rng;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct VocabItem {
    id: usize,
    pub in_native_lang: String,
    pub transliterated: Option<String>,
    in_original_lang: Option<String>,
    time_added: DateTime<Utc>,
}


#[derive(Deserialize, Serialize, Debug)]
enum LabelType {
    WordType,
    WordArea,
    Group,
}

#[derive(Deserialize, Serialize, Debug)]
struct Label {
    id: usize,
    display_name: String,
    label_type: LabelType,
}

#[derive(Deserialize, Serialize, Debug)]
struct ItemLabel {
    item_id: usize,
    label_id: usize,
}

pub struct Vocab {
    vocab: HashMap<usize, VocabItem>,
    labels: HashMap<usize, Label>,
    label_mapping: Vec<ItemLabel>,
    vocab_file: String,
    labels_file: String,
    map_file: String,
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
            vocab_file: vocab_filename.to_string(),
            labels_file: labels_filename.to_string(),
            map_file: mapping_filename.to_string(),
        }
    }

    pub fn random_items(&self, n: usize) -> Vec<&VocabItem> {
        let mut rng = thread_rng();
        self.vocab.values().choose_multiple(&mut rng, n)
    }

    pub fn add_item(&mut self, native: &str, transliterated: &str) {
        let max_id = self.vocab.keys().max().unwrap();
        let new_item = VocabItem {
            id: max_id + 1,
            in_native_lang: native.to_string(),
            transliterated: Some(transliterated.to_string()),
            in_original_lang: Option::None,
            time_added: Utc::now(),
        };
        self.vocab.insert(new_item.id, new_item);
        self.save_vocab();
    }

    fn save_vocab(&self) -> std::result::Result<(), Box<std::error::Error>> {
        let new_file = "new_".to_owned() + &self.vocab_file;
        let archive_file = "old_".to_owned() + &self.vocab_file;
        let mut wtr = csv::Writer::from_path(&new_file)?;
        for item in self.vocab.values() {
            wtr.serialize(item)?;
        }
        wtr.flush()?;

        fs::rename(&self.vocab_file, archive_file);
        fs::rename(new_file, &self.vocab_file);
        Ok(())
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
