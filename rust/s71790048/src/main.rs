use std::error::Error;
use std::str::FromStr;
use std::io;
use std::io::Read;
use std::process;
// use serde::{Deserialize, Deserializer, Serialize};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct A {
    value: i8,
}
#[derive(Deserialize, Debug)]
struct B {
    value: String,
}
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum C {
    One(A),
    Two(B),
}

fn main() -> io::Result<()> {
    // let mut buffer = String::new();
    // io::stdin().read_to_string(&mut buffer)?;
    // println!("{:?}", buffer);
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // for result in rdr.records() {
    //     // The iterator yields Result<StringRecord, Error>, so we check the
    //     // error here.
    //     // let record = result?;
    //     // println!("{:?}", record);
    //     let record: Result<C, csv::Error> = result;
    //     match record {
    //         Ok(_) => {
    //             println!("ok");
    //         }
    //         Err(error) => {
    //             println!("Error parsing line: {}", error);
    //         }
    //     }
    // }
    for result in rdr.deserialize() {
        let record: Result<C, csv::Error> = result;
        println!("{:?}", record);
        match record {
            Ok(_) => {
                println!("ok");
            }
            Err(error) => {
                println!("Error parsing line: {}", error);
            }
        }
    }
    Ok(())
}
