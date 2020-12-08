use regex::Regex;
use std::collections::HashMap;

mod utils;

fn main() {
    let data = utils::load_input("./data/day_7.txt");
    let re = Regex::new(r"\b(?P<container>[a-z]+ [a-z]+) bags \b(?:contain no|contain(?P<list>(?: \d+ [a-z]+ [a-z]+ bags?.)+))").unwrap();
    let re_list = Regex::new(r"(\d) ([a-z]+ [a-z]+)").unwrap();

    // HashMap to store all the rules
    let mut bag_rules: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    for cap in re.captures_iter(&data) {
        let list = match cap.name("list") {
            Some(list) => {
                let mut v: Vec<(String, u32)> = [].to_vec();
                for item in re_list.captures_iter(list.as_str()) {
                    v.push((item[2].to_string(), item[1].parse().unwrap()));
                }
                v
            }
            None => [].to_vec(),
        };

        bag_rules.insert(cap["container"].to_string(), list);
    }

    let mut sum: usize = 0;

    for (bag_color, _list) in bag_rules.iter() {
        if bag_contains("shiny gold", &bag_color, &bag_rules) {
            sum += 1
        }
    }

    let sum_2 = count_bags("shiny gold", &bag_rules) - 1; // minus the shiny gold bag

    println!("Answer 1/2: {}", sum);
    println!("Answer 2/2: {}", sum_2);
}

fn count_bags(bag_color: &str, bag_rules: &HashMap<String, Vec<(String, u32)>>) -> u64 {
    let bag_infos = match bag_rules.get(bag_color) {
        Some(b) => b,
        None => {
            return 1;
        }
    };

    let mut sum = 1; // current bag

    for info in bag_infos {
        sum += info.1 as u64 * count_bags(info.0.as_str(), bag_rules);
    }

    sum
}

fn bag_contains(
    needle: &str,
    bag_color: &str,
    bag_rules: &HashMap<String, Vec<(String, u32)>>,
) -> bool {
    let bag_infos: Vec<&str> = match bag_rules.get(bag_color) {
        Some(colors) => colors.iter().map(|x| x.0.as_str()).collect(),
        None => {
            return false;
        }
    };

    if bag_infos.contains(&needle) {
        return true;
    }

    for bag_color in bag_infos {
        if bag_contains(needle, bag_color, bag_rules) {
            return true;
        }
    }

    return false;
}
