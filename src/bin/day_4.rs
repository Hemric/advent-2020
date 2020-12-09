use regex::RegexSet;

mod utils;

fn main() {
    let data = utils::load_input("./data/day_4.txt").unwrap();
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
