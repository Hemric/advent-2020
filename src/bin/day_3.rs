mod utils;

fn main() {
    let data = utils::load_input("./data/day_3.txt");
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
