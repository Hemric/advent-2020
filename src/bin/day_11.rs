mod utils;

const COL_LEN: usize = 97;
const ROW_LEN: usize = 93;

fn main() {
    let data = utils::load_input("./data/day_11.txt").unwrap();

    let seats: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let seats_1 = fill_seats(&seats, count_occupied_adjacent_seats, 4);
    let seats_2 = fill_seats(&seats, count_occupied_directions, 5);

    println!("Answer 1/2: {}", count_all_occupied(seats_1));
    println!("Answer 2/2: {}", count_all_occupied(seats_2));
}

fn count_all_occupied(seats: Vec<Vec<char>>) -> usize {
    seats.iter().flatten().filter(|&s| *s == '#').count()
}

fn fill_seats(
    seats: &[Vec<char>],
    count_method: fn((usize, usize), &[Vec<char>]) -> u8, // adjacent || direction
    tolerance: u8,
) -> Vec<Vec<char>> {
    let mut unstable = true;
    let mut seats = seats.to_vec();
    let mut seats_next_round = seats.to_vec();

    loop {
        if unstable == false {
            break;
        }

        unstable = false;

        for (row, seats_row) in seats.iter().enumerate() {
            for (col, seat) in seats_row.iter().enumerate() {
                match seat {
                    'L' => {
                        if count_method((row, col), &seats) == 0 {
                            seats_next_round[row][col] = '#';
                            unstable = true;
                        }
                    }
                    '#' => {
                        if count_method((row, col), &seats) >= tolerance {
                            seats_next_round[row][col] = 'L';
                            unstable = true;
                        }
                    }
                    _ => (),
                }
            }
        }

        seats = seats_next_round.to_vec();
    }

    seats
}

fn count_occupied_adjacent_seats(coord: (usize, usize), seats: &[Vec<char>]) -> u8 {
    let r_len = ROW_LEN as isize;
    let c_len = COL_LEN as isize;
    let (r, c) = (coord.0 as isize, coord.1 as isize);

    [
        (r - 1, c - 1),
        (r - 1, c),
        (r - 1, c + 1),
        (r, c - 1),
        (r, c + 1),
        (r + 1, c - 1),
        (r + 1, c),
        (r + 1, c + 1),
    ]
    .iter()
    .filter(|(r, c)| (*r >= 0 && *c >= 0) && (*r < r_len && *c < c_len))
    .fold(0, |mut acc, (r, c)| {
        if seats[*r as usize][*c as usize] == '#' {
            acc += 1
        }

        acc
    })
}

fn count_occupied_directions(coord: (usize, usize), seats: &[Vec<char>]) -> u8 {
    let (r, c) = (coord.0 as isize, coord.1 as isize);

    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .fold(0, |mut acc, dir| {
        if find_occupied_seats_in_direction((r, c), *dir, seats) {
            acc += 1
        }

        acc
    })
}

fn find_occupied_seats_in_direction(
    i: (isize, isize),   // (row, col)
    dir: (isize, isize), // direction modificator (row, col)
    seats: &[Vec<char>],
) -> bool {
    let r_len = ROW_LEN as isize;
    let c_len = COL_LEN as isize;

    let (r, c) = (i.0 + dir.0, i.1 + dir.1);

    if r < 0 || r >= r_len || c < 0 || c >= c_len {
        return false;
    }

    let current_seat = seats[r as usize][c as usize];

    match current_seat {
        'L' => return false,
        '#' => return true,
        _ => (),
    }

    find_occupied_seats_in_direction((r, c), dir, seats)
}
