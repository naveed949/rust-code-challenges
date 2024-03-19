use std::{error::Error, io, process};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<String>,
}

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b':')
        .has_headers(false)
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}