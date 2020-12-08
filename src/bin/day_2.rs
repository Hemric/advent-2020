use regex::Regex;

mod utils;

fn main() {
    let data = utils::load_input("./data/day_2.txt");
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
