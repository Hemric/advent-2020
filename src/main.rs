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
        3 => day_3(),
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
        .map(|datum| datum.trim().parse())
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
        let min = cap["min"].parse().unwrap();
        let max = cap["max"].parse().unwrap();

        if letter_occurence >= min && letter_occurence <= max {
            valid_password_count += 1;
        }
    }

    println!("Answer : {}", valid_password_count);
}

fn day_3() {
    let data = load_data("./data/day_3.txt");
    let lines: Vec<&str> = data.split("\n").collect();

    let mut tree_count = 0;
    let mut line_index = 1; // first line is ignored
    let mut index = 3;

    loop {
        let line = lines[line_index];

        let spot = match line.chars().nth(index) {
            Some(x) => x,
            None => {
                let line_length = line.len();
                if line_length == 0 {
                    break;
                }
                index = index - line_length;
                continue;
            }
        };

        if spot == '#' {
            tree_count += 1;
        }

        index += 3;
        line_index += 1;
    }

    println!("Answer : {}", tree_count);
}
