use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    day_1();
}

fn day_1() {
    let path = Path::new("./data/day_1.txt");

    let mut file = File::open(&path)
        .expect("Error opening file");

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error reading file");

    let mut data: Vec<u32> = data.lines()
        .map(|datum| datum.trim().parse::<u32>())
        .filter_map(Result::ok)
        .collect();

    let mut answer_not_found = true;

    while answer_not_found {
        let current_datum = data.pop().expect("No more data");
        for datum in &data {
            if current_datum + datum == 2020 {
                println!("answer found : {} + {} = 2020", current_datum, datum);
                let result = current_datum * datum;
                println!("result : {}", result);
                answer_not_found = false;
            }
        }
    }
}
