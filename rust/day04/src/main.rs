use std::{collections::HashSet, fs};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Item {
    line: String,
}

fn main() {
    let content = fs::read_to_string("logs.json").expect("can't read file");
    let items: Vec<Item> = serde_json::from_str(&content).expect("can't parse JSON");
    let unique: HashSet<String> = items.into_iter().map(|i| i.line).collect();
    let result: String = unique.iter().map(|s| format!("{}\n", s)).collect();

    fs::write("result.txt", result).expect("can't write a result");

    println!("{:#?}", unique);
}
