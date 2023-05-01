use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;
use serde_json::{Value};
use polars::prelude::*;

#[derive(Deserialize, Debug)]
struct ResultSet {
    name: String,
    headers: Vec<String>,
    row_set: Value,
}

#[derive(Deserialize, Debug)]
struct Parameters {
    game_id: String,
}

#[derive(Deserialize, Debug)]
struct Sample {
    resource: String,
    parameters: Parameters,
    result_sets: Vec<ResultSet>,
}

// fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Sample, Box<dyn Error>> {
//     // Open the file in read-only mode with buffer.
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);
//
//     // Read the JSON contents of the file as an instance of `Sample`.
//     let u = serde_json::from_reader(reader)?;
//     println!("{:?}", u);
//
//     // Return the `Sample`.
//     Ok(u)
// }

fn main() {
    // let path = "sample.json";
    // let file = File::open(path).unwrap();
    // let reader = BufReader::new(file);
    // //let u = read_user_from_file(path).unwrap();
    // let u = serde_json::from_reader::<_, Value>(reader).unwrap();
    // println!("{:#?}", u);

    let df = df![
        "a" => [1, 2],
        "b" => [3, 4]
    ];

    println!("{:#?}", df.unwrap());
}
