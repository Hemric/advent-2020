use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    println!("Which day ?");

    let mut day = String::new();

    io::stdin()
        .read_line(&mut day)
        .expect("Error reading line");

    let day: u32 = day.trim().parse()
        .expect("Please type a number");

    match day {
        1 => day_1(),
        _ => println!("Challenge not found"),
    }
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
                println!("Answer found with: {} + {} = 2020", current_datum, datum);
                let result = current_datum * datum;
                println!("Answer : {}", result);
                answer_not_found = false;
            }
        }
    }
}
