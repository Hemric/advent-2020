mod utils;

fn main() {
    let data = utils::load_input("./data/day_1.txt").unwrap();

    let mut data: Vec<u32> = data
        .lines()
        .map(|l| l.trim().parse::<u32>().unwrap())
        .collect();

    let mut answer_not_found = true;
    let mut answer_1 = 0;
    let mut answer_2 = 0;

    while answer_not_found {
        let d1 = data.pop().expect("No more data");
        for d2 in &data {
            if d1 + d2 == 2020 {
                answer_1 = d1 * d2;
                answer_not_found = false;
            }
            for d3 in &data {
                if d1 + d2 + d3 == 2020 {
                    answer_2 = d1 * d2 * d3;
                }
            }
        }
    }

    println!("Answer 1/2: {}", answer_1);
    println!("Answer 2/2: {}", answer_2);
}
