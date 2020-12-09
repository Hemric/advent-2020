mod utils;

fn main() {
    let data = utils::load_input("./data/day_8.txt").unwrap();

    let instructions: Vec<(&str, i32)> = data
        .lines()
        .map(|l| {
            let i: Vec<&str> = l.split(" ").collect();
            (i[0], i[1].parse::<i32>().unwrap())
        })
        .collect();

    println!("Answer 1/2 {}", answer_1(&instructions));
    println!("Answer 2/2 {}", answer_2(&instructions));
}

fn answer_2(instructions: &Vec<(&str, i32)>) -> i32 {
    let mut acc: i32 = 0;
    let mut cursor: usize = 0;
    let len = instructions.len();
    let mut recorder = vec![false; len];
    let mut switch_recorder = recorder.clone();
    let mut switched_once = false;

    while cursor < len {
        if recorder[cursor] {
            // reset everyhting but switch_recorder
            cursor = 0;
            acc = 0;
            switched_once = false;
            recorder = vec![false; len];
        }

        recorder[cursor] = true;

        let current_instruction = instructions[cursor];

        match current_instruction.0 {
            "acc" => {
                acc += current_instruction.1;
                cursor += 1;
            }
            "jmp" => {
                if switched_once || switch_recorder[cursor] {
                    // default behavior
                    cursor = (cursor as i32 + current_instruction.1) as usize
                } else {
                    switched_once = true;
                    switch_recorder[cursor] = true;
                    // switched behavior
                    cursor += 1;
                }
            }
            "nop" => {
                if current_instruction.1 == 0 || switched_once || switch_recorder[cursor] {
                    // default behavior
                    cursor += 1;
                } else {
                    switched_once = true;
                    switch_recorder[cursor] = true;
                    // switched behavior
                    cursor = (cursor as i32 + current_instruction.1) as usize
                }
            }
            _ => (),
        }
    }

    acc
}

fn answer_1(instructions: &Vec<(&str, i32)>) -> i32 {
    let mut acc: i32 = 0;
    let mut cursor: usize = 0;
    let mut recorder = vec![false; instructions.len()];

    loop {
        if recorder[cursor] {
            break;
        }

        recorder[cursor] = true;

        let current_instruction = instructions[cursor];

        match current_instruction.0 {
            "acc" => {
                acc += current_instruction.1;
                cursor += 1;
            }
            "jmp" => cursor = (cursor as i32 + current_instruction.1) as usize,
            _ => cursor += 1,
        }
    }

    acc
}
