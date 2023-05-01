use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "topic")]
pub enum Catalog {
    CarEntry(Car),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    pub name: String,
    pub HP: i32
}

fn main() {
    let car = Car { name: String::from("Honda"), HP: 200 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&car).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Car = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

}
