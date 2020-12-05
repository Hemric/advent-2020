use regex::Regex;
use regex::RegexSet;
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
        4 => day_4(),
        5 => day_5(),
        _ => println!("Challenge not found"),
    }
}

fn load_data(path: &str) -> String {
    let path = Path::new(path);

    let mut file = File::open(path).expect("Error opening file");

    let mut data = String::new();

    file.read_to_string(&mut data).expect("Error reading file");

    data.trim_end().to_string()
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

    let configs = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

    let mut answer: u32 = 1;

    for config in configs.iter() {
        answer *= slop(&lines, config[0], config[1]);
    }

    println!("Answer : {}", answer);
}

fn slop(lines: &Vec<&str>, right: usize, down: usize) -> u32 {
    let mut tree_count = 0;
    let mut line_index = down; // first line is ignored
    let mut index = right;

    loop {
        let line = match lines.get(line_index) {
            Some(x) => x,
            None => break,
        };

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

        index += right;
        line_index += down;
    }

    println!("Right {}, Down {}, Tree {}", right, down, tree_count);

    tree_count
}

fn day_4() {
    let data = load_data("./data/day_4.txt");
    let passports: Vec<&str> = data.split("\n\n").collect();

    let set_1 = RegexSet::new(&[
        r"byr:", r"iyr:", r"eyr:", r"hgt:", r"hcl:", r"ecl:", r"pid:",
    ])
    .unwrap();

    let set_2 = RegexSet::new(&[
        r"\bbyr:(19[2-9][0-9]|200[0-2])\b",
        r"\biyr:(201[0-9]|2020)\b",
        r"\beyr:(202[0-9]|2030)\b",
        r"\bhgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)\b",
        r"\bhcl:#([a-f]|[0-9]){6}\b",
        r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b",
        r"\bpid:[0-9]{9}\b",
    ])
    .unwrap();

    let mut valid_passport_count_1 = 0;
    let mut valid_passport_count_2 = 0;

    for passport in passports.iter() {
        let matches: Vec<_> = set_1.matches(passport).iter().collect();

        if matches.len() == set_1.len() {
            valid_passport_count_1 += 1;
        }

        let matches: Vec<_> = set_2.matches(passport).iter().collect();

        if matches.len() == set_2.len() {
            valid_passport_count_2 += 1;
        }
    }

    println!("Answer 1/2: {}", valid_passport_count_1);
    println!("Answer 2/2: {}", valid_passport_count_2);
}

fn day_5() {
    let data = load_data("./data/day_5.txt");
    let tickets: Vec<&str> = data.split("\n").collect();
    let mut seats: Vec<u32> = [].to_vec();

    for ticket in tickets {
        let row = search((0.0, 127.0), ticket[..7].chars().rev().collect()) as u32;
        let column = search((0.0, 7.0), ticket[7..].chars().rev().collect()) as u32;
        seats.push(row * 8 + column);
    }

    seats.sort();

    let mut my_seat: u32 = 0;

    for (i, seat) in seats.iter().enumerate() {
        if seats[i+1] != seat + 1 {
            my_seat = seat + 1;
            break;
        }
    }

    println!("Answer 1/2: {}", seats.last().unwrap());
    println!("Answer 2/2: {}", my_seat);
}

fn search(mut range: (f32, f32), mut letters: Vec<char>) -> f32 {
    let letter = match letters.pop() {
        Some(l) => l.to_string(),
        None => return range.0,
    };

    let total_range = range.0 + range.1;

    if letter == "F" || letter == "L" {
        range.1 = (total_range / 2.0).floor();
    }

    if letter == "B" || letter == "R" {
        range.0 = (total_range / 2.0).ceil();
    }

    search(range, letters)
}
