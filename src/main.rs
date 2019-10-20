extern crate chrono;

use std::error::Error;
use std::io;
use chrono::{ DateTime, Utc };
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct VocabItem {
    native: String,
    transliterated: Option<String>,
    original_language: Option<String>,
    time_added: DateTime<Utc>,
}

fn read_vocab() -> Result<(), Box<dyn Error>> { 
    let mut rdr = csv::Reader::from_path("test.csv")?;
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
