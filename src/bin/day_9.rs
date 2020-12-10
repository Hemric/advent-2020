use std::cmp;
use std::cmp::Ordering;

mod utils;

fn main() {
    let data = utils::load_input("./data/day_9.txt").unwrap();

    let nbs: Vec<u64> = data.lines().map(|nb| nb.parse().unwrap()).collect();

    let invalid_nb: u64 = find_invalid_nb(&nbs).unwrap();
    let weakness: u64 = find_weakness(&nbs, invalid_nb).unwrap();

    println!("Answer 1/2: {}", invalid_nb);
    println!("Answer 2/2: {}", weakness);
}

fn find_weakness(nbs: &[u64], invalid_nb: u64) -> Option<u64> {
    let nbs_len = nbs.len();
    let mut w_start: usize = 0;
    let mut w_end: usize = 2;

    while w_end < nbs_len {
        let w: &[u64] = &nbs[w_start..w_end];
        let w_sum: u64 = w.iter().sum();

        match w_sum.cmp(&invalid_nb) {
            Ordering::Greater => {
                w_start += 1;
                w_end = w_start + 2;
            }
            Ordering::Less => {
                w_end += 1;
            }
            Ordering::Equal => return Some(w.iter().min().unwrap() + w.iter().max().unwrap()),
        }
    }

    None
}

fn find_invalid_nb(nbs: &[u64]) -> Option<u64> {
    let pre_len = 25;
    let nbs_len = nbs.len();

    for (i, nb) in nbs.iter().skip(pre_len).enumerate() {
        let pre_end = cmp::min(nbs_len, i + pre_len);
        let pre = &nbs[i..pre_end];

        if is_valid(&nb, &pre) {
            continue;
        }

        return Some(*nb);
    }

    None
}

fn is_valid(nb: &u64, pre: &[u64]) -> bool {
    for n in pre {
        for b in pre {
            if (n + b) == *nb {
                return true;
            }
        }
    }

    return false;
}
