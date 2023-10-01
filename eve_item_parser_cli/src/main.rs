use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let data = line.unwrap();
        if data.is_empty() {
            return;
        }

        let items = match eve_item_parser::parse(&data) {
            Ok(items) => items,
            Err(e) => {
                println!("Failed to parse: {e}");
                std::process::exit(1);
            }
        };
        items.into_iter().for_each(|item| println!("{item}"));
    }
}
