use std::collections::HashMap;

mod utils;

fn main() {
    let data = utils::load_input("./data/day_6.txt").unwrap();
    let groups: Vec<&str> = data.split("\n\n").collect();

    let mut sum_1: usize = 0;
    let mut sum_2: usize = 0;

    for group in groups {
        let mut map = HashMap::new();
        for (_i, c) in group.chars().filter(|c| c.to_string() != "\n").enumerate() {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let member_count = group.lines().count();

        sum_1 += map.iter().count();
        sum_2 += map.iter().fold(0, |mut acc, c| {
            if c.1 == &member_count {
                acc += 1;
            }
            acc
        })
    }

    println!("Answer 1/2: {}", sum_1);
    println!("Answer 2/2: {}", sum_2);
}
