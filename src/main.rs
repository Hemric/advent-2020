use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Which day ?");

    let mut day = String::new();

    io::stdin().read_line(&mut day).expect("Error reading line");

    let day: u32 = day.trim().parse().expect("Please type a number");

    match day {
        1 => day_1(),
        2 => day_2(),
        _ => println!("Challenge not found"),
    }
}

fn load_data(path: &str) -> String {
    let path = Path::new(path);

    let mut file = File::open(path).expect("Error opening file");

    let mut data = String::new();

    file.read_to_string(&mut data).expect("Error reading file");

    data
}

fn day_1() {
    let data = load_data("./data/day_1.txt");

    let mut data: Vec<u32> = data
        .lines()
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

fn day_2() {
    let data = load_data("./data/day_2.txt");
    let re =
        Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<password>[a-z]*)").unwrap();
    let mut valid_password_count = 0;

    for cap in re.captures_iter(&data) {
        let letter_occurence = cap["password"].matches(&cap["letter"]).count();
        let min: usize = cap["min"].parse().unwrap();
        let max: usize = cap["max"].parse().unwrap();

        if letter_occurence >= min && letter_occurence <= max {
            valid_password_count += 1;
        }
    }

    println!("Answer : {}", valid_password_count);
}
