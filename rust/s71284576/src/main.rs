extern crate mongodb;
use mongodb::doc;
// use mongodb::{bson, doc};
// use mongodb::coll::options::IndexOptions;

// async fn create_id_fields(client: &Client) {
//     let options = IndexOptions::builder().unique(true).build();
//     let model = IndexModel::builder()
//         .keys(doc! {"number": 1})
//         .options(options)
//         .build();
//     client
//         .database(DB_NAME)
//         .collection::<Raffle>(COLL_NAME_RAFFLE)
//         .create_index(model, None)
//         .await
//         .expect("error creating index!");
// }

fn main() {
    println!("Hello, world!");
    let options = IndexOptions::new();
    options.unique = true;
    println!("{:?}", options);
    println!("{:?}", IndexModel::new(doc! {"number": 1}, options));
}
