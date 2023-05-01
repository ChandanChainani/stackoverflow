use std::io::{stdout, Write};
use curl::easy::{Easy,List};

use std::collections::HashMap;
use serde_bytes::Bytes;
/*
data = {
    "key": client_secret,
    "image": base64.b64encode(file),
    "type": "base64",
}
*/

fn main() {
    let mut list = List::new();
    list.append("Authorization  Client-ID MYCLIENTID").unwrap();
    let mut easy = Easy::new();
    easy.url("https://api.imgur.com/3/upload.json").unwrap();
    easy.http_headers(list).unwrap();

    let mut cache = HashMap::new();
    cache.insert("key", Bytes::new(b"secret"));
    cache.insert("image", Bytes::new(b"file"));
    cache.insert("type", Bytes::new(b"base64"));

    easy.read_function(|into| {
        bincode::serialize_into(&mut into, &cache)
    }).unwrap();

    easy.post(true).unwrap();

    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}
