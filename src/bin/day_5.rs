mod utils;

fn main() {
    let data = utils::load_input("./data/day_5.txt").unwrap();
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
        if seats[i + 1] != seat + 1 {
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
