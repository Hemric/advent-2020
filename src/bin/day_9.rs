use std::cmp;

mod utils;

fn main() {
    let data = utils::load_input("./data/day_9.txt").unwrap();

    let nbs: Vec<u64> = data.lines().map(|nb| nb.parse().unwrap()).collect();

    let pre_len = 25;
    let nbs_len = nbs.len();
    let mut nb_invalid = None;

    for (i, nb) in nbs.iter().skip(pre_len).enumerate() {
        let pre_end = cmp::min(nbs_len, i + pre_len);
        let pre = &nbs[i..pre_end];

        if is_valid(&nb, &pre) {
            continue;
        } else {
            nb_invalid = Some(nb);
            break;
        }
    }

    println!("Answer 1/2: {}", nb_invalid.unwrap());
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
