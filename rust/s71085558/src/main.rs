use mongodb::bson::oid::ObjectId;
use schemars::JsonSchema;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::gen::SchemaGenerator;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectID(ObjectId);

impl JsonSchema for ObjectID {
    fn schema_name() -> String {
        stringify!(String).to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema: SchemaObject = <String>::json_schema(gen).into();
        schema.number().minimum = Some(1.0);
        schema.into()
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Customer {
    pub _id: ObjectID,
    pub name: String
}

fn main() {
    let c = Customer{_id: ObjectID(ObjectId::new()), name: String::from("John")};
    println!("{:?}", c);
    let serialized = serde_json::to_string(&c).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Customer = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
