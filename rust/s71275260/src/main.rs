use rayon::prelude::*;

fn main() {
    let urls = vec!["https://example.com"];
    urls.par_iter().for_each(|url: &&str| {
        println!("Hello... {:?}", *url);
        let resp = reqwest::blocking::get(*url).unwrap().text();
        println!("{:#?}", resp);
    });
}
