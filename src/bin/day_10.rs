use std::cmp;
use std::collections::HashMap;

mod utils;

fn main() {
    let data = utils::load_input("./data/day_10.txt").unwrap();

    let mut nbs: Vec<u32> = data.lines().map(|nb| nb.parse().unwrap()).collect();

    nbs.push(0);
    nbs.sort_unstable();
    nbs.push(nbs.last().unwrap() + 3);

    let mut diff: (u32, u32) = (0, 0);
    let mut nb_prev: &u32 = &0;

    for nb in nbs.iter() {
        if nb_prev + 1 == *nb {
            diff.0 += 1;
        }
        if nb_prev + 3 == *nb {
            diff.1 += 1;
        }
        nb_prev = nb;
    }

    println!("Answer 1/2: {}", diff.0 * diff.1);
    println!(
        "Answer 2/2: {}",
        count_combinations(0, &mut HashMap::new(), &nbs)
    );
}

fn count_combinations(i: usize, memo: &mut HashMap<usize, usize>, nbs: &Vec<u32>) -> usize {
    if i == nbs.len() - 1 {
        return 1; // 1 path has been found
    }

    if let Some(&count) = memo.get(&i) {
        return count;
    }

    let mut count: usize = 0;

    let i_next = i + 1;
    let i_next_end = cmp::min(nbs.len(), i + 4);

    // explore the three path
    for (j, nb) in nbs[i_next..i_next_end].iter().enumerate() {
        if nb - nbs[i] <= 3 {
            count += count_combinations(i_next + j, memo, nbs);
        }
    }

    memo.insert(i, count);

    count
}
